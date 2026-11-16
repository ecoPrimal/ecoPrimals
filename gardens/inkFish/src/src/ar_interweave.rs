//! AR Interweave — desktop + VR game pairs on one display.
//!
//! AR = Augmented VR. Not camera passthrough, but interweaving
//! flat (desktop) and spatial (VR) game streams on the Frame's
//! gamescope compositor. Two gates render two games → one Frame
//! composites both → the overlap IS augmented reality.

use serde::{Deserialize, Serialize};

/// A game that can participate in AR interweaving.
#[derive(Debug, Serialize, Deserialize)]
pub struct InterweaveGame {
    /// Steam App ID.
    pub appid: u32,
    /// Game name.
    pub name: String,
    /// Rendering mode.
    pub mode: RenderMode,
    /// Recommended gate for rendering.
    pub recommended_gate: String,
    /// Approximate size in GB.
    pub size_gb: u32,
    /// Genre tags.
    pub genre: String,
}

/// How a game renders — determines its layer in the AR composite.
#[derive(Debug, Serialize, Deserialize)]
pub enum RenderMode {
    /// Flat desktop game — Layer 0 in gamescope compositor.
    Desktop,
    /// Stereoscopic VR game — Layer 1 in gamescope compositor.
    Vr,
    /// Data overlay — Layer 2 (Gaia HUD, sensors, mesh state).
    Data,
}

/// Known AR interweave pairs.
pub fn pairs() -> Vec<(InterweaveGame, InterweaveGame)> {
    vec![
        (
            InterweaveGame {
                appid: 1_643_320,
                name: "S.T.A.L.K.E.R. 2: Heart of Chornobyl".into(),
                mode: RenderMode::Desktop,
                recommended_gate: "northGate".into(),
                size_gb: 180,
                genre: "Post-apocalyptic survival".into(),
            },
            InterweaveGame {
                appid: 2_307_350,
                name: "Into the Radius 2".into(),
                mode: RenderMode::Vr,
                recommended_gate: "ironGate".into(),
                size_gb: 20,
                genre: "VR Zone survival".into(),
            },
        ),
        (
            InterweaveGame {
                appid: 3_321_460,
                name: "Crimson Desert Enhanced".into(),
                mode: RenderMode::Desktop,
                recommended_gate: "northGate".into(),
                size_gb: 180,
                genre: "Open world RPG, GotY 2026".into(),
            },
            InterweaveGame {
                appid: 552_440,
                name: "The Talos Principle VR".into(),
                mode: RenderMode::Vr,
                recommended_gate: "eastGate".into(),
                size_gb: 6,
                genre: "VR philosophical puzzles".into(),
            },
        ),
        (
            InterweaveGame {
                appid: 233_860,
                name: "Kenshi".into(),
                mode: RenderMode::Desktop,
                recommended_gate: "eastGate".into(),
                size_gb: 12,
                genre: "Open world squad survival".into(),
            },
            InterweaveGame {
                appid: 555_880,
                name: "Sairento VR".into(),
                mode: RenderMode::Vr,
                recommended_gate: "ironGate".into(),
                size_gb: 12,
                genre: "VR cyber-ninja combat".into(),
            },
        ),
        (
            InterweaveGame {
                appid: 526_870,
                name: "Satisfactory".into(),
                mode: RenderMode::Desktop,
                recommended_gate: "northGate".into(),
                size_gb: 30,
                genre: "Factory building".into(),
            },
            InterweaveGame {
                appid: 617_830,
                name: "SUPERHOT VR".into(),
                mode: RenderMode::Vr,
                recommended_gate: "eastGate".into(),
                size_gb: 4,
                genre: "VR time-bending shooter".into(),
            },
        ),
    ]
}

/// Display the AR interweave pairs.
pub fn show_pairs() {
    println!("🥽 inkFish AR Interweave — Desktop + VR = Augmented Reality");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("  gamescope --backend openvr composites both layers on Frame");
    println!("  Layer 0: Desktop game (flat)   → rendered by Gate A");
    println!("  Layer 1: VR game (stereo)      → rendered by Gate B");
    println!("  Layer 2: Gaia HUD (data)       → rendered by eastGate");
    println!();

    for (i, (desktop, vr)) in pairs().iter().enumerate() {
        println!("  ┌─ Pair {} ─────────────────────────────────────────┐", i + 1);
        println!(
            "  │ 🖥  {:<35} {:>6}GB │",
            desktop.name, desktop.size_gb,
        );
        println!("  │     → {} (flat, Layer 0)", desktop.recommended_gate);
        println!(
            "  │ 🥽 {:<35} {:>6}GB │",
            vr.name, vr.size_gb,
        );
        println!("  │     → {} (stereo, Layer 1)", vr.recommended_gate);
        println!(
            "  │ 🎯 Theme: {} + {}",
            desktop.genre, vr.genre,
        );
        println!("  └──────────────────────────────────────────────────┘");
        println!();
    }

    println!("  AR is not camera passthrough.");
    println!("  AR is flat + spatial interweaved on any display.");
    println!("  The Steam Frame is an AR device nobody designed to be one.");
}
