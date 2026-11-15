# inkFish — Product Specification

**Version:** 0.1.0
**Date:** Nov 15, 2026
**Status:** Architecture documented, hardware proven, identity model defined

---

## What Is inkFish?

inkFish is a sovereign Steam/VR gaming sovereignty layer. Like a squid
releases ink to mark territory and communicate, inkFish is how Gaia marks
her presence in the Steam ecosystem.

**Not a spring.** A standalone sporeGarden product that IS the primals
working together as a Steam ecosystem integration system. Exercises
ludoSpring's game science via real hardware and manages the HOLDER/PLAYER
identity architecture.

## Naming

| Garden | Spring | Domain | Metaphor |
|--------|--------|--------|----------|
| blueFish | wetSpring | PFAS chemistry | Blue = water/PFAS |
| **inkFish** | **ludoSpring** | Steam/VR gaming | Ink = mark/publish/identity |

## Core Domains

### 1. Identity (`identity`)
- HOLDER/PLAYER model
- Steam account management (TamisonEcoPrimal, gw_kevinmok, MokVive)
- Steam Guard bridge (Pixel authenticator)
- Family Sharing composition
- Session attribution (who played what when)

### 2. Hardware (`hardware`)
- Steam Frame management (XR, passthrough, eye tracking)
- Steam Deck management (CEF control, display)
- Steam Link / Remote Caster (tail beacon)
- Valve peripherals (WiFi dongle, controllers)
- ADB bridge for non-Steam devices (Pixel as VR eye)

### 3. Streaming (`streaming`)
- Steam Remote Play coordination
- vrlink UDP streaming
- Pixel camera → Frame VR pipeline
- gamescope/compositor management
- Cross-house streaming (10G spine)

### 4. Spatial (`spatial`)
- Flint 1/2 RSSI triangulation
- Device location awareness
- House 1 ↔ House 2 topology
- WiFi signal field mapping
- Head tracking (Pixel IMU → Frame VR)

### 5. Metrics (`metrics`)
- play_time: human metric (hours played per account)
- hours_active: agent metric (hardware usage time)
- session_log: WHO on WHAT device for HOW LONG
- engagement: ludoSpring flow/engagement science applied to real data
- VR_time: passthrough vs immersive vs idle

### 6. Library (`library`)
- Family Sharing composition (3 accounts → 1 merged library)
- VR game inventory (MokVive lineage + current)
- Game compatibility matrix (ARM64 native vs Proton vs streaming)
- Workshop content management

## Hardware Map

```
┌─ HOUSE 1 (Brain Side) ──────────────────────┐
│  eastGate (.244) — BRAIN, RTX 4070           │
│  northGate (.147) — HEART, RTX 5090          │
│  deckGate (.176) — DISPLAY, dead battery     │
│  sporeGate (.3) — FORGE, DNS                 │
│  Left Ear (Flint 1) — RSSI at House 1        │
│  Valve WiFi Dongle — on eastGate USB          │
└──────────────────────────────────────────────┘
        │ 10G fiber spine (0.87ms) │
┌─ HOUSE 2 (South Pole) ──────────────────────┐
│  grapheneGate (.202) — LEFT EYE, camera       │
│    └─ Steam Guard (TamisonEcoPrimal)          │
│  frameGate (.181) — FACE, VR/AR headset       │
│  Right Ear (Flint 2) — RSSI at House 2        │
│  Steam Link — TAIL beacon                     │
└──────────────────────────────────────────────┘
```

## Composition Model

inkFish composes primals via `inkfish_cell.toml`:
- **squirrel** — MCP agent dispatch (ADB, CEF, SSH commands)
- **nestGate** — data gateway (Steam API, sensor data)
- **petalTongue** — VR UI rendering (HUD, spatial minimap)
- **sweetGrass** — knowledge graph (game library, device topology)
- **bearDog** — identity (HOLDER/PLAYER crypto signing)
- **toadStool** — service discovery (Steam broadcast, device scan)
- **swarmVine** — gossip (device state propagation)
- **songBird** — transport (vrlink, Remote Play coordination)
- **barraCuda** — compute (VR math, engagement calculations)
- **biomeOS** — orchestration (hardware lifecycle management)
