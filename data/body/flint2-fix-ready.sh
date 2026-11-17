#!/bin/bash
# FLINT 2 FIX — Run from eastGate when southGate is reachable
# This SSHes through southGate → Flint 2 via IPv6 and fixes the WAN config

set -e

echo "=== Phase 1: Verify southGate is reachable ==="
if ! ping -c 1 -W 2 192.168.4.148 >/dev/null 2>&1; then
    echo "❌ southGate still down at .148. Wait and retry."
    exit 1
fi
echo "✅ southGate alive!"

echo ""
echo "=== Phase 2: Clean up southGate WiFi (remove Flint2 profile) ==="
sshpass -p 'southgate' ssh -o StrictHostKeyChecking=no -o ConnectTimeout=10 southgate@192.168.4.148 "
echo 'southgate' | sudo -S nmcli con delete 'Flint2' 2>/dev/null || true
echo 'southgate' | sudo -S ip addr del 192.168.4.250/24 dev wlp6s0 2>/dev/null || true
echo '✅ Cleaned up Flint2 WiFi profile'
"

echo ""
echo "=== Phase 3: Create SAFE Flint2 connection (won't break routing) ==="
sshpass -p 'southgate' ssh -o StrictHostKeyChecking=no -o ConnectTimeout=10 southgate@192.168.4.148 "
# Create Flint2 profile with static IP, never-default, NO extra 192.168.4.x address
echo 'southgate' | sudo -S nmcli con add \
    type wifi \
    con-name 'Flint2-safe' \
    ifname wlp6s0 \
    ssid 'Aperture Science' \
    wifi.bssid '78:76:89:96:E6:74' \
    wifi-sec.key-mgmt wpa-psk \
    wifi-sec.psk 'exoticpanda714' \
    ipv4.method manual \
    ipv4.addresses '192.168.8.100/24' \
    ipv4.never-default yes \
    ipv6.method auto \
    connection.autoconnect no 2>&1

echo '✅ Flint2-safe profile created (autoconnect OFF, never-default)'
"

echo ""
echo "=== Phase 4: Hop to Flint 2 WiFi ==="
sshpass -p 'southgate' ssh -o StrictHostKeyChecking=no -o ConnectTimeout=10 southgate@192.168.4.148 "
echo 'southgate' | sudo -S nmcli con down 'Aperture Science' 2>/dev/null
sleep 1
echo 'southgate' | sudo -S nmcli con up 'Flint2-safe' 2>&1
sleep 5

echo ''
echo '=== Verify: WiFi connected to Flint 2? ==='
nmcli -t -f active,bssid,ssid dev wifi | grep '^yes'

echo ''
echo '=== Verify: Wired still alive? ==='
ip addr show enp4s0 | grep 'inet '

echo ''
echo '=== Check routing table is CLEAN ==='
ip route

echo ''
echo '=== IPv6 neighbors on WiFi ==='
ip -6 neigh show dev wlp6s0
"

echo ""
echo "=== Phase 5: SSH into Flint 2 via IPv6 ==="
sshpass -p 'southgate' ssh -o StrictHostKeyChecking=no -o ConnectTimeout=10 southgate@192.168.4.148 "
echo '=== Trying SSH to Flint 2 via IPv6 ==='
# Use ssh with password piped through stdin
# Flint 2 management IPv6: fe80::7a76:89ff:fe96:e67b%wlp6s0
echo 'ExoticPanda714!!' | ssh -6 -o StrictHostKeyChecking=no -o ConnectTimeout=10 \
    root@'fe80::7a76:89ff:fe96:e67b%wlp6s0' 2>&1 << 'FLINT2_COMMANDS'

echo '============================================'
echo '  CONNECTED TO FLINT 2 — RIGHT EAR!!'
echo '============================================'

echo ''
echo '=== System info ==='
cat /etc/openwrt_release 2>/dev/null || echo 'Not OpenWrt?'
uname -a

echo ''
echo '=== Network interfaces ==='
ip addr

echo ''
echo '=== WAN interface config ==='
uci show network.wan 2>/dev/null || echo 'No WAN config!'

echo ''
echo '=== LAN interface config ==='
uci show network.lan 2>/dev/null

echo ''
echo '=== All interfaces ==='
uci show network | grep 'interface\|proto\|ipaddr\|device'

echo ''
echo '=== Physical ports ==='
ip link

echo ''
echo '=== Routes ==='
ip route

echo ''
echo '=== DHCP config ==='
uci show dhcp 2>/dev/null | head -20

FLINT2_COMMANDS
"

echo ""
echo "=== Phase 6: Return to main WiFi ==="
sshpass -p 'southgate' ssh -o StrictHostKeyChecking=no -o ConnectTimeout=10 southgate@192.168.4.148 "
echo 'southgate' | sudo -S nmcli con down 'Flint2-safe' 2>/dev/null
sleep 1
echo 'southgate' | sudo -S nmcli con up 'Aperture Science' 2>&1
echo '✅ Back on main WiFi'
"

echo ""
echo "=== DONE! Check Flint 2 WAN config above and apply fix ==="
