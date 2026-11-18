#!/bin/bash
# 🍞 BREADCRUMB — follow this trail home to eastGate
# Run this on ANY gate to phone home. Works over WiFi, cellular, USB tether.

echo "🍞 Following the breadcrumb trail..."
echo "Gate: $(hostname)"
echo "OS: $(cat /etc/os-release 2>/dev/null | grep PRETTY_NAME | cut -d= -f2)"
echo "Arch: $(uname -m)"
echo ""

# Try EVERY path home
PATHS=(
  "192.168.4.244"     # eastGate LAN (router)
  "192.168.1.131"     # eastGate ISP subnet
  "162.226.225.148"   # eastGate WAN/public
  "8.8.8.8"           # internet check
)

FOUND=""
for p in "${PATHS[@]}"; do
  if ping -c1 -W2 $p >/dev/null 2>&1; then
    echo "✅ Can reach $p"
    FOUND="$p"
  else
    echo "❌ Cannot reach $p"
  fi
done

echo ""
if [ -z "$FOUND" ]; then
  echo "💀 No path home! Need WiFi/USB tether/cellular."
  echo "Try: nmcli dev wifi connect 'Aperture Science' password 'exoticpanda714'"
  exit 1
fi

# Enable SSH
echo "--- Enabling SSH ---"
sudo systemctl enable sshd 2>/dev/null || sudo systemctl enable ssh 2>/dev/null
sudo systemctl start sshd 2>/dev/null || sudo systemctl start ssh 2>/dev/null
echo ""

# Report back
echo "--- Network info ---"
ip addr show | grep "inet " | grep -v 127
echo ""
echo "--- Gateway ---"
ip route show | head -3
echo ""

# Try reverse SSH to eastGate
echo "--- Attempting reverse tunnel to eastGate ---"
echo "If this works, eastGate can SSH back to this gate!"
echo "Run manually: ssh -R 0:localhost:22 eastgate@162.226.225.148 -p 22"
echo ""

echo "🍞 Breadcrumb complete! $(hostname) is at:"
ip addr show | grep "inet " | grep -v "127\|docker\|virbr" | awk '{print "  " $2}'
