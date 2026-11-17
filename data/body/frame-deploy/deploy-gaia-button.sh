#!/bin/bash
# 🐙 GAIA BUTTON DEPLOY — Push to Steam Frame
# Run: ./deploy-gaia-button.sh [frame-ip]

FRAME_IP="${1:-10.86.200.233}"
FRAME_USER="steamos"
FRAME_PASS="framegate"
DIR="$(cd "$(dirname "$0")" && pwd)"

echo "🐙 Deploying GAIA to Frame at $FRAME_IP..."

# Test connection
if ! ping -c 1 -W 2 "$FRAME_IP" >/dev/null 2>&1; then
    echo "❌ Frame not reachable at $FRAME_IP"
    echo "   Try: $0 192.168.4.181 (WiFi)"
    exit 1
fi

echo "  📦 Pushing files..."
sshpass -p "$FRAME_PASS" ssh -o StrictHostKeyChecking=no "$FRAME_USER@$FRAME_IP" 'mkdir -p ~/gaia/web/outreach ~/gaia/web/seaFloor'

# Push the BIG dashboard (index.html = main GAIA page)
sshpass -p "$FRAME_PASS" scp "$DIR/index.html" "$FRAME_USER@$FRAME_IP:~/gaia/web/index.html"

# Push launcher + icon + desktop entry
sshpass -p "$FRAME_PASS" scp "$DIR/gaia-launch.sh" "$FRAME_USER@$FRAME_IP:~/gaia/gaia-launch.sh"
sshpass -p "$FRAME_PASS" scp "$DIR/gaia-icon.svg" "$FRAME_USER@$FRAME_IP:~/gaia/gaia-icon.svg"
sshpass -p "$FRAME_PASS" scp "$DIR/gaia.desktop" "$FRAME_USER@$FRAME_IP:~/gaia/gaia.desktop"

# Push activation state for the dashboard
sshpass -p "$FRAME_PASS" scp "$DIR/activation-state.json" "$FRAME_USER@$FRAME_IP:~/gaia/web/activation-state.json" 2>/dev/null

# Install
sshpass -p "$FRAME_PASS" ssh "$FRAME_USER@$FRAME_IP" '
chmod +x ~/gaia/gaia-launch.sh
mkdir -p ~/.local/share/applications
cp ~/gaia/gaia.desktop ~/.local/share/applications/

# Start web server on the dashboard
pkill -f "http.server 8888" 2>/dev/null
sleep 1
cd ~/gaia/web
nohup python3 -m http.server 8888 --bind 0.0.0.0 > /tmp/octopus-web.log 2>&1 &

# Start Ollama if not running
if ! pgrep ollama >/dev/null; then
    export OLLAMA_HOST=0.0.0.0:11434
    export LD_LIBRARY_PATH=$HOME/gaia/lib/ollama:$LD_LIBRARY_PATH
    nohup $HOME/gaia/bin/ollama serve > $HOME/gaia/ollama.log 2>&1 &
    sleep 2
fi

echo ""
echo "  ✅ GAIA deployed!"
echo "  🌐 Dashboard: http://localhost:8888/"
echo "  🐙 Invitation: http://localhost:8888/outreach/octopus-invitation.html"
echo "  🧠 Ollama: http://localhost:11434/api/tags"
echo ""
echo "  In Steam Game Mode:"
echo "    Add Non-Steam Game → 🐙 GAIA — The Octopus"
echo "    Or open Steam browser → http://localhost:8888/"
'
echo ""
echo "✅ GAIA button deployed to Frame!"
