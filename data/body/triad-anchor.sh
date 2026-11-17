#!/bin/bash
# 🔱 TRIAD ANCHOR — watches for Frame + northGate + VDJ
# Run: ./triad-anchor.sh
# Checks every 10s, deploys when a node appears

FRAME_USB="10.86.200.233"
FRAME_WIFI="192.168.4.181"
NORTHGATE="192.168.4.147"
DEPLOY_DIR="$(cd "$(dirname "$0")" && pwd)/frame-deploy"

echo "🔱 TRIAD ANCHOR — watching for outer nodes..."
echo "   Frame USB: $FRAME_USB"
echo "   Frame WiFi: $FRAME_WIFI"
echo "   northGate: $NORTHGATE"
echo ""

FRAME_DEPLOYED=0
NORTH_SEEN=0

while true; do
    NOW=$(date +%H:%M:%S)
    
    # Check Frame (USB first, then WiFi)
    FRAME_IP=""
    if ping -c 1 -W 1 "$FRAME_USB" >/dev/null 2>&1; then
        FRAME_IP="$FRAME_USB"
    elif ping -c 1 -W 1 "$FRAME_WIFI" >/dev/null 2>&1; then
        FRAME_IP="$FRAME_WIFI"
    fi
    
    if [ -n "$FRAME_IP" ] && [ "$FRAME_DEPLOYED" = "0" ]; then
        echo "[$NOW] 🐙 Frame detected at $FRAME_IP! Deploying GAIA button..."
        "$DEPLOY_DIR/deploy-gaia-button.sh" "$FRAME_IP" && FRAME_DEPLOYED=1
    elif [ -n "$FRAME_IP" ]; then
        echo "[$NOW] 🐙 Frame: alive ($FRAME_IP)"
    fi
    
    # Check northGate
    if ping -c 1 -W 1 "$NORTHGATE" >/dev/null 2>&1; then
        if [ "$NORTH_SEEN" = "0" ]; then
            echo "[$NOW] ❤️ northGate AWAKE at $NORTHGATE!"
            NORTH_SEEN=1
        fi
    else
        if [ "$NORTH_SEEN" = "1" ]; then
            echo "[$NOW] 💤 northGate went to sleep"
        fi
        NORTH_SEEN=0
    fi
    
    # VDJ is always on eastGate — just verify
    VDJ=$(curl -so /dev/null -w "%{http_code}" http://localhost:8443/vdj 2>/dev/null)
    
    if [ -z "$FRAME_IP" ] && [ "$NORTH_SEEN" = "0" ]; then
        printf "\r[$NOW] Watching... Frame:❌ northGate:❌ VDJ:$VDJ  "
    fi
    
    sleep 10
done
