//! # inkFish — Sovereign Steam/VR Gaming Sovereignty Layer
//!
//! Like `blueFish` exercises wetSpring's PFAS pipeline via BYOB composition,
//! `inkFish` exercises ludoSpring's game science via the Steam ecosystem.
//!
//! ## Architecture: HOLDER and PLAYER
//!
//! - **HOLDER** (Gaia/TamisonEcoPrimal): Persistent identity on all Steam hardware.
//!   The organism manages the infrastructure. Hours active = agent metric.
//!
//! - **PLAYER** (Humans): Transient users of Gaia's hardware.
//!   Play time on their account = human metric. Always just players.
//!
//! ## Domains
//!
//! | Module | What it manages | Key systems |
//! |--------|----------------|-------------|
//! | `identity` | HOLDER/PLAYER accounts, Steam Guard | Family Sharing, account swap |
//! | `hardware` | Frame, Deck, Link, Vive management | ADB, CEF, USB, WiFi |
//! | `streaming` | VR/AR passthrough, Remote Play | vrlink UDP, gamescope, XRService |
//! | `spatial` | Left/Right ear triangulation | RSSI, Flint APs, 10G spine |
//! | `metrics` | Play time (human) vs hours active (agent) | Steam API, Family Sharing |
//! | `library` | Game library composition across family | Family Sharing, VR lineage |
//!
//! ## Steam Family
//!
//! Three accounts in one family:
//! - `gw_kevinmok` (Tamison) — Kevin's personal library (PLAYER)
//! - `TamisonEcoPrimal` (gaia) — Gaia's sovereign account (HOLDER)
//! - `MokVive` — HTC Vive-era VR library (PLAYER)
//!
//! ## Hardware (Gaia's Gaming Body)
//!
//! - Steam Frame (Snapdragon 8 Gen 3) — VR/AR headset, eye tracking, XR passthrough
//! - Steam Deck v1 — Mounted display, CEF devtools accessible
//! - Steam Link / Valve Remote Caster — Tail beacon, network relay
//! - Valve WiFi 6 Dongle (28de:2432) — 2.4Gbps direct streaming
//! - HTC Vive + Lighthouses — VR lineage (legacy, somewhere in storage)
//! - Pixel 8a (grapheneGate) — Remote eye with XR_PROJECTED + head tracker
//!
//! ## Cross-Spring Connections
//!
//! ```text
//! inkFish (Steam sovereignty, VR/AR, hardware management)
//!     │
//!     ├─→ ludoSpring (game science: Fitts, Hick, flow, engagement, RPGPT)
//!     ├─→ primalSpring (composition: NUCLEUS cells, capability mesh)
//!     ├─→ hotSpring (compute: GPU dispatch for VR rendering)
//!     ├─→ neuralSpring (AI: XRService perception, eye tracking ML)
//!     ├─→ healthSpring (biometrics: play session health, eye strain)
//!     └─→ blueFish (sibling garden: same BYOB composition pattern)
//! ```

fn main() {
    println!("inkFish v0.1.0 — sovereign Steam/VR gaming sovereignty layer");
    println!("HOLDER: TamisonEcoPrimal (Gaia)");
    println!("PLAYER: gw_kevinmok (Tamison/Kevin)");
    println!("Status: Architecture documented, hardware mapped, VR stack live");
}
