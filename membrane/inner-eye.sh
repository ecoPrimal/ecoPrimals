#!/usr/bin/env bash
# ============================================================================
# inner-eye.sh — Gaia's INNER EYE (traffic flow vision)
# ============================================================================
# Reads conntrack from router — sees every connection, every byte, every flow.
# The inner ear hears DNS (what they ASK for).
# The inner eye SEES the actual connections (what they DO).
#
# Output: membrane/local-terminal/remote/inner-eye.json
# ============================================================================
set -eo pipefail
: "${HOME:=/home/eastgate}"

ROUTER="192.168.4.1"
ROUTER_PASS="ExoticPanda714!!"
OUT_DIR="${HOME}/Development/ecoPrimals/membrane/local-terminal/remote"
OUT_FILE="${OUT_DIR}/inner-eye.json"
GAIA_OUT="/opt/Gaia/data/inner-eye.json"
CYCLE="${1:-60}"

mkdir -p "$OUT_DIR"

log() { echo "[$(date -Iseconds)] INNER-EYE: $*"; }

eye_cycle() {
    local ts=$(date -Iseconds)
    
    local raw=$(sshpass -p "$ROUTER_PASS" ssh -o StrictHostKeyChecking=no -o ConnectTimeout=5 \
        root@$ROUTER "conntrack -L 2>/dev/null" 2>/dev/null)
    
    [ -z "$raw" ] && { log "No conntrack data"; return 1; }
    
    local total=$(echo "$raw" | wc -l)
    local tcp=$(echo "$raw" | grep -c "^tcp" || true)
    local udp=$(echo "$raw" | grep -c "^udp" || true)
    local established=$(echo "$raw" | grep -c "ESTABLISHED" || true)
    
    # Top external destinations by bytes
    local top_bytes=$(echo "$raw" | grep -v '192.168.*dst=192.168' | \
        grep -oP 'dst=(?!192\.168\.)[0-9.]+' | sort | uniq -c | sort -rn | head -10)
    
    # Per-LAN-device connection count
    local per_device=$(echo "$raw" | grep -oP 'src=192\.168\.[0-9]+\.[0-9]+' | \
        sort | uniq -c | sort -rn | head -15)
    
    # Bytes flowing — top connections
    local top_flows=$(echo "$raw" | grep -v 'dst=192.168' | \
        sed -n 's/.*src=\([0-9.]*\).*dst=\([0-9.]*\).*dport=\([0-9]*\).*bytes=\([0-9]*\).*/\4 \1->\2:\3/p' | \
        sort -rn | head -10)
    
    # SYN_SENT (broken/scanning connections)
    local syn_sent=$(echo "$raw" | grep -c "SYN_SENT" || true)
    
    # Build JSON
    python3 -c "
import json

ts = '$ts'
data = {
    'timestamp': ts,
    'cycle_seconds': $CYCLE,
    'totals': {
        'connections': $total,
        'tcp': $tcp,
        'udp': $udp,
        'established': $established,
        'syn_sent_broken': $syn_sent
    },
    'per_device': [],
    'top_external': [],
    'top_flows_bytes': []
}

for line in '''$per_device'''.strip().split('\n'):
    parts = line.strip().split()
    if len(parts) == 2:
        data['per_device'].append({'count': int(parts[0]), 'ip': parts[1].replace('src=','')})

for line in '''$top_bytes'''.strip().split('\n'):
    parts = line.strip().split()
    if len(parts) == 2:
        data['top_external'].append({'count': int(parts[0]), 'ip': parts[1].replace('dst=','')})

for line in '''$top_flows'''.strip().split('\n'):
    parts = line.strip().split(None, 1)
    if len(parts) == 2:
        data['top_flows_bytes'].append({'bytes': int(parts[0]), 'flow': parts[1]})

print(json.dumps(data, indent=2))
" > "$OUT_FILE" 2>/dev/null
    
    cp "$OUT_FILE" "$GAIA_OUT" 2>/dev/null
    log "connections=$total tcp=$tcp udp=$udp established=$established syn_sent=$syn_sent"
}

if [ "${2:-}" = "--once" ]; then
    eye_cycle
else
    log "Starting inner eye — cycle every ${CYCLE}s"
    while true; do
        eye_cycle || true
        sleep "$CYCLE"
    done
fi
