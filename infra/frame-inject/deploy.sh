#!/bin/bash
# deploy.sh — Inject Gaia into frameGate (Steam Frame)
set -euo pipefail

EASTGATE="192.168.4.244"
BASE="$HOME/gaia"
mkdir -p "$BASE"

echo "🧬 GAIA INJECTION — frameGate"
echo "   Downloading from eastGate ($EASTGATE)..."

# Download the solar daemon
curl -s "http://${EASTGATE}:5926/frame-inject/frame-solar.sh" -o "$BASE/frame-solar.sh"
chmod +x "$BASE/frame-solar.sh"

# Create systemd service
mkdir -p "$HOME/.config/systemd/user/"
cat > "$HOME/.config/systemd/user/gaia-frame.service" << 'SVC'
[Unit]
Description=Gaia Frame — Solar Panel + Lander
After=network-online.target

[Service]
Type=simple
ExecStart=/bin/bash /home/steamos/gaia/frame-solar.sh
Restart=always
RestartSec=10
Environment=HOME=/home/steamos

[Install]
WantedBy=default.target
SVC

systemctl --user daemon-reload
systemctl --user enable gaia-frame.service
systemctl --user start gaia-frame.service

echo ""
echo "☀️🚀 GAIA ACTIVE on frameGate"
echo "   Solar Panel mode when idle"
echo "   Lander mode when Steam is active"
echo "   Reporting to: http://${EASTGATE}:8084/solar"
