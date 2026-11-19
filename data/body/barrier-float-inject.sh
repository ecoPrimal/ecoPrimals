#!/bin/bash
# barrier-float-inject.sh — Push Barrier CLIENT into floating devices
# Runs FROM eastGate (brain). Reaches into Steam Deck, Frame, Pixel.
#
# The floating hypothesis: devices FLOAT between houses.
# We reach them via whatever path is alive: WiFi, wired, ADB, BT.
# Each device becomes a Barrier client pointing back to eastGate:24800.
#
# Usage:
#   ./barrier-float-inject.sh deck      # inject into Steam Deck
#   ./barrier-float-inject.sh frame     # inject into Steam Frame  
#   ./barrier-float-inject.sh pixel     # inject into Pixel (via ADB)
#   ./barrier-float-inject.sh all       # try all
#   ./barrier-float-inject.sh scan      # just find who's alive

set -euo pipefail

BRAIN_IP="192.168.4.244"
BARRIER_PORT=24800
BARRIER_SERVER="eastgate"

# --- Device discovery table ---
# Each device has multiple possible paths. Try them in order.
declare -A DECK_IPS=( [wired]="192.168.4.176" [wifi]="192.168.4.203" )
declare -A FRAME_IPS=( [wifi]="192.168.4.181" [usb]="10.86.200.234" )
declare -A PIXEL_IPS=( [gaia_wifi]="192.168.9.203" [lan_wifi]="192.168.4.170" [usb]="192.168.4.249" )

DECK_USER="deck"
DECK_PASS="deck"
FRAME_USER="steamos"
PIXEL_SERIAL="44251JEKB04957"

C_GREEN='\033[0;32m'
C_RED='\033[0;31m'
C_YELLOW='\033[1;33m'
C_RESET='\033[0m'

log() { echo -e "${C_GREEN}[+]${C_RESET} $*"; }
warn() { echo -e "${C_YELLOW}[!]${C_RESET} $*"; }
fail() { echo -e "${C_RED}[-]${C_RESET} $*"; }

# Find first alive IP from an associative array
find_alive() {
    local -n ips=$1
    for path in "${!ips[@]}"; do
        local ip="${ips[$path]}"
        if ping -c 1 -W 1 "$ip" &>/dev/null; then
            echo "$ip|$path"
            return 0
        fi
    done
    return 1
}

# Find Pixel via ADB (USB or WiFi)
find_pixel_adb() {
    # Check USB first
    local r
    r=$(adb -s "$PIXEL_SERIAL" get-state 2>/dev/null) || true
    if [ "$r" = "device" ]; then
        echo "$PIXEL_SERIAL|usb"
        return 0
    fi
    # Try WiFi ADB candidates
    for path in "${!PIXEL_IPS[@]}"; do
        local ip="${PIXEL_IPS[$path]}"
        local ep="${ip}:5555"
        [ "$path" = "gaia_wifi" ] && ep="${ip}:36135"
        adb connect "$ep" &>/dev/null || true
        local serial
        serial=$(adb -s "$ep" shell getprop ro.serialno 2>/dev/null) || true
        if [ "$serial" = "$PIXEL_SERIAL" ]; then
            echo "$ep|adb_$path"
            return 0
        fi
        adb disconnect "$ep" &>/dev/null || true
    done
    return 1
}

# --- SCAN ---
do_scan() {
    echo "╔════════════════════════════════════════╗"
    echo "║  🔭 Float Scan — Finding Bodies        ║"
    echo "╚════════════════════════════════════════╝"
    echo ""

    if result=$(find_alive DECK_IPS); then
        IFS='|' read -r ip path <<< "$result"
        log "deckGate ALIVE at $ip ($path)"
    else
        fail "deckGate DARK on all paths"
    fi

    if result=$(find_alive FRAME_IPS); then
        IFS='|' read -r ip path <<< "$result"
        log "frameGate ALIVE at $ip ($path)"
    else
        fail "frameGate DARK on all paths"
    fi

    if result=$(find_pixel_adb); then
        IFS='|' read -r dev path <<< "$result"
        log "grapheneGate (Pixel) ALIVE via $dev ($path)"
    else
        fail "grapheneGate (Pixel) DARK on all paths"
    fi

    echo ""
    echo "ADB devices:"
    adb devices -l 2>/dev/null
}

# --- INJECT BARRIER INTO STEAMOS (Deck or Frame) ---
inject_steamos() {
    local name="$1"
    local user="$2"
    local -n ips=$3
    local screen_name="$4"

    echo "╔════════════════════════════════════════╗"
    echo "║  🧬 Injecting Barrier → $name"
    echo "╚════════════════════════════════════════╝"

    local result ip path
    if ! result=$(find_alive ips); then
        fail "$name is DARK on all paths. Kevin needs to power it up."
        return 1
    fi
    IFS='|' read -r ip path <<< "$result"
    log "$name found at $ip ($path)"

    log "Unlocking SteamOS filesystem..."
    ssh -o StrictHostKeyChecking=no -o ConnectTimeout=5 "${user}@${ip}" \
        "sudo steamos-readonly disable 2>/dev/null || true" 2>/dev/null

    log "Installing Barrier client via pacman..."
    ssh -o StrictHostKeyChecking=no -o ConnectTimeout=5 "${user}@${ip}" \
        "sudo pacman -Sy --noconfirm barrier 2>/dev/null || echo 'pacman failed — trying flatpak...'
         command -v barrierc && echo 'barrierc installed!' || {
             flatpak install -y flathub com.github.debauchee.barrier 2>/dev/null || echo 'flatpak failed too'
         }" 2>/dev/null

    log "Creating Barrier client config..."
    ssh -o StrictHostKeyChecking=no -o ConnectTimeout=5 "${user}@${ip}" \
        "mkdir -p ~/.local/share/barrier/SSL
         cat > ~/.local/share/barrier/barrier-client.conf << 'EOF'
section: options
    heartbeat = 5000
    switchCorners = none
    screenSaverSync = true
end
EOF" 2>/dev/null

    log "Creating systemd service for Barrier client..."
    ssh -o StrictHostKeyChecking=no -o ConnectTimeout=5 "${user}@${ip}" \
        "mkdir -p ~/.config/systemd/user
         cat > ~/.config/systemd/user/barrier-client.service << SVC
[Unit]
Description=Barrier Client — ${name} → eastGate mesh
After=graphical-session.target

[Service]
Type=simple
ExecStart=/usr/bin/barrierc --no-daemon --name ${screen_name} --enable-crypto ${BRAIN_IP}:${BARRIER_PORT}
Restart=on-failure
RestartSec=10
Environment=DISPLAY=:0

[Install]
WantedBy=default.target
SVC
         systemctl --user daemon-reload
         loginctl enable-linger \$(whoami) 2>/dev/null || true
         systemctl --user enable barrier-client.service 2>/dev/null || true" 2>/dev/null

    log "Starting Barrier client on $name..."
    ssh -o StrictHostKeyChecking=no -o ConnectTimeout=5 "${user}@${ip}" \
        "systemctl --user start barrier-client.service 2>/dev/null || \
         DISPLAY=:0 nohup barrierc --no-daemon --name ${screen_name} --enable-crypto ${BRAIN_IP}:${BARRIER_PORT} &" 2>/dev/null

    log "$name Barrier client INJECTED! Screen: ${screen_name} → eastGate:${BARRIER_PORT}"
}

# --- INJECT ADB BRIDGE FOR PIXEL ---
inject_pixel() {
    echo "╔════════════════════════════════════════╗"
    echo "║  🧬 Injecting ADB Bridge → Pixel       ║"
    echo "╚════════════════════════════════════════╝"

    local result dev path
    if ! result=$(find_pixel_adb); then
        fail "Pixel is DARK on all paths. Kevin needs to connect USB or WiFi."
        return 1
    fi
    IFS='|' read -r dev path <<< "$result"
    log "Pixel found via $dev ($path)"

    log "Checking scrcpy (screen mirror + input)..."
    if command -v scrcpy &>/dev/null; then
        log "scrcpy installed — can mirror Pixel screen + send input"
        log ""
        log "To start Pixel mirror NOW:"
        log "  scrcpy -s $dev --turn-screen-off --stay-awake"
        log ""
        log "To start headless (input only, no window):"
        log "  scrcpy -s $dev --no-video --no-audio"
        log ""
        log "To start with specific window position:"
        log "  scrcpy -s $dev --window-x 1920 --window-y 0 --window-width 540 --window-height 1200"
    else
        warn "scrcpy NOT installed — using raw ADB input"
    fi

    log "Setting up ADB port forward for Barrier..."
    adb -s "$dev" reverse tcp:${BARRIER_PORT} tcp:${BARRIER_PORT} 2>/dev/null && \
        log "ADB reverse port forward: Pixel:${BARRIER_PORT} → eastGate:${BARRIER_PORT}" || \
        warn "ADB reverse port failed (might need developer options)"

    log "Pixel ADB bridge ready!"
    log "  Connection: $dev via $path"
    log "  scrcpy: screen mirror + touch input"
    log "  ADB reverse: port ${BARRIER_PORT} bridged"
}

# --- MAIN ---
case "${1:-scan}" in
    scan)
        do_scan
        ;;
    deck)
        inject_steamos "deckGate" "$DECK_USER" DECK_IPS "deckgate"
        ;;
    frame)
        inject_steamos "frameGate" "$FRAME_USER" FRAME_IPS "framegate"
        ;;
    pixel)
        inject_pixel
        ;;
    all)
        do_scan
        echo ""
        inject_steamos "deckGate" "$DECK_USER" DECK_IPS "deckgate" || true
        echo ""
        inject_steamos "frameGate" "$FRAME_USER" FRAME_IPS "framegate" || true
        echo ""
        inject_pixel || true
        ;;
    *)
        echo "Usage: $0 [scan|deck|frame|pixel|all]"
        exit 1
        ;;
esac

echo ""
echo "═══════════════════════════════════════════"
echo "  Barrier Server: eastGate:${BARRIER_PORT}"
echo "  Mesh: 7 gates + 3 floaters"
echo "  Kevin's 1 keyboard → ALL screens"
echo "═══════════════════════════════════════════"
