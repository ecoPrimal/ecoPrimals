//! Parity module — bring ANY device to Steam-grade.
//!
//! Steam/SteamOS is the master craft. 100% parity = native SteamOS.
//! Everything else gets a parity score based on what it CAN do,
//! then inkFish applies chromatophore adaptations to raise that score.
//!
//! ## The Chromatophore Pattern
//!
//! An octopus doesn't make all its skin identical — it makes each cell
//! express the RIGHT pattern for its position. inkFish does the same:
//! each device gets the protocol translation it needs to reach parity.

use serde::{Deserialize, Serialize};

/// Steam parity score (0-100%).
/// 100 = native SteamOS, full Steam library, all features.
/// 0 = no gaming capability at all.
type ParityScore = u8;

/// A device in the gaming mesh with its parity assessment.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParityDevice {
    /// Device name.
    pub name: String,
    /// IP on LAN (if known).
    pub ip: Option<String>,
    /// Native OS.
    pub os: DeviceOs,
    /// Hardware class.
    pub hw_class: HardwareClass,
    /// Base parity score (before chromatophore adaptation).
    pub base_parity: ParityScore,
    /// Parity score after chromatophore adaptation.
    pub adapted_parity: ParityScore,
    /// Which chromatophore adaptations are applied.
    pub adaptations: Vec<Chromatophore>,
    /// Current status.
    pub status: DeviceStatus,
}

/// Operating system family.
#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceOs {
    /// Valve SteamOS (Deck, Frame, Link).
    SteamOs,
    /// Linux with Steam installed.
    Linux,
    /// Xbox OS (Series X/S, One).
    Xbox,
    /// PlayStation Orbis OS (PS4/PS5).
    PlayStation,
    /// Android (Pixel, phones).
    Android,
    /// iOS (iPhone, iPad).
    Ios,
    /// Cast OS (Chromecast, smart TVs).
    CastOs,
    /// Windows.
    Windows,
    /// RISC-V Linux (milkGate).
    RiscV,
}

/// Hardware classification for gaming.
#[derive(Debug, Serialize, Deserialize)]
pub enum HardwareClass {
    /// Dedicated GPU with RT cores (RTX series).
    DedicatedGpu { rt_cores: u32, vram_gb: u32 },
    /// Integrated/mobile GPU.
    MobileGpu { name: String },
    /// Console APU (Xbox, PlayStation).
    ConsoleApu { name: String, tflops: f32 },
    /// Display-only (Chromecast, Steam Link).
    DisplayOnly,
    /// CPU-only (no meaningful GPU for gaming).
    CpuOnly,
}

/// Chromatophore — a protocol adaptation that raises parity score.
/// Named after octopus chromatophore cells that change color.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Chromatophore {
    /// Steam Remote Play — stream games from a Steam host.
    SteamRemotePlay,
    /// Sunshine/Moonlight — open-source game streaming (works on Xbox!).
    SunshineMoonlight,
    /// Steam Link app — available on Xbox, PS, Android, iOS, Cast.
    SteamLinkApp,
    /// Chiaki — open-source PS Remote Play client.
    Chiaki,
    /// Xbox Cloud Gaming (xCloud) — stream Game Pass games.
    XboxCloudGaming,
    /// Proton/Wine — run Windows games on Linux.
    Proton,
    /// Greenlight — Xbox streaming to PC/mobile.
    Greenlight,
    /// PS Remote Play — stream from PS4/PS5 to other devices.
    PsRemotePlay,
    /// ADB bridge — control Android devices from the mesh.
    AdbBridge,
    /// CEF DevTools — inject into Steam's embedded browser.
    CefDevtools,
    /// Cross-stream back-channel — sensor data flowing back.
    CrossStreamBackChannel,
    /// Solar panel inversion — idle device becomes sensor.
    SolarPanelInversion,
    /// gamescope compositor — AR interweave layer.
    GamescopeAr,
}

impl Chromatophore {
    /// How many parity points this adaptation adds.
    #[must_use]
    pub const fn parity_boost(&self) -> u8 {
        match self {
            Self::SteamRemotePlay => 25,
            Self::SunshineMoonlight => 20,
            Self::SteamLinkApp => 15,
            Self::Chiaki => 10,
            Self::XboxCloudGaming => 10,
            Self::Proton => 15,
            Self::Greenlight => 10,
            Self::PsRemotePlay => 10,
            Self::AdbBridge => 5,
            Self::CefDevtools => 5,
            Self::CrossStreamBackChannel => 10,
            Self::SolarPanelInversion => 5,
            Self::GamescopeAr => 10,
        }
    }

    /// Human-readable description.
    #[must_use]
    pub const fn description(&self) -> &str {
        match self {
            Self::SteamRemotePlay => "Stream games from any Steam host on LAN",
            Self::SunshineMoonlight => "Open-source streaming (works on Xbox!)",
            Self::SteamLinkApp => "Official Steam Link receiver app",
            Self::Chiaki => "Open-source PS Remote Play (stream FROM PS4/PS5)",
            Self::XboxCloudGaming => "Stream Game Pass titles from cloud",
            Self::Proton => "Run Windows games natively on Linux",
            Self::Greenlight => "Stream Xbox to PC/mobile",
            Self::PsRemotePlay => "Stream from PS4/PS5 to any screen",
            Self::AdbBridge => "Control device from organism mesh",
            Self::CefDevtools => "Inject into Steam's embedded browser",
            Self::CrossStreamBackChannel => "Sensor data flows back to organism",
            Self::SolarPanelInversion => "Idle device becomes environmental sensor",
            Self::GamescopeAr => "AR compositor — desktop+VR interweave",
        }
    }
}

/// Device status.
#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceStatus {
    /// On the network, responding.
    Alive,
    /// Known but powered off.
    Down,
    /// Not yet connected to the mesh.
    NotOnNetwork,
    /// Active gaming session.
    Gaming,
}

/// All gaming devices in the organism.
pub fn all_devices() -> Vec<ParityDevice> {
    vec![
        // === NATIVE STEAMOS (100% base) ===
        ParityDevice {
            name: "frameGate (Steam Frame)".into(),
            ip: Some("192.168.4.181".into()),
            os: DeviceOs::SteamOs,
            hw_class: HardwareClass::MobileGpu { name: "Adreno 750".into() },
            base_parity: 100,
            adapted_parity: 100,
            adaptations: vec![
                Chromatophore::SteamRemotePlay,
                Chromatophore::CrossStreamBackChannel,
                Chromatophore::SolarPanelInversion,
                Chromatophore::GamescopeAr,
                Chromatophore::CefDevtools,
            ],
            status: DeviceStatus::Gaming,
        },
        ParityDevice {
            name: "deckGate (Steam Deck v1)".into(),
            ip: Some("192.168.4.176".into()),
            os: DeviceOs::SteamOs,
            hw_class: HardwareClass::MobileGpu { name: "RDNA 2 (Aerith)".into() },
            base_parity: 100,
            adapted_parity: 100,
            adaptations: vec![
                Chromatophore::SteamRemotePlay,
                Chromatophore::CefDevtools,
            ],
            status: DeviceStatus::Alive,
        },

        // === LINUX + STEAM (90% base — native Steam, Proton) ===
        ParityDevice {
            name: "northGate (RTX 5090)".into(),
            ip: Some("192.168.4.147".into()),
            os: DeviceOs::Linux,
            hw_class: HardwareClass::DedicatedGpu { rt_cores: 192, vram_gb: 32 },
            base_parity: 90,
            adapted_parity: 100,
            adaptations: vec![
                Chromatophore::Proton,
                Chromatophore::SteamRemotePlay,
                Chromatophore::SunshineMoonlight,
            ],
            status: DeviceStatus::Alive,
        },
        ParityDevice {
            name: "ironGate (RTX 5070)".into(),
            ip: Some("192.168.4.237".into()),
            os: DeviceOs::Linux,
            hw_class: HardwareClass::DedicatedGpu { rt_cores: 70, vram_gb: 12 },
            base_parity: 90,
            adapted_parity: 100,
            adaptations: vec![
                Chromatophore::Proton,
                Chromatophore::SteamRemotePlay,
                Chromatophore::SunshineMoonlight,
            ],
            status: DeviceStatus::Alive,
        },
        ParityDevice {
            name: "eastGate (RTX 4070)".into(),
            ip: Some("192.168.4.244".into()),
            os: DeviceOs::Linux,
            hw_class: HardwareClass::DedicatedGpu { rt_cores: 46, vram_gb: 12 },
            base_parity: 90,
            adapted_parity: 100,
            adaptations: vec![
                Chromatophore::Proton,
                Chromatophore::SteamRemotePlay,
                Chromatophore::SunshineMoonlight,
                Chromatophore::CrossStreamBackChannel,
            ],
            status: DeviceStatus::Alive,
        },
        ParityDevice {
            name: "southGate (RTX 4060)".into(),
            ip: Some("192.168.4.148".into()),
            os: DeviceOs::Linux,
            hw_class: HardwareClass::DedicatedGpu { rt_cores: 34, vram_gb: 8 },
            base_parity: 90,
            adapted_parity: 100,
            adaptations: vec![
                Chromatophore::Proton,
                Chromatophore::SteamRemotePlay,
                Chromatophore::SunshineMoonlight,
            ],
            status: DeviceStatus::Alive,
        },
        ParityDevice {
            name: "swiftGate (RTX 3070)".into(),
            ip: Some("192.168.4.171".into()),
            os: DeviceOs::Linux,
            hw_class: HardwareClass::DedicatedGpu { rt_cores: 46, vram_gb: 8 },
            base_parity: 90,
            adapted_parity: 100,
            adaptations: vec![
                Chromatophore::Proton,
                Chromatophore::SteamRemotePlay,
            ],
            status: DeviceStatus::Down,
        },

        // === XBOX (30% base → 75% with chromatophores) ===
        ParityDevice {
            name: "Xbox Series (Dasean's — DO NOT TOUCH identity)".into(),
            ip: Some("192.168.4.114".into()),
            os: DeviceOs::Xbox,
            hw_class: HardwareClass::ConsoleApu {
                name: "Xbox Custom RDNA 2".into(),
                tflops: 12.0,
            },
            base_parity: 30,
            adapted_parity: 75,
            adaptations: vec![
                Chromatophore::SteamLinkApp,        // +15 — Steam Link on Xbox Store
                Chromatophore::SunshineMoonlight,   // +20 — Moonlight on Xbox (sideload)
                Chromatophore::XboxCloudGaming,     // +10 — native xCloud
            ],
            status: DeviceStatus::Down,
        },

        // === PS4 (20% base → 65% with chromatophores) ===
        ParityDevice {
            name: "PS4 (Kevin's — needs power + ethernet)".into(),
            ip: None,
            os: DeviceOs::PlayStation,
            hw_class: HardwareClass::ConsoleApu {
                name: "PS4 Liverpool APU (GCN)".into(),
                tflops: 1.84,
            },
            base_parity: 20,
            adapted_parity: 65,
            adaptations: vec![
                Chromatophore::PsRemotePlay,       // +10 — stream FROM PS4 to any screen
                Chromatophore::Chiaki,             // +10 — open-source PS RP client on Linux
                Chromatophore::SteamLinkApp,       // +15 — Steam Link on PS Store
                Chromatophore::SunshineMoonlight,  // +20 — Moonlight (if jailbroken/homebrew)
            ],
            status: DeviceStatus::NotOnNetwork,
        },

        // === DISPLAY DEVICES ===
        ParityDevice {
            name: "Chromecast (.169)".into(),
            ip: Some("192.168.4.169".into()),
            os: DeviceOs::CastOs,
            hw_class: HardwareClass::DisplayOnly,
            base_parity: 5,
            adapted_parity: 25,
            adaptations: vec![
                Chromatophore::SteamLinkApp,       // +15 — Steam Link on Chromecast
            ],
            status: DeviceStatus::Alive,
        },

        // === MOBILE ===
        ParityDevice {
            name: "grapheneGate (Pixel 8a)".into(),
            ip: Some("192.168.4.202".into()),
            os: DeviceOs::Android,
            hw_class: HardwareClass::MobileGpu { name: "Mali-G715".into() },
            base_parity: 15,
            adapted_parity: 50,
            adaptations: vec![
                Chromatophore::SteamLinkApp,       // +15
                Chromatophore::AdbBridge,          // +5
                Chromatophore::CrossStreamBackChannel, // +10
                Chromatophore::SolarPanelInversion,    // +5
            ],
            status: DeviceStatus::Alive,
        },
        ParityDevice {
            name: "iPhone (.202)".into(),
            ip: Some("192.168.4.202".into()),
            os: DeviceOs::Ios,
            hw_class: HardwareClass::MobileGpu { name: "Apple GPU".into() },
            base_parity: 10,
            adapted_parity: 35,
            adaptations: vec![
                Chromatophore::SteamLinkApp,       // +15
                Chromatophore::PsRemotePlay,       // +10
            ],
            status: DeviceStatus::Alive,
        },

        // === RISC-V ===
        ParityDevice {
            name: "milkGate (Milk-V Jupiter 2)".into(),
            ip: Some("192.168.4.144".into()),
            os: DeviceOs::RiscV,
            hw_class: HardwareClass::CpuOnly,
            base_parity: 5,
            adapted_parity: 20,
            adaptations: vec![
                Chromatophore::CrossStreamBackChannel, // +10
                Chromatophore::SolarPanelInversion,    // +5
            ],
            status: DeviceStatus::Alive,
        },
    ]
}

/// Display the parity model.
pub fn show_parity() {
    let devices = all_devices();
    let total: usize = devices.len();
    let alive: usize = devices.iter().filter(|d| !matches!(d.status, DeviceStatus::Down | DeviceStatus::NotOnNetwork)).count();

    println!("🦑 inkFish Parity — Steam as Master Grade");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();
    println!("  SteamOS = 100%.  Everything else gets chromatophore adaptations");
    println!("  to reach closer to parity.  Same organism, different skins.");
    println!();

    for device in &devices {
        let status_icon = match device.status {
            DeviceStatus::Alive => "🟢",
            DeviceStatus::Gaming => "🎮",
            DeviceStatus::Down => "🔴",
            DeviceStatus::NotOnNetwork => "⚫",
        };

        let bar_filled = device.adapted_parity as usize / 5;
        let bar_empty = 20 - bar_filled;
        let bar = format!("{}{}", "█".repeat(bar_filled), "░".repeat(bar_empty));

        println!(
            "  {} {:<40} [{bar}] {:>3}%",
            status_icon, device.name, device.adapted_parity,
        );

        if device.base_parity != device.adapted_parity {
            println!(
                "     base: {}% → adapted: {}%  (+{} from {} chromatophores)",
                device.base_parity,
                device.adapted_parity,
                device.adapted_parity - device.base_parity,
                device.adaptations.len(),
            );
            for adapt in &device.adaptations {
                println!("       🔹 +{:>2}  {}", adapt.parity_boost(), adapt.description());
            }
        }
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════════");
    println!("  {total} devices | {alive} alive | Steam is the master grade");
    println!();
    println!("  KEY INSIGHT: inkFish doesn't make every device identical.");
    println!("  It applies the RIGHT chromatophore to each device's skin.");
    println!("  Xbox gets Moonlight + Steam Link. PS4 gets Chiaki + Steam Link.");
    println!("  Pixel gets ADB + Solar. milkGate gets back-channel + sensors.");
    println!("  Same organism, same games, different protocol translations.");
}
