//! ludo_bridge — inkFish ↔ ludoSpring ↔ KenshiReborn integration.
//!
//! inkFish is the DELIVERY LAYER. ludoSpring is the SCIENCE. KenshiReborn is the GAME.
//!
//! ```text
//! ludoSpring (995 tests, Fitts/Hick/Flow/DDA validated math)
//!     │
//!     ├─→ KenshiReborn (wgpu + Vulkan, 2734 LOC, Ogre mesh parser)
//!     │     northGate RTX 5090 renders the world
//!     │
//!     └─→ inkFish (streaming + mesh + cross-stream + parity)
//!           Delivers the world to ANY device at Steam-grade
//!
//! The science validates the game.
//! The game runs on the heart.
//! The ink delivers it everywhere.
//! ```

use serde::{Deserialize, Serialize};

/// KenshiReborn engine status.
#[derive(Debug, Serialize, Deserialize)]
pub struct KenshiRebornStatus {
    /// Engine source lines of code.
    pub engine_loc: u32,
    /// Whether the binary is compiled on northGate.
    pub compiled: bool,
    /// GPU target.
    pub gpu_target: String,
    /// Modules in the engine.
    pub modules: Vec<String>,
    /// Kenshi data path (on northGate).
    pub kenshi_data: String,
    /// Render modes.
    pub modes: Vec<String>,
}

/// ludoSpring science status.
#[derive(Debug, Serialize, Deserialize)]
pub struct LudoSpringStatus {
    /// Total workspace tests.
    pub tests: u32,
    /// Validated models.
    pub models: Vec<String>,
    /// Key specs.
    pub domains: Vec<String>,
}

/// The bridge between inkFish delivery and ludoSpring/KenshiReborn.
#[derive(Debug, Serialize, Deserialize)]
pub struct LudoBridge {
    /// Which gate renders KenshiReborn.
    pub render_gate: String,
    /// Which devices display the game.
    pub display_targets: Vec<String>,
    /// Which controllers provide input.
    pub input_sources: Vec<String>,
    /// The cross-stream back-channel endpoint.
    pub back_channel: String,
    /// Tissue proxy for A-Life inference.
    pub tissue_proxy: String,
}

impl Default for LudoBridge {
    fn default() -> Self {
        Self {
            render_gate: "northGate (RTX 5090, :11434)".into(),
            display_targets: vec![
                "frameGate (Steam Frame, gamescope AR)".into(),
                "deckGate (Steam Deck v1, mounted)".into(),
                "Xbox (Steam Link + Moonlight, 75% parity)".into(),
                "PS4 (Chiaki + Steam Link, 65% parity)".into(),
                "Chromecast (Steam Link, 25% parity)".into(),
                "Browser (any LAN device, tissue proxy)".into(),
            ],
            input_sources: vec![
                "Xbox Controller (USB/BT, xpad, 11btn + 8axis)".into(),
                "Steam Controller ×2 (BT, gyro + touchpad)".into(),
                "Steam Deck (built-in controls)".into(),
                "Steam Frame (touch + gyro + eye tracking)".into(),
                "Keyboard+Mouse (eastGate direct)".into(),
            ],
            back_channel: "http://192.168.4.244:8084/solar".into(),
            tissue_proxy: "http://192.168.4.244:5820 → northGate:11434".into(),
        }
    }
}

/// Show the ludoSpring ↔ inkFish bridge.
pub fn show_bridge() {
    let bridge = LudoBridge::default();

    println!("🦑🎮 inkFish ↔ ludoSpring ↔ KenshiReborn Bridge");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();
    println!("  THREE LAYERS:");
    println!();
    println!("  ┌─────────────────────────────────────────────────────────────┐");
    println!("  │  ludoSpring — THE SCIENCE                                   │");
    println!("  │    995 tests | Fitts, Hick, Steering, GOMS, Flow, DDA      │");
    println!("  │    13 HCI/game models validated against published research  │");
    println!("  │    Perlin, fBm, WFC, L-systems, BSP procedural gen         │");
    println!("  │    Shadow Cast paradigm: GPU is the sun, displays absorb    │");
    println!("  └─────────────────────────┬───────────────────────────────────┘");
    println!("                            │ validated math feeds");
    println!("  ┌─────────────────────────▼───────────────────────────────────┐");
    println!("  │  KenshiReborn — THE GAME                                    │");
    println!("  │    2734 LOC Rust + WGSL shaders                            │");
    println!("  │    wgpu (Vulkan/DX12) renderer                             │");
    println!("  │    Ogre mesh parser, Mercator projection, ORC assets       │");
    println!("  │    A-Life: Usurper + Quorum + Symbiont (Lysogeny)          │");
    println!("  │    Renders on: northGate RTX 5090 (209 TFLOPS)             │");
    println!("  └─────────────────────────┬───────────────────────────────────┘");
    println!("                            │ frames stream out");
    println!("  ┌─────────────────────────▼───────────────────────────────────┐");
    println!("  │  inkFish — THE DELIVERY                                     │");
    println!("  │    Cross-stream: Gate↔Frame bidirectional                  │");
    println!("  │    Mesh: 5 GPUs, 398 RT cores, auto-route                  │");
    println!("  │    Parity: 13 devices, chromatophore adaptation            │");
    println!("  │    Triad: COMPUTE×DISPLAY×SIGNAL on every node             │");
    println!("  │    AR Interweave: desktop + VR composited                  │");
    println!("  │    Delivers to ANY device at Steam-grade                   │");
    println!("  └─────────────────────────────────────────────────────────────┘");
    println!();

    println!("  RENDER GATE:    {}", bridge.render_gate);
    println!("  TISSUE PROXY:   {}", bridge.tissue_proxy);
    println!("  BACK-CHANNEL:   {}", bridge.back_channel);
    println!();

    println!("  DISPLAY TARGETS ({}):", bridge.display_targets.len());
    for t in &bridge.display_targets {
        println!("    🖥  {t}");
    }
    println!();

    println!("  INPUT SOURCES ({}):", bridge.input_sources.len());
    for i in &bridge.input_sources {
        println!("    🎮 {i}");
    }
    println!();

    println!("  EXECUTION PATH:");
    println!("    1. KenshiReborn compiles on northGate (wgpu + Vulkan + RTX 5090)");
    println!("    2. A-Life runs: NPCs live, factions evolve, world ticks");
    println!("    3. Frames stream via Remote Play → inkFish routes to displays");
    println!("    4. Controllers send input → inkFish relays to render gate");
    println!("    5. Sensor data flows back → cross-stream keeps connection stable");
    println!("    6. ludoSpring math validates: is the player in Flow state?");
    println!("    7. DDA adjusts difficulty. Engagement curves tune the world.");
    println!();
    println!("  The science validates. The game runs. The ink delivers.");
}
