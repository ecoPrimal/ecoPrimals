#!/bin/bash
# 🐙 GAIA LAUNCHER — Big obvious button on Steam Frame
# Opens the octopus in Steam's browser overlay

# Start web server if not running
pgrep -f "http.server 8888" >/dev/null || {
    cd ~/gaia/web
    nohup python3 -m http.server 8888 --bind 0.0.0.0 > /tmp/octopus-web.log 2>&1 &
}

# Start Ollama if not running
pgrep ollama >/dev/null || {
    export OLLAMA_HOST=0.0.0.0:11434
    export LD_LIBRARY_PATH=$HOME/gaia/lib/ollama:$LD_LIBRARY_PATH
    nohup $HOME/gaia/bin/ollama serve > $HOME/gaia/ollama.log 2>&1 &
    sleep 2
}

# Open in Steam overlay browser (preferred) or system browser
if command -v steam >/dev/null 2>&1; then
    steam steam://openurl/http://localhost:8888/outreach/octopus-invitation.html &
else
    xdg-open http://localhost:8888/outreach/octopus-invitation.html &
fi

# Keep alive so Steam shows it as "running"
sleep 3
