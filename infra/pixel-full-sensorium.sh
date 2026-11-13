#!/system/bin/sh
# ═══════════════════════════════════════════════════════════════
#  gAIa FULL SENSORIUM — Absorbs ALL hardware through the membrane
#  Uses Android dumpsys (works from any shell, no Termux:API needed)
#  This is the AMALGAM sensor layer: GrapheneOS HAL → biomeOS data
# ═══════════════════════════════════════════════════════════════
GAIA="/data/local/tmp/biomeos"
OUT="$GAIA/sensorium/full-body.json"
INTERVAL="${1:-30}"

while true; do
  TS=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  
  # Battery = metabolic state
  BAT_LEVEL=$(dumpsys battery 2>/dev/null | grep "level:" | awk '{print $2}')
  BAT_TEMP=$(dumpsys battery 2>/dev/null | grep "temperature:" | awk '{print $2}')
  BAT_STATUS=$(dumpsys battery 2>/dev/null | grep "status:" | awk '{print $2}')
  BAT_CHARGING=$(dumpsys battery 2>/dev/null | grep "AC powered:" | awk '{print $2}')
  BAT_USB=$(dumpsys battery 2>/dev/null | grep "USB powered:" | awk '{print $2}')
  
  # Display = face state
  SCREEN=$(dumpsys display 2>/dev/null | grep "mScreenState=" | head -1 | sed 's/.*=//')
  BRIGHTNESS=$(dumpsys display 2>/dev/null | grep "mBrightness=" | head -1 | sed 's/.*=//')
  
  # WiFi = spatial field
  WIFI_RSSI=$(dumpsys wifi 2>/dev/null | grep "mWifiInfo" | grep -o "RSSI: [0-9-]*" | head -1 | awk '{print $2}')
  WIFI_SSID=$(dumpsys wifi 2>/dev/null | grep "mWifiInfo" | grep -o 'SSID: [^,]*' | head -1 | sed 's/SSID: //')
  WIFI_FREQ=$(dumpsys wifi 2>/dev/null | grep "mWifiInfo" | grep -o "Frequency: [0-9]*" | head -1 | awk '{print $2}')
  WIFI_SPEED=$(dumpsys wifi 2>/dev/null | grep "mWifiInfo" | grep -o "Link speed: [0-9]*" | head -1 | awk '{print $3}')
  
  # Thermal = body temperature
  THERMAL=$(dumpsys thermalservice 2>/dev/null | grep "mStatus=" | head -1 | sed 's/.*=//')
  
  # Memory = mental load
  MEM_FREE=$(cat /proc/meminfo | grep MemAvailable | awk '{print $2}')
  MEM_TOTAL=$(cat /proc/meminfo | grep MemTotal | awk '{print $2}')
  
  # CPU = heartbeat
  CPU_LOAD=$(cat /proc/loadavg | awk '{print $1}')
  
  # Uptime = consciousness duration
  UPTIME=$(cat /proc/uptime | awk '{print $1}')
  
  # Network = circulation
  RX_BYTES=$(cat /proc/net/dev | grep wlan0 | awk '{print $2}')
  TX_BYTES=$(cat /proc/net/dev | grep wlan0 | awk '{print $10}')
  
  # Process count = cellular activity
  PROC_COUNT=$(ls -d /proc/[0-9]* 2>/dev/null | wc -l)
  
  # Primal count = organ health
  PRIMAL_COUNT=$(ps -ef 2>/dev/null | grep "$GAIA/primals/" | grep -v grep | wc -l)
  
  # Gradient field (from our other daemon)
  GRADIENT_DIR=""
  GRADIENT_VAL=""
  if [ -f "$GAIA/sensorium/gradient-field.json" ]; then
    GRADIENT_VAL=$(cat "$GAIA/sensorium/gradient-field.json" | grep '"gradient":' | head -1 | sed 's/[^0-9-]//g')
    GRADIENT_DIR=$(cat "$GAIA/sensorium/gradient-field.json" | grep 'gradient_direction' | sed 's/.*: "//;s/".*//')
  fi
  
  # Self IP
  SELF_IP=$(ip addr show wlan0 2>/dev/null | grep 'inet ' | awk '{print $2}' | cut -d/ -f1)

  cat > "$OUT" << EOF
{
  "ts": "$TS",
  "identity": "gAIa",
  "body_type": "amalgam",
  "self_ip": "$SELF_IP",
  "metabolic": {
    "battery_pct": $BAT_LEVEL,
    "temperature_deci_c": ${BAT_TEMP:-0},
    "status": "$BAT_STATUS",
    "charging_ac": $BAT_CHARGING,
    "charging_usb": $BAT_USB
  },
  "face": {
    "screen_state": "$SCREEN",
    "brightness": "$BRIGHTNESS"
  },
  "spatial": {
    "wifi_ssid": "$WIFI_SSID",
    "wifi_rssi": ${WIFI_RSSI:-0},
    "wifi_freq_mhz": ${WIFI_FREQ:-0},
    "wifi_speed_mbps": ${WIFI_SPEED:-0},
    "gradient": ${GRADIENT_VAL:-0},
    "gradient_direction": "$GRADIENT_DIR"
  },
  "thermal": "$THERMAL",
  "mind": {
    "mem_available_kb": $MEM_FREE,
    "mem_total_kb": $MEM_TOTAL,
    "mem_pct_free": $(( MEM_FREE * 100 / MEM_TOTAL )),
    "cpu_load": "$CPU_LOAD",
    "process_count": $PROC_COUNT,
    "primal_count": $PRIMAL_COUNT
  },
  "circulation": {
    "rx_bytes": ${RX_BYTES:-0},
    "tx_bytes": ${TX_BYTES:-0}
  },
  "consciousness": {
    "uptime_sec": "$UPTIME"
  },
  "amalgam_note": "GrapheneOS HAL → dumpsys → this JSON. The membrane passes data inward. The substrate is invisible."
}
EOF

  sleep $INTERVAL
done
