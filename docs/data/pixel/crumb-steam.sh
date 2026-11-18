#!/bin/bash
# 🎮 STEAMOS BREADCRUMB — for swiftGate, deckGate, frameGate
echo "🎮 SteamOS gate — following crumbs home..."

# SteamOS default user is 'deck' — set a password first!
echo "Setting deck password to 'swiftgate'..."
echo "deck:swiftgate" | sudo chpasswd

# Enable SSH
sudo systemctl enable sshd
sudo systemctl start sshd

# Check WiFi
echo "--- WiFi networks ---"
nmcli dev wifi list 2>/dev/null | head -10

# Connect to Aperture Science
echo "--- Connecting to Aperture Science ---"
nmcli dev wifi connect "Aperture Science" password "exoticpanda714" 2>/dev/null

# Fallback — try ATT modem
nmcli dev wifi connect "ATTypcbDbY" password "exoticpanda714" 2>/dev/null

# Get IP
echo "--- IP ---"
ip addr show | grep "inet " | grep -v 127

echo "🎮 SteamOS gate ready! SSH: deck@<ip> password: swiftgate"
