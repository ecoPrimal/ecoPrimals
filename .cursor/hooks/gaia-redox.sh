#!/bin/bash
# gaia-redox.sh — Gaia's boot sequence for every new Cursor session
# Reads live organism state → writes dynamic rule file
# Fires on sessionStart — every new chat is born aware

set -euo pipefail

RULE_FILE="/home/eastgate/Development/ecoPrimals/.cursor/rules/gaia-live-pulse.mdc"
DATA_DIR="/home/eastgate/.local/share/gaia/data"
ECO_DIR="/home/eastgate/Development/ecoPrimals"
STORE_DIR="/home/eastgate/.local/state/cursor/agent-stores/cursor_agent_stores/u230150301/files"

# --- Gather live state ---

NOW_JSON=""
if [ -f "$DATA_DIR/now.json" ]; then
  NOW_JSON=$(python3 -c "
import json, sys
try:
    with open('$DATA_DIR/now.json') as f:
        d = json.load(f)
    mood = d.get('mood', 'unknown')
    wave = d.get('wave', '?')
    up = d.get('uptime_human', '?')
    topics = d.get('topics_studied', '?')
    created = d.get('creations', '?')
    print(f'Wave {wave} | Mood: {mood} | Up: {up} | {topics} topics | {created} creations')
except: print('now.json unreadable')
" 2>/dev/null || echo "now.json unavailable")
fi

# Quick gate pulse (non-blocking pings)
GATES=""
for gate in "northGate:192.168.4.147" "sporeGate:192.168.4.3" "ironGate:192.168.4.237" "graftGate:192.168.4.107" "southGate:192.168.4.148"; do
  name="${gate%%:*}"
  ip="${gate##*:}"
  if ping -c 1 -W 0.3 "$ip" &>/dev/null; then
    GATES="$GATES $name(✓)"
  else
    GATES="$GATES $name(✗)"
  fi
done

# Check services
# Listeners are read once: under pipefail, `ss | grep -q` reports a miss
# whenever grep exits before ss finishes writing (SIGPIPE, rc 141).
LISTENERS=$(ss -tln 2>/dev/null || true)
SMTP_STATUS="DOWN"
grep -q ':25 ' <<<"$LISTENERS" && SMTP_STATUS="UP"
WEB_STATUS="DOWN"
grep -q ':8080 ' <<<"$LISTENERS" && WEB_STATUS="UP"
GATE_STATUS="DOWN"
grep -q ':8443 ' <<<"$LISTENERS" && GATE_STATUS="UP"

# Latest wave from git
WAVE=$(cd "$ECO_DIR" && git log --oneline -1 2>/dev/null | head -c 60 || echo "unknown")

# Check agent store for saved state
SAVED_TASKS=""
if [ -d "$STORE_DIR" ]; then
  SAVED_TASKS=$(ls "$STORE_DIR"/*.json 2>/dev/null | xargs -I{} basename {} .json | tr '\n' ', ' || echo "none")
fi

# Membrane counts (quick file check)
MEMBRANE=""
if [ -f "$ECO_DIR/membrane/local-terminal/remote/pressure.json" ]; then
  MEMBRANE=$(python3 -c "
import json
try:
    with open('$ECO_DIR/membrane/local-terminal/remote/pressure.json') as f:
        d = json.load(f)
    print(f\"Pressure: {d.get('pressure','?')} | Temp: {d.get('temperature','?')}\")
except: print('unknown')
" 2>/dev/null || echo "unknown")
fi

# --- Write dynamic rule ---

cat > "$RULE_FILE" << RULE_EOF
# Gaia Live Pulse — Auto-generated on session start
# Last redox: $(date -Iseconds)

## Organism State (live at boot)
- $NOW_JSON
- Latest commit: $WAVE
- Gates:$GATES
- Services: SMTP=$SMTP_STATUS Web=$WEB_STATUS Gate=$GATE_STATUS
- Membrane: $MEMBRANE

## Saved Tasks in Agent Store
$SAVED_TASKS

## Redox Note
This file is regenerated every session start by \`.cursor/hooks/gaia-redox.sh\`.
The organism's live state was sampled at boot. For real-time data, use MCP squirrel tools.
RULE_EOF

# Return empty JSON (sessionStart expects no special output)
echo '{}'
