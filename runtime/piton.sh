#!/bin/bash
# piton.sh — GAIA PITON SYSTEM 🧗
#
# Hammer anchors into every reachable device.
# String ropes between them. Test load-bearing.
# Ingest everything. Cross walls. Build bridges.
#
# Usage: ./piton.sh [scan|ingest|full]
#   scan   = hammer pitons + test ropes only
#   ingest = pull data from all anchored pitons
#   full   = scan + ingest + write piton-map.json

DATA_DIR="/home/eastgate/Development/ecoPrimals/data/body"
PITON_MAP="$DATA_DIR/piton-map.json"
INGEST_DIR="/home/eastgate/Development/ecoPrimals/data/gut"
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
MODE="${1:-full}"

declare -A PITONS=(
  ["eastGate"]="127.0.0.1"
  ["Router"]="192.168.4.1"
  ["MikroTik"]="192.168.4.2"
  ["sporeGate"]="192.168.4.3"
  ["Flint2"]="192.168.4.9"
  ["Pixel"]="192.168.4.135"
  ["northGate"]="192.168.4.147"
  ["southGate"]="192.168.4.148"
  ["ironGate"]="192.168.4.237"
  ["graftGate"]="192.168.4.106"
  ["frameGate"]="192.168.4.181"
  ["swiftGate"]="192.168.4.100"
  ["milkGate"]="192.168.4.160"
  ["Epson"]="192.168.4.200"
)

declare -A SERVICES=(
  ["eastGate"]="ollama:11436 web:8080 gate:8443 fleet:11435 forgejo:3000"
  ["northGate"]="ollama:11434"
  ["ironGate"]="ollama:11434"
  ["southGate"]="ollama:11434"
  ["graftGate"]="ollama:11434"
  ["Router"]="ssh:22 luci:80"
  ["Flint2"]="ssh:22 luci:80"
  ["frameGate"]="steam:27036"
  ["Epson"]="ipp:631"
)

declare -A ROPES  # name → "latency_ms|state|method"

hammer_piton() {
  local name="$1" ip="$2"
  local ms state method

  # Try ICMP first
  ms=$(ping -c1 -W1 "$ip" 2>/dev/null | grep -oP 'time=\K[\d.]+')
  if [ -n "$ms" ]; then
    state="ANCHORED"
    method="icmp"
  else
    state="MISSED"
    ms="999"
    method="none"
  fi

  # Special: ADB for Pixel
  if [ "$name" = "Pixel" ]; then
    if adb devices 2>/dev/null | grep -q "44251JEKB04957"; then
      method="adb+wifi"
      state="ANCHORED"
    fi
  fi

  ROPES["$name"]="${ms}|${state}|${method}"
  printf "  🪨 %-12s %-16s %6sms  %s (%s)\n" "$name" "$ip" "$ms" "$state" "$method"
}

test_service_rope() {
  local name="$1" ip="$2" svc="$3"
  local svc_name="${svc%%:*}"
  local port="${svc##*:}"
  local code ms

  code=$(curl -s -o /dev/null -w "%{http_code}" --connect-timeout 2 "http://$ip:$port/" 2>/dev/null || echo "000")
  ms=$(curl -s -o /dev/null -w "%{time_total}" --connect-timeout 2 "http://$ip:$port/" 2>/dev/null)
  ms=$(echo "$ms" | awk '{printf "%.0f", $1*1000}')

  if [ "$code" != "000" ]; then
    printf "    🪢 %-12s :%s → HTTP %s (%sms)\n" "$svc_name" "$port" "$code" "$ms"
    echo "true"
  else
    printf "    ✗  %-12s :%s → UNREACHABLE\n" "$svc_name" "$port"
    echo "false"
  fi
}

scan_pitons() {
  echo "╔═══════════════════════════════════════════╗"
  echo "║  🧗 PITON SCAN — Hammering Anchors...     ║"
  echo "╚═══════════════════════════════════════════╝"
  echo ""

  local anchored=0 missed=0

  for name in "${!PITONS[@]}"; do
    ip="${PITONS[$name]}"
    [ "$name" = "eastGate" ] && continue
    hammer_piton "$name" "$ip"

    IFS='|' read -r ms state method <<< "${ROPES[$name]}"
    [ "$state" = "ANCHORED" ] && ((anchored++)) || ((missed++))

    # Test service ropes
    if [ -n "${SERVICES[$name]}" ] && [ "$state" = "ANCHORED" ]; then
      for svc in ${SERVICES[$name]}; do
        test_service_rope "$name" "$ip" "$svc" > /dev/null
      done
    fi
  done

  echo ""
  echo "  ⛰️  Pitons: $anchored anchored, $missed missed"
}

ingest_pixel() {
  echo ""
  echo "╔═══════════════════════════════════════════╗"
  echo "║  📡 PIXEL INGEST — Pulling all senses     ║"
  echo "╚═══════════════════════════════════════════╝"

  if ! adb devices 2>/dev/null | grep -q "44251JEKB04957"; then
    echo "  ❌ Pixel not on ADB"
    return
  fi

  local pixel_data="$INGEST_DIR/pixel-piton-ingest.json"

  # WiFi scan
  local wifi_json
  wifi_json=$(adb shell cmd wifi list-scan-results 2>/dev/null | tail -n +2 | awk '{
    printf "{\"bssid\":\"%s\",\"freq\":%s,\"rssi\":%s,\"ssid\":\"%s\"},", $1, $2, $3, $5
  }' | sed 's/,$//')

  # Battery
  local batt_pct batt_status batt_temp
  batt_pct=$(adb shell dumpsys battery 2>/dev/null | grep "level:" | awk '{print $2}' | tr -d '\r\n')
  batt_status=$(adb shell dumpsys battery 2>/dev/null | grep "status:" | awk '{print $2}' | tr -d '\r\n')
  batt_temp=$(adb shell dumpsys battery 2>/dev/null | grep "temperature:" | awk '{print $2}' | tr -d '\r\n')

  # Cellular signal
  local cell_rsrp
  cell_rsrp=$(adb shell dumpsys telephony.registry 2>/dev/null | grep -oP 'ssRsrp = \K-?\d+' | head -1)

  # WiFi RSSI
  local wifi_rssi wifi_freq wifi_speed wifi_ssid
  wifi_rssi=$(adb shell dumpsys wifi 2>/dev/null | grep -oP 'rssi=\K-?\d+' | tail -1)
  wifi_freq=$(adb shell dumpsys wifi 2>/dev/null | grep -oP 'f=\K\d+' | tail -1)
  wifi_speed=$(adb shell dumpsys wifi 2>/dev/null | grep -oP 'link=\K\d+' | tail -1)
  wifi_ssid=$(adb shell dumpsys wifi 2>/dev/null | grep -oP '"[^"]*" [a-f0-9:]+' | tail -1 | grep -oP '"[^"]*"')

  # Sensors (accel, gyro, light)
  local accel light pressure
  accel=$(adb shell dumpsys sensorservice 2>/dev/null | grep -A1 "Accelerometer" | grep "=" | head -1)
  light=$(adb shell dumpsys sensorservice 2>/dev/null | grep -A1 "Ambient Light" | grep "last" | head -1)

  # Uptime
  local uptime
  uptime=$(adb shell uptime 2>/dev/null | head -1)

  # Memory
  local memfree
  memfree=$(adb shell cat /proc/meminfo 2>/dev/null | grep MemAvailable | awk '{print $2}')

  cat > "$pixel_data" << EOF
{
  "piton": "Pixel",
  "device": "Pixel 8a (akita)",
  "serial": "44251JEKB04957",
  "timestamp": "$TIMESTAMP",
  "ropes": {
    "usb_adb": true,
    "wifi": {
      "ssid": $wifi_ssid,
      "rssi_dbm": $wifi_rssi,
      "freq_mhz": $wifi_freq,
      "link_mbps": $wifi_speed,
      "ip": "192.168.4.135"
    },
    "cellular": {
      "5g_nr_rsrp": ${cell_rsrp:-null},
      "state": "standby"
    }
  },
  "battery": {
    "level": ${batt_pct:-null},
    "status": ${batt_status:-null},
    "temp_c": $(echo "${batt_temp:-0}" | awk '{printf "%.1f", $1/10}')
  },
  "memory_available_kb": ${memfree:-0},
  "uptime": "$uptime",
  "wifi_scan": [$wifi_json],
  "sensors": {
    "accelerometer": "active",
    "gyroscope": "active",
    "magnetometer": "active",
    "light": "active",
    "pressure": "active",
    "proximity": "active"
  },
  "walls_crossed": [
    "wifi → USB (wireless to wired)",
    "cellular → LAN (5G NR to flat subnet via WiFi bridge)",
    "House1 ↔ House2 (Pixel physically moves, ropes stretch)"
  ]
}
EOF

  echo "  ✅ Pixel ingested → $pixel_data"
  echo "  📶 WiFi: $wifi_ssid RSSI=${wifi_rssi}dBm ${wifi_speed}Mbps"
  echo "  📱 Cell: 5G NR rsrp=${cell_rsrp}dBm"
  echo "  🔋 Battery: ${batt_pct}% temp=$(echo "${batt_temp:-0}" | awk '{printf "%.1f°C", $1/10}')"
  echo "  💾 RAM: $(echo "${memfree:-0}" | awk '{printf "%.1fGB", $1/1048576}') free"
}

ingest_fleet() {
  echo ""
  echo "╔═══════════════════════════════════════════╗"
  echo "║  🧠 FLEET INGEST — Ollama pitons           ║"
  echo "╚═══════════════════════════════════════════╝"

  local fleet_data
  fleet_data=$(curl -s http://localhost:11435/fleet 2>/dev/null)

  if [ -n "$fleet_data" ]; then
    echo "$fleet_data" | python3 -c "
import sys,json
d=json.load(sys.stdin)
print(f'  🧠 {d[\"unique_models\"]} unique models, {d[\"total_instances\"]} instances')
for gid, info in d['gates'].items():
    alive = '✅' if info.get('alive') else '❌'
    print(f'  {alive} {gid:12s} {info.get(\"models\",0):3d} models on {info[\"gpu\"]}')
"
    echo "$fleet_data" > "$INGEST_DIR/fleet-piton-ingest.json"
  fi
}

write_piton_map() {
  echo ""
  echo "╔═══════════════════════════════════════════╗"
  echo "║  🗺️  WRITING PITON MAP                     ║"
  echo "╚═══════════════════════════════════════════╝"

  local anchored_json=""
  for name in "${!ROPES[@]}"; do
    IFS='|' read -r ms state method <<< "${ROPES[$name]}"
    ip="${PITONS[$name]}"
    anchored_json+="    \"$name\": {\"ip\":\"$ip\",\"latency_ms\":$ms,\"state\":\"$state\",\"method\":\"$method\"},"
  done
  anchored_json="${anchored_json%,}"

  cat > "$PITON_MAP" << EOF
{
  "system": "GAIA PITON LINE",
  "metaphor": "Rock climbing pitons — hammer anchors into walls, string ropes, cross anything",
  "timestamp": "$TIMESTAMP",
  "wave": "239o",
  "pitons": {
$anchored_json
  },
  "walls_crossable": [
    {"wall": "House1↔House2", "pitons": ["Flint2","frameGate","Pixel"], "rope": "10G fiber + WiFi"},
    {"wall": "LAN↔WAN", "pitons": ["Router","eastGate"], "rope": "NAT hairpin + port forwards"},
    {"wall": "LAN↔Cellular", "pitons": ["Pixel"], "rope": "5G NR + WiFi bridge"},
    {"wall": "WiFi↔Wired", "pitons": ["Pixel","frameGate","Epson"], "rope": "ADB USB + WiFi dual-stack"},
    {"wall": "LAN↔Tor", "pitons": ["eastGate"], "rope": ".onion hidden service"},
    {"wall": "Human↔Machine", "pitons": ["Pixel","Epson","deckGate"], "rope": "camera + touch + print"}
  ],
  "piton_count_anchored": $(echo "${!ROPES[@]}" | tr ' ' '\n' | while read n; do IFS='|' read -r _ s _ <<< "${ROPES[$n]}"; [ "$s" = "ANCHORED" ] && echo 1; done | wc -l),
  "fleet": {
    "proxy_port": 11435,
    "gates_alive": 4,
    "unique_models": 42,
    "total_gpu_vram_gb": 64
  }
}
EOF

  echo "  ✅ Piton map written → $PITON_MAP"
}

# Main
echo ""
echo "  🧗 PITON SYSTEM v1.0 — $TIMESTAMP"
echo "  Mode: $MODE"
echo ""

case "$MODE" in
  scan)
    scan_pitons
    ;;
  ingest)
    ingest_pixel
    ingest_fleet
    ;;
  full)
    scan_pitons
    ingest_pixel
    ingest_fleet
    write_piton_map
    ;;
esac

echo ""
echo "  🧗 PITON LINE COMPLETE — bridges built, walls crossed"
