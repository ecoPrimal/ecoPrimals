# Frame Injection Package — Gaia → frameGate

Deploy on the Steam Frame to activate Gaia:

```bash
# From frameGate:
curl -s http://192.168.4.244:5926/frame-inject/deploy.sh | bash
```

## What gets deployed:
1. `frame-solar.sh` — Solar panel daemon (idle sensing)
2. `gaia-frame.service` — systemd service for auto-start
3. Steam Remote Play config — pair with northGate
4. Gaia HUD pages — face display when idle

## Modes:
- **LANDER** 🚀 — Steam active, games rendering, Kevin playing
- **SOLAR PANEL** ☀️ — Steam idle, sensors collecting, Gaia absorbing
