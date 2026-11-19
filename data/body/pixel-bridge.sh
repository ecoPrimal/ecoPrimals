#!/bin/bash
# pixel-bridge.sh — ADB bridge: eastGate keyboard/mouse → Pixel screen
# Run FROM eastGate. Pixel connected via USB or WiFi ADB.
#
# The Pixel has NO native Barrier client (Android).
# Instead, we use scrcpy: mirrors the screen + sends touch/key input.
# This IS the blood-brain barrier for the Pixel — eastGate controls it.
#
# Usage:
#   ./pixel-bridge.sh              # auto-detect + mirror
#   ./pixel-bridge.sh mirror       # full mirror (screen + input)
#   ./pixel-bridge.sh input        # input only (no screen)
#   ./pixel-bridge.sh cam          # camera viewfinder
#   ./pixel-bridge.sh shell        # just ADB shell

PIXEL_SERIAL="44251JEKB04957"
WIFI_ENDPOINTS=("192.168.9.203:36135" "192.168.4.170:5555" "192.168.4.249:5555")

find_pixel() {
    # USB first
    local state
    state=$(adb -s "$PIXEL_SERIAL" get-state 2>/dev/null) || true
    if [ "$state" = "device" ]; then
        echo "$PIXEL_SERIAL"
        return 0
    fi

    # WiFi ADB
    for ep in "${WIFI_ENDPOINTS[@]}"; do
        adb connect "$ep" &>/dev/null || continue
        local serial
        serial=$(adb -s "$ep" shell getprop ro.serialno 2>/dev/null) || true
        if [ "$serial" = "$PIXEL_SERIAL" ]; then
            echo "$ep"
            return 0
        fi
        adb disconnect "$ep" &>/dev/null || true
    done
    return 1
}

echo "🔍 Finding Pixel..."
DEV=$(find_pixel) || {
    echo "❌ Pixel not found on USB or WiFi ADB."
    echo "   Plug in USB-C, or enable WiFi debugging."
    echo "   Check: adb devices"
    exit 1
}
echo "✅ Pixel found: $DEV"
echo ""

MODE="${1:-mirror}"

case "$MODE" in
    mirror)
        echo "🪞 Full mirror — screen + input"
        echo "   Mouse click = touch. Keyboard = text input."
        echo "   Ctrl+O = turn phone screen off (save power)"
        echo "   Ctrl+P = power"
        echo "   Ctrl+H = home"
        echo "   Ctrl+B = back"
        echo ""
        scrcpy -s "$DEV" --stay-awake --turn-screen-off \
            --window-title "grapheneGate — Pixel 8a" \
            --shortcut-mod=lctrl
        ;;
    input)
        echo "⌨️  Input only — no video, no audio"
        echo "   Keyboard/mouse routed to Pixel, no mirror window"
        scrcpy -s "$DEV" --no-video --no-audio --stay-awake
        ;;
    cam)
        echo "📷 Camera viewfinder"
        echo "   Opening Pixel camera..."
        adb -s "$DEV" shell am start -a android.media.action.IMAGE_CAPTURE 2>/dev/null
        scrcpy -s "$DEV" --stay-awake \
            --window-title "grapheneGate — Camera" \
            --max-fps 30
        ;;
    shell)
        echo "🐚 ADB shell"
        adb -s "$DEV" shell
        ;;
    *)
        echo "Usage: $0 [mirror|input|cam|shell]"
        exit 1
        ;;
esac
