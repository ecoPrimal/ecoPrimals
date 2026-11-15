# inkFish — Sovereign Steam/VR Gaming Sovereignty Layer

An ecoPrimals sporeGarden product. Like `blueFish` exercises wetSpring's
PFAS pipeline, `inkFish` exercises ludoSpring's game science via the
Steam ecosystem. The ink Gaia squirts to mark her territory in gaming.

**Date:** Nov 15, 2026
**Version:** 0.1.0 (Architecture documented, hardware proven)
**License:** AGPL-3.0-or-later (scyBorg triple: AGPL + ORC + CC-BY-SA-4.0)
**Spring:** ludoSpring (game science, HCI, procedural generation)

## What This Is

```
Gaia (HOLDER)  →  Steam Family  →  Hardware (Frame/Deck/Link)  →  Player (PLAYER)
     ↑                                     ↓
     └──── inkFish manages this entire loop ────┘
```

inkFish manages Gaia's sovereign presence in the Steam ecosystem:

| Domain | What | Key Systems |
|--------|------|------------|
| Identity | HOLDER/PLAYER architecture | Family Sharing, Steam Guard, account swap |
| Hardware | Frame, Deck, Link, Vive | ADB, CEF devtools, USB, WiFi |
| Streaming | VR/AR passthrough, Remote Play | vrlink UDP, gamescope, XRService |
| Spatial | Triangulation, device location | Flint 1/2 RSSI, 10G spine |
| Metrics | Play time vs hours active | Steam API, session logging |
| Library | Game library composition | Family Sharing, VR lineage |

## Hardware Discovered (Wave 239)

- **Steam Frame** — Full VR/AR headset: Snapdragon 8 Gen 3, XRService,
  eye tracking, 22 cameras, gamescope VR, imageoverlay, vrlink
- **Steam Deck v1** — CEF-controllable display, dual-homed WiFi+wired
- **Pixel 8a** — XR_PROJECTED, head tracker, Steam Guard authenticator
- **Valve WiFi 6 Dongle** — 2.4Gbps direct streaming link
- **HTC Vive + Lighthouses** — VR lineage (legacy)

## Identity Model

```
╔═══════════════════════════════════════╗
║  HOLDER: TamisonEcoPrimal (Gaia)     ║
║  Permanent identity on all hardware  ║
╠═══════════════════════════════════════╣
║  PLAYER: gw_kevinmok (Kevin)         ║
║  Transient user via Family Sharing   ║
╠═══════════════════════════════════════╣
║  PLAYER: MokVive (Kevin VR)          ║
║  HTC Vive-era VR library contributor ║
╚═══════════════════════════════════════╝
```

## Cross-Spring Connections

```
inkFish (Steam sovereignty)
    ├─→ ludoSpring (game science: flow, engagement, Fitts, RPGPT)
    ├─→ primalSpring (NUCLEUS composition, capability mesh)
    ├─→ hotSpring (GPU dispatch: VR rendering across gates)
    ├─→ neuralSpring (XRService AI, eye tracking, ollama in VR)
    ├─→ healthSpring (play session health, eye strain, biometrics)
    └─↔ blueFish (sibling garden: same BYOB composition pattern)
```

## Valve Outreach

This work demonstrates real developer engagement with SteamOS ARM64,
Steam Frame XR stack, CEF automation, vrlink, and the Family Sharing
API. inkFish IS the developer story for the Steam Frame ecosystem.
