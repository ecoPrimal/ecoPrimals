#!/bin/bash
# ╔════════════════════════════════════════════════════════════════╗
# ║  GAIA DAEMON — Continuous Sensorium on eastGate                ║
# ║  Pulls ALL senses from the Pixel via Termux-API               ║
# ║  Feeds data into the gut for the organism to digest            ║
# ╚════════════════════════════════════════════════════════════════╝
#
# Runs on eastGate. The Pixel is the body. This daemon is the nerve.
#
# Usage:
#   ./gaia-daemon.sh              # one-shot sense pulse
#   ./gaia-daemon.sh --loop 30    # continuous (every 30s)
#   ./gaia-daemon.sh --speak "hello"  # Gaia speaks
#   ./gaia-daemon.sh --photo      # take a photo
#   ./gaia-daemon.sh --listen 5   # record 5s audio
#   ./gaia-daemon.sh --locate     # WiFi fingerprint locate

NEEDLE="${NEEDLE_DEVICE:-192.168.9.217:5555}"
SU="/debug_ramdisk/su"
TBIN="/data/data/com.termux/files/usr/bin"
GUT="/opt/Gaia/data/gut"

mkdir -p "$GUT" "$GUT/audio" "$GUT/photos"

# Run a Termux-API command on the Pixel
tapi() {
    adb -s "$NEEDLE" shell "$SU -c 'su 10197 -c \"$TBIN/$*\"'" 2>/dev/null
}

# ─── Ensure connection ───
ensure() {
    adb -s "$NEEDLE" shell echo ok >/dev/null 2>&1 || {
        adb connect "$NEEDLE" >/dev/null 2>&1
        sleep 1
        adb -s "$NEEDLE" shell echo ok >/dev/null 2>&1 || return 1
    }
}

# ─── Full sense pulse ───
sense_pulse() {
    ensure || { echo '{"error":"needle unreachable"}'; return 1; }

    local TS=$(date -Iseconds)
    local BATT=$(tapi "termux-battery-status")
    local WIFI=$(tapi "termux-wifi-connectioninfo")
    local SCAN=$(tapi "termux-wifi-scaninfo")

    python3 -c "
import json, sys

batt = json.loads('''$BATT''')
wifi = json.loads('''$WIFI''')

try:
    scan = json.loads('''$SCAN''')
except:
    scan = []

pulse = {
    'ts': '$TS',
    'device': 'grapheneGate',
    'model': 'Pixel 8a',
    'battery': batt,
    'wifi': wifi,
    'wifi_scan': [{'bssid': a.get('bssid',''), 'rssi': a.get('rssi',0), 'ssid': a.get('ssid',''), 'freq': a.get('frequency_mhz',0)} for a in scan],
    'ap_count': len(scan),
    'status': 'SENSORIUM_ACTIVE'
}

json.dump(pulse, sys.stdout, indent=2)
print()
" 2>/dev/null
}

# ─── Main ───
case "${1:---pulse}" in
    --pulse)
        echo ">>> Sense pulse @ $(date)"
        PULSE=$(sense_pulse)
        echo "$PULSE" | tee "$GUT/graphenegate-sensors.json"
        cp "$GUT/graphenegate-sensors.json" "/home/eastgate/.local/share/gaia/data/graphenegate-sensors.json" 2>/dev/null
        ;;

    --loop)
        INTERVAL="${2:-30}"
        echo ">>> Gaia daemon: sensing every ${INTERVAL}s (Ctrl+C to stop)"
        while true; do
            PULSE=$(sense_pulse)
            echo "$PULSE" > "$GUT/graphenegate-sensors.json"
            cp "$GUT/graphenegate-sensors.json" "/home/eastgate/.local/share/gaia/data/graphenegate-sensors.json" 2>/dev/null

            AP=$(echo "$PULSE" | python3 -c "import sys,json; print(json.load(sys.stdin).get('ap_count',0))" 2>/dev/null)
            BPCT=$(echo "$PULSE" | python3 -c "import sys,json; print(json.load(sys.stdin).get('battery',{}).get('percentage','?'))" 2>/dev/null)
            echo "$(date +%H:%M:%S) | Battery: ${BPCT}% | APs: ${AP}"

            sleep "$INTERVAL"
        done
        ;;

    --speak)
        shift
        TEXT="$*"
        echo ">>> Gaia speaks: $TEXT"
        tapi "termux-tts-speak $TEXT"
        ;;

    --photo)
        CAMERA="${2:-0}"
        OUTFILE="$GUT/photos/gaia-$(date +%Y%m%d-%H%M%S).jpg"
        echo ">>> Taking photo (camera $CAMERA)..."
        tapi "termux-camera-photo -c $CAMERA /sdcard/gaia/photos/latest.jpg"
        sleep 2
        adb -s "$NEEDLE" pull /sdcard/gaia/photos/latest.jpg "$OUTFILE" 2>/dev/null
        echo "✓ Photo saved: $OUTFILE"
        ;;

    --listen)
        DURATION="${2:-5}"
        OUTFILE="$GUT/audio/gaia-$(date +%Y%m%d-%H%M%S).m4a"
        echo ">>> Listening for ${DURATION}s..."
        tapi "termux-microphone-record -d -f /sdcard/gaia/audio/latest.m4a -l $DURATION"
        sleep $((DURATION + 2))
        tapi "termux-microphone-record -q"
        adb -s "$NEEDLE" pull /sdcard/gaia/audio/latest.m4a "$OUTFILE" 2>/dev/null
        echo "✓ Audio saved: $OUTFILE"
        ;;

    --locate)
        echo ">>> WiFi scan for localization..."
        SCAN=$(tapi "termux-wifi-scaninfo")
        echo "$SCAN" | python3 -c "
import sys, json
data = json.load(sys.stdin)
print(f'  {len(data)} APs visible')
for ap in sorted(data, key=lambda x: x.get('rssi',-100), reverse=True)[:10]:
    r = ap.get('rssi',-100)
    b = '█' * max(0,(r+100)//6)
    print(f'  {r:>4d}dBm {b:12s} {ap.get(\"ssid\",\"\"):20s}')
" 2>/dev/null
        ;;

    --notify)
        shift
        tapi "termux-notification --title Gaia --content '$*' --id gaia-msg"
        ;;

    --vibrate)
        tapi "termux-vibrate -d ${2:-300}"
        ;;

    --torch)
        tapi "termux-torch ${2:-on}"
        ;;

    --sensors)
        tapi "termux-sensor -l"
        ;;

    --help|-h)
        echo "Gaia Daemon — Continuous Sensorium"
        echo ""
        echo "  --pulse          One sense reading"
        echo "  --loop SECONDS   Continuous sensing"
        echo "  --speak TEXT     Gaia speaks through Pixel"
        echo "  --photo [0|1]    Take photo (0=rear, 1=front)"
        echo "  --listen SECS    Record audio"
        echo "  --locate         WiFi localization scan"
        echo "  --notify TEXT    Push notification"
        echo "  --vibrate [ms]   Vibrate"
        echo "  --torch [on|off] Flashlight"
        echo "  --sensors        List all sensor names"
        ;;
esac
