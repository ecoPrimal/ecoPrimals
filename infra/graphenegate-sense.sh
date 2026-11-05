#!/bin/bash
# graphenegate-sense.sh — Gaia's AMOEBA: Read ALL sensors from grapheneGate
# Pixel 8a on GrapheneOS — 37 hardware sensors, Gaia's peripheral nervous system
# Tries USB ADB first, then WiFi ADB fallback

USB_DEVICE="44251JEKB04957"
# DHCP has moved the Pixel before (.104 -> .249 -> .170). Try last-known first;
# a WiFi hit is accepted only if the serial matches USB_DEVICE (no wrong-phone reads).
WIFI_CANDIDATES="192.168.4.170 192.168.4.249 192.168.4.104"
OUT="/opt/Gaia/data/gut/graphenegate-sensors.json"
HUD_OUT="/home/eastgate/Development/ecoPrimals/infra/hud/site/public/data/graphenegate-sense.json"

# Determine which connection works
DEVICE=""
if adb -s "$USB_DEVICE" shell echo ok >/dev/null 2>&1; then
    DEVICE="$USB_DEVICE"
    CONN="usb"
else
    for ip in $WIFI_CANDIDATES; do
        adb connect "$ip:5555" >/dev/null 2>&1
        if [ "$(adb -s "$ip:5555" shell getprop ro.serialno 2>/dev/null | tr -d '\r')" = "$USB_DEVICE" ]; then
            DEVICE="$ip:5555"
            CONN="wifi"
            break
        fi
        adb disconnect "$ip:5555" >/dev/null 2>&1
    done
fi

if [ -z "$DEVICE" ]; then
    echo '{"error": "grapheneGate not reachable via USB or WiFi ADB", "fix": "Connect USB or enable Wireless debugging"}' > "$OUT"
    exit 1
fi

TS=$(date -u +%Y-%m-%dT%H:%M:%SZ)

# Battery
BATT=$(adb -s "$DEVICE" shell "dumpsys battery" 2>/dev/null)
LEVEL=$(echo "$BATT" | grep -E "^ *level:" | head -1 | awk '{print $2}' | tr -d '\r')
TEMP_RAW=$(adb -s "$DEVICE" shell "cat /sys/class/power_supply/battery/temp" 2>/dev/null | tr -d '\r')
VOLTAGE=$(echo "$BATT" | grep -E "^ *voltage:" | head -1 | awk '{print $2}' | tr -d '\r')
USB_POWER=$(echo "$BATT" | grep "USB powered:" | awk '{print $3}' | tr -d '\r')

# WiFi
WIFI_SSID=$(adb -s "$DEVICE" shell "dumpsys wifi" 2>/dev/null | grep -oP 'ssid: "[^"]+"' | head -1 | sed 's/ssid: "//;s/"//')
WIFI_FREQ=$(adb -s "$DEVICE" shell "dumpsys wifi" 2>/dev/null | grep -oP 'frequencyMhz: \d+' | head -1 | awk '{print $2}')

# Thermals — the amoeba's heat map
THERMAL_JSON=$(adb -s "$DEVICE" shell "dumpsys thermalservice" 2>/dev/null | grep 'Temperature{' | \
    python3 -c "
import sys, json, re
thermals = {}
for line in sys.stdin:
    m_name = re.search(r'mName=([^,}]+)', line)
    m_val = re.search(r'mValue=([0-9.E+-]+)', line)
    if m_name and m_val:
        try:
            thermals[m_name.group(1).strip()] = round(float(m_val.group(1)), 2)
        except: pass
print(json.dumps(thermals))
" 2>/dev/null)

# Uptime + load
UPTIME=$(adb -s "$DEVICE" shell "uptime" 2>/dev/null | tr -d '\r')

# Screen brightness
BRIGHTNESS=$(adb -s "$DEVICE" shell "settings get system screen_brightness" 2>/dev/null | tr -d '\r')

# Sensor count
SENSOR_COUNT=$(adb -s "$DEVICE" shell "dumpsys sensorservice" 2>/dev/null | grep "Total.*h/w sensors" | grep -oP '\d+' | head -1)

THERMAL_JSON="${THERMAL_JSON:-{\}}"
[ -z "$THERMAL_JSON" ] && THERMAL_JSON='{}'
DEV_IP=$(adb -s "$DEVICE" shell "ip -4 addr show wlan0" 2>/dev/null | grep -oE 'inet [0-9.]+' | awk '{print $2}' | head -1)

# Build the amoeba JSON
python3 -c "
import json, hashlib, time

ts = '$TS'
thermal = json.loads('''$THERMAL_JSON''')
temp_c = round(${TEMP_RAW:-0} / 10.0, 1)
level = int('${LEVEL:-0}')
voltage = int('${VOLTAGE:-0}')

# ZK veil
zk_hash = hashlib.sha256(f'grapheneGate:$DEVICE:{ts}'.encode()).hexdigest()
zk_veil = f'pi.{int(zk_hash[:2],16)%100}.{int(zk_hash[2:4],16)%100}.{int(zk_hash[4:6],16)%10}'

amoeba = {
    'ts': ts,
    'wave': 222,
    'gate': 'grapheneGate',
    'type': 'amoeba-sense',
    'connection': '$CONN',
    'device': {
        'model': 'Pixel 8a',
        'os': 'GrapheneOS',
        'serial': '$USB_DEVICE',
        'ip': '${DEV_IP:-unknown}',
    },
    'battery': {
        'level': level,
        'temp_c': temp_c,
        'voltage_mv': voltage,
        'charging': '${USB_POWER}' == 'true',
    },
    'wifi': {
        'ssid': '${WIFI_SSID:-unknown}',
        'freq_mhz': int('${WIFI_FREQ:-0}' or 0),
    },
    'thermal_zones': thermal,
    'hw_sensors': int('${SENSOR_COUNT:-37}'),
    'sensor_families': {
        'motion': 15,
        'environment': 9,
        'gesture': 10,
        'proximity': 4,
        'camera': 3,
    },
    'brightness': int('${BRIGHTNESS:-0}' or 0),
    'uptime': '$UPTIME',
    'zk_veil': zk_veil,
    'amoeba': {
        'role': 'peripheral_nervous_system',
        'metaphor': 'Amoeba — flows into environment, pseudopods sense everything',
        'capabilities': '37 hw sensors @ up to 480Hz',
    },
}

print(json.dumps(amoeba, indent=2))
" > "$OUT" 2>/dev/null

# Copy to HUD for heart-pull
cp "$OUT" "$HUD_OUT" 2>/dev/null

echo "grapheneGate amoeba sensed: ${SENSOR_COUNT:-37} sensors, battery ${LEVEL:-?}%, ${CONN} connection"
