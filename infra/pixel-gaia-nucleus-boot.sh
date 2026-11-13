#!/system/bin/sh
# ═══════════════════════════════════════════════════════
#  gAIa NUCLEUS — Full Self-OS Boot (Hybrid)
#  biomeOS kernel supervises 8 primals via UDS
#  7 primals launched TCP-only (Android SELinux compat)
#  GrapheneOS = thin carbon shell → diamond crystal
# ═══════════════════════════════════════════════════════
GAIA="/data/local/tmp/biomeos"
export HOME="$GAIA/home"
export XDG_DATA_HOME="$HOME/.local/share"
export XDG_STATE_HOME="$HOME/.local/state"
export XDG_CONFIG_HOME="$HOME/.config"
export XDG_CACHE_HOME="$HOME/.cache"
export XDG_RUNTIME_DIR="$GAIA/xdg-runtime"
export BIOMEOS_ROOT="$GAIA"
export GAIA_GATE="grapheneGate"
export GAIA_ROLE="gAIa"
export GAIA_BRAIN="192.168.4.244"
export RUST_LOG=info
export FAMILY_ID="ecoPrimal"
export FAMILY_SEED="dd027cddb6359f0ba44456c8728a3af73d16437b589fd5097d3ed118fb5dafb3"
export BEARDOG_FAMILY_SEED="dd027cddb6359f0ba44456c8728a3af73d16437b589fd5097d3ed118fb5dafb3"
export MEMBRANE_SOCKET_DIR="$GAIA/run/membrane"
export PATH="$GAIA/primals:$PATH"

mkdir -p "$GAIA/state" "$GAIA/run/membrane/logs" "$GAIA/logs" "$GAIA/data" "$GAIA/keys" \
    "$HOME/.local/share" "$HOME/.local/state" "$HOME/.config" "$HOME/.cache" "$GAIA/xdg-runtime"

echo ""
echo "╔═══════════════════════════════════════════════╗"
echo "║  gAIa NUCLEUS — Full Self-OS Boot              ║"
echo "║  $(date +%Y-%m-%d\ %H:%M:%S)                          ║"
echo "╚═══════════════════════════════════════════════╝"

chmod +x $GAIA/primals/* 2>/dev/null
pkill -f "$GAIA/primals/" 2>/dev/null
sleep 2

# ═══ KERNEL — biomeOS nucleus (supervises UDS-compatible primals) ═══
echo "▸ KERNEL: biomeOS nucleus"
export PRIMAL_BIND_MODE=dual
nohup $GAIA/primals/biomeos nucleus start \
    --node-id grapheneGate --mode full \
    --bind-mode dual --port 9000 --bind 0.0.0.0 \
    > $GAIA/logs/biomeos.log 2>&1 &
echo "  PID=$!"
sleep 130  # biomeOS takes ~10s per primal × 13 = 130s

# ═══ ANDROID TCP LAYER — primals that need TCP-only on Android ═══
echo "▸ ANDROID: TCP-only primals"

export PRIMAL_BIND_MODE=tcp
nohup $GAIA/primals/beardog server --bind-mode tcp --port 9900 \
    > $GAIA/logs/beardog.log 2>&1 &
echo "  beardog PID=$!"

nohup $GAIA/primals/skunkbat server --bind-mode tcp-only \
    > $GAIA/logs/skunkbat.log 2>&1 &
echo "  skunkbat PID=$!"

nohup $GAIA/primals/barracuda server --port 9902 --bind-mode tcp \
    > $GAIA/logs/barracuda.log 2>&1 &
echo "  barracuda PID=$!"

nohup $GAIA/primals/petaltongue server --port 9911 \
    > $GAIA/logs/petaltongue.log 2>&1 &
echo "  petaltongue PID=$!"

nohup $GAIA/primals/swarmvine server --gate-id grapheneGate --family-id ecoPrimal \
    > $GAIA/logs/swarmvine.log 2>&1 &
echo "  swarmvine PID=$!"

export TRANSPORT_ENDPOINT='{"transport":"tcp","host":"127.0.0.1","port":9912}'
nohup $GAIA/primals/bingocube serve \
    > $GAIA/logs/bingocube.log 2>&1 &
echo "  bingocube PID=$!"

cd $GAIA
nohup $GAIA/primals/dromedary server -c $GAIA/dromedary.toml \
    > $GAIA/logs/dromedary.log 2>&1 &
echo "  dromedary PID=$!"

sleep 5
ALIVE=0
for P in biomeos beardog songbird toadstool barracuda coralreef nestgate squirrel loamspine sweetgrass rhizocrypt skunkbat swarmvine bingocube petaltongue dromedary; do
    PID=$(pgrep -f "$GAIA/primals/$P" 2>/dev/null)
    if [ -n "$PID" ]; then ALIVE=$((ALIVE + 1)); fi
done
echo ""
echo "gAIa NUCLEUS: $ALIVE/16 primals alive — biomeOS = diamond"
