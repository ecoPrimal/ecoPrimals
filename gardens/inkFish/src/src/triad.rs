//! Triad module — every device is THREE things at once.
//!
//! ## The Three Lines
//!
//! ```text
//!         COMPUTE ─────── DISPLAY
//!              \           /
//!               \         /
//!                \       /
//!                 SIGNAL
//! ```
//!
//! - **COMPUTE**: GPU renders, CPU processes, AI infers.
//! - **DISPLAY**: Screen shows, speaker sounds, haptic vibrates.
//! - **SIGNAL**: WiFi relays, BT beacons, USB casts, sensor sweeps.
//!
//! Every device participates in ALL THREE simultaneously.
//! The Frame renders (compute) AND displays AND relays WiFi signal.
//! The Xbox receives streams (display) AND relays BT AND could compute xCloud.
//! Even the Chromecast relays WiFi packets while displaying a stream.
//!
//! ## The Third Line
//!
//! The overlooked one. Everyone thinks of compute + display.
//! But the SIGNAL line — all the radios, all the beacons, all the relays —
//! is what makes the mesh ALIVE. Every device is a radio tower.
//! Every WiFi chip is a beacon. Every BT module is a sensor.
//! The USB caster, the Steam Link dongle, the Flint APs, the Chromecast —
//! they're all the same thing: signal fabric.
//!
//! ## ALL INGEST ALL
//!
//! The Frame doesn't just display games — it ingests WiFi RSSI, BT proximity,
//! accelerometer vibration, microphone audio, touch capacitance, camera light.
//! Every device is a sensor that happens to also compute and display.
//! The signal line IS the organism's nervous system.

use serde::{Deserialize, Serialize};

/// Triad score — how capable a device is in each of the three lines.
/// Each score is 0-100.
#[derive(Debug, Serialize, Deserialize)]
pub struct TriadScore {
    /// Compute capability (GPU TFLOPS, CPU cores, AI inference).
    pub compute: u8,
    /// Display capability (screens, speakers, haptics, resolution).
    pub display: u8,
    /// Signal capability (WiFi, BT, USB, sensors, relay reach).
    pub signal: u8,
}

impl TriadScore {
    /// Total triad strength (0-300).
    #[must_use]
    pub fn total(&self) -> u16 {
        u16::from(self.compute) + u16::from(self.display) + u16::from(self.signal)
    }

    /// Balance ratio — how evenly distributed across the three lines.
    /// 1.0 = perfect triangle, 0.0 = all in one line.
    #[must_use]
    pub fn balance(&self) -> f32 {
        let vals = [f32::from(self.compute), f32::from(self.display), f32::from(self.signal)];
        let max = vals.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        if max == 0.0 { return 0.0; }
        let min = vals.iter().cloned().fold(f32::INFINITY, f32::min);
        min / max
    }
}

/// Radio — a signal emitter/receiver on a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Radio {
    /// Radio type.
    pub kind: RadioKind,
    /// Signal reach (approximate meters, or "mesh" for relay).
    pub reach_m: u32,
    /// Can this radio relay data for OTHER devices?
    pub relay: bool,
    /// Can this radio sense the environment?
    pub sense: bool,
}

/// Types of radios in the mesh.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RadioKind {
    /// WiFi (2.4/5/6 GHz).
    WiFi { band: String, protocol: String },
    /// Bluetooth (Classic + LE).
    Bluetooth { version: String },
    /// USB dongle/caster (Valve WiFi 6, etc).
    UsbDongle { name: String },
    /// Ethernet (wired, but still a "radio" in the signal line).
    Ethernet { speed: String },
    /// 10G Fiber (the spine).
    Fiber10g,
    /// Infrared (TV remotes, proximity).
    Infrared,
    /// NFC (near field).
    Nfc,
    /// Cellular (4G/5G — phone only).
    Cellular { generation: String },
    /// Steam Remote Play UDP (virtual radio over WiFi).
    SteamRpUdp,
    /// ADB WiFi (virtual radio for Android control).
    AdbWifi,
    /// Sensor array (accelerometer, gyro, magnetometer — passive signal).
    SensorArray { count: u8 },
    /// Camera (light sensor — passive signal).
    Camera { count: u8 },
    /// Microphone (acoustic sensor).
    Microphone,
}

/// A device in the triad model.
#[derive(Debug, Serialize, Deserialize)]
pub struct TriadDevice {
    /// Name.
    pub name: String,
    /// Triad scores.
    pub triad: TriadScore,
    /// Radios — the signal line hardware.
    pub radios: Vec<Radio>,
    /// Role description.
    pub role: String,
}

/// All devices with their triad scores and radio inventories.
pub fn all_triads() -> Vec<TriadDevice> {
    vec![
        // === STEAM FRAME ===
        TriadDevice {
            name: "frameGate (Steam Frame)".into(),
            triad: TriadScore { compute: 45, display: 95, signal: 90 },
            radios: vec![
                Radio { kind: RadioKind::WiFi { band: "5GHz".into(), protocol: "WiFi 6E".into() }, reach_m: 30, relay: true, sense: true },
                Radio { kind: RadioKind::Bluetooth { version: "5.3".into() }, reach_m: 10, relay: false, sense: true },
                Radio { kind: RadioKind::SteamRpUdp, reach_m: 0, relay: false, sense: false },
                Radio { kind: RadioKind::SensorArray { count: 37 }, reach_m: 0, relay: false, sense: true },
                Radio { kind: RadioKind::Camera { count: 22 }, reach_m: 5, relay: false, sense: true },
                Radio { kind: RadioKind::Microphone, reach_m: 3, relay: false, sense: true },
            ],
            role: "FACE — display+AR+22 cameras+37 sensors+WiFi relay".into(),
        },

        // === STEAM DECK ===
        TriadDevice {
            name: "deckGate (Steam Deck v1)".into(),
            triad: TriadScore { compute: 30, display: 70, signal: 50 },
            radios: vec![
                Radio { kind: RadioKind::WiFi { band: "5GHz".into(), protocol: "WiFi 5".into() }, reach_m: 20, relay: true, sense: true },
                Radio { kind: RadioKind::Bluetooth { version: "5.0".into() }, reach_m: 10, relay: false, sense: true },
                Radio { kind: RadioKind::SteamRpUdp, reach_m: 0, relay: false, sense: false },
                Radio { kind: RadioKind::SensorArray { count: 4 }, reach_m: 0, relay: false, sense: true },
            ],
            role: "DISPLAY — 7\" touch LCD, gyro, mounted in House 1".into(),
        },

        // === NORTH GATE ===
        TriadDevice {
            name: "northGate (RTX 5090)".into(),
            triad: TriadScore { compute: 100, display: 10, signal: 60 },
            radios: vec![
                Radio { kind: RadioKind::Ethernet { speed: "2.5GbE".into() }, reach_m: 100, relay: true, sense: false },
                Radio { kind: RadioKind::WiFi { band: "5GHz".into(), protocol: "WiFi 6".into() }, reach_m: 30, relay: true, sense: true },
                Radio { kind: RadioKind::SteamRpUdp, reach_m: 0, relay: true, sense: false },
            ],
            role: "HEART — 192 RT4 cores, primary render, Steam RP host".into(),
        },

        // === IRON GATE ===
        TriadDevice {
            name: "ironGate (RTX 5070)".into(),
            triad: TriadScore { compute: 85, display: 10, signal: 55 },
            radios: vec![
                Radio { kind: RadioKind::Ethernet { speed: "2.5GbE".into() }, reach_m: 100, relay: true, sense: false },
                Radio { kind: RadioKind::SteamRpUdp, reach_m: 0, relay: true, sense: false },
            ],
            role: "FORGE 2 — secondary render, Sunshine host".into(),
        },

        // === EAST GATE ===
        TriadDevice {
            name: "eastGate (RTX 4070)".into(),
            triad: TriadScore { compute: 70, display: 10, signal: 80 },
            radios: vec![
                Radio { kind: RadioKind::Ethernet { speed: "1GbE".into() }, reach_m: 100, relay: true, sense: false },
                Radio { kind: RadioKind::Ethernet { speed: "1GbE".into() }, reach_m: 100, relay: true, sense: false },
                Radio { kind: RadioKind::WiFi { band: "5GHz".into(), protocol: "WiFi 6".into() }, reach_m: 30, relay: true, sense: true },
                Radio { kind: RadioKind::SteamRpUdp, reach_m: 0, relay: true, sense: false },
                Radio { kind: RadioKind::AdbWifi, reach_m: 0, relay: true, sense: false },
            ],
            role: "BRAIN — 3 NICs, ADB bridge, Tor relay, data spine hub".into(),
        },

        // === SOUTH GATE ===
        TriadDevice {
            name: "southGate (RTX 4060)".into(),
            triad: TriadScore { compute: 60, display: 10, signal: 45 },
            radios: vec![
                Radio { kind: RadioKind::Ethernet { speed: "1GbE".into() }, reach_m: 100, relay: true, sense: false },
                Radio { kind: RadioKind::SteamRpUdp, reach_m: 0, relay: true, sense: false },
            ],
            role: "SOUTH POLE — House 2 render node".into(),
        },

        // === SWIFT GATE ===
        TriadDevice {
            name: "swiftGate (RTX 3070)".into(),
            triad: TriadScore { compute: 55, display: 10, signal: 30 },
            radios: vec![
                Radio { kind: RadioKind::WiFi { band: "5GHz".into(), protocol: "WiFi 6".into() }, reach_m: 30, relay: true, sense: true },
                Radio { kind: RadioKind::SteamRpUdp, reach_m: 0, relay: true, sense: false },
            ],
            role: "POLARIS H2 — WiFi-only, asleep".into(),
        },

        // === XBOX ===
        TriadDevice {
            name: "Xbox Series".into(),
            triad: TriadScore { compute: 50, display: 80, signal: 40 },
            radios: vec![
                Radio { kind: RadioKind::Ethernet { speed: "1GbE".into() }, reach_m: 100, relay: false, sense: false },
                Radio { kind: RadioKind::WiFi { band: "5GHz".into(), protocol: "WiFi 5".into() }, reach_m: 20, relay: false, sense: true },
                Radio { kind: RadioKind::Bluetooth { version: "5.0".into() }, reach_m: 10, relay: false, sense: true },
                Radio { kind: RadioKind::Infrared, reach_m: 5, relay: false, sense: true },
                Radio { kind: RadioKind::SteamRpUdp, reach_m: 0, relay: false, sense: false },
            ],
            role: "CONSOLE — 12 TFLOPS RDNA 2, HDMI display, Steam Link receiver".into(),
        },

        // === PS4 ===
        TriadDevice {
            name: "PS4".into(),
            triad: TriadScore { compute: 25, display: 75, signal: 35 },
            radios: vec![
                Radio { kind: RadioKind::Ethernet { speed: "100Mbps".into() }, reach_m: 100, relay: false, sense: false },
                Radio { kind: RadioKind::WiFi { band: "5GHz".into(), protocol: "WiFi 5".into() }, reach_m: 20, relay: false, sense: true },
                Radio { kind: RadioKind::Bluetooth { version: "4.0".into() }, reach_m: 10, relay: false, sense: true },
            ],
            role: "CONSOLE — 1.84 TFLOPS GCN, Chiaki source, Steam Link receiver".into(),
        },

        // === PIXEL ===
        TriadDevice {
            name: "grapheneGate (Pixel 8a)".into(),
            triad: TriadScore { compute: 20, display: 40, signal: 85 },
            radios: vec![
                Radio { kind: RadioKind::WiFi { band: "5GHz".into(), protocol: "WiFi 6E".into() }, reach_m: 30, relay: true, sense: true },
                Radio { kind: RadioKind::Bluetooth { version: "5.3".into() }, reach_m: 10, relay: true, sense: true },
                Radio { kind: RadioKind::Cellular { generation: "5G".into() }, reach_m: 10000, relay: false, sense: true },
                Radio { kind: RadioKind::Nfc, reach_m: 0, relay: false, sense: true },
                Radio { kind: RadioKind::AdbWifi, reach_m: 0, relay: false, sense: false },
                Radio { kind: RadioKind::SensorArray { count: 37 }, reach_m: 0, relay: false, sense: true },
                Radio { kind: RadioKind::Camera { count: 3 }, reach_m: 5, relay: false, sense: true },
                Radio { kind: RadioKind::Microphone, reach_m: 3, relay: false, sense: true },
            ],
            role: "LEFT EYE — 5G+WiFi6E+BT5.3+NFC+37 sensors+3 cameras, ADB roving".into(),
        },

        // === FLINT APS ===
        TriadDevice {
            name: "Flint 1 (Left Ear, House 1)".into(),
            triad: TriadScore { compute: 5, display: 0, signal: 95 },
            radios: vec![
                Radio { kind: RadioKind::WiFi { band: "2.4GHz".into(), protocol: "WiFi 6".into() }, reach_m: 50, relay: true, sense: true },
                Radio { kind: RadioKind::WiFi { band: "5GHz".into(), protocol: "WiFi 6".into() }, reach_m: 30, relay: true, sense: true },
                Radio { kind: RadioKind::Ethernet { speed: "1GbE".into() }, reach_m: 100, relay: true, sense: false },
            ],
            role: "LEFT EAR — WiFi triangulation, RSSI sensing, relay".into(),
        },
        TriadDevice {
            name: "Flint 2 (Right Ear, House 2)".into(),
            triad: TriadScore { compute: 5, display: 0, signal: 95 },
            radios: vec![
                Radio { kind: RadioKind::WiFi { band: "2.4GHz".into(), protocol: "WiFi 6".into() }, reach_m: 50, relay: true, sense: true },
                Radio { kind: RadioKind::WiFi { band: "5GHz".into(), protocol: "WiFi 6".into() }, reach_m: 30, relay: true, sense: true },
                Radio { kind: RadioKind::Ethernet { speed: "1GbE".into() }, reach_m: 100, relay: true, sense: false },
            ],
            role: "RIGHT EAR — WiFi triangulation, RSSI sensing, relay".into(),
        },

        // === GL-MT6000 ROUTER ===
        TriadDevice {
            name: "GL-MT6000 (Voice)".into(),
            triad: TriadScore { compute: 10, display: 0, signal: 100 },
            radios: vec![
                Radio { kind: RadioKind::WiFi { band: "2.4GHz".into(), protocol: "WiFi 6".into() }, reach_m: 50, relay: true, sense: true },
                Radio { kind: RadioKind::WiFi { band: "5GHz".into(), protocol: "WiFi 6".into() }, reach_m: 30, relay: true, sense: true },
                Radio { kind: RadioKind::Ethernet { speed: "2.5GbE".into() }, reach_m: 100, relay: true, sense: false },
            ],
            role: "VOICE — main router, 3 SSIDs, NTP, DHCP, WAN gateway".into(),
        },

        // === CHROMECAST ===
        TriadDevice {
            name: "Chromecast (.169)".into(),
            triad: TriadScore { compute: 5, display: 60, signal: 40 },
            radios: vec![
                Radio { kind: RadioKind::WiFi { band: "5GHz".into(), protocol: "WiFi 5".into() }, reach_m: 20, relay: true, sense: true },
                Radio { kind: RadioKind::Bluetooth { version: "4.2".into() }, reach_m: 10, relay: false, sense: true },
            ],
            role: "CAST — HDMI display, Steam Link, WiFi relay".into(),
        },

        // === MILK-V JUPITER ===
        TriadDevice {
            name: "milkGate (Milk-V Jupiter 2)".into(),
            triad: TriadScore { compute: 15, display: 0, signal: 70 },
            radios: vec![
                Radio { kind: RadioKind::Ethernet { speed: "10GbE SFP+".into() }, reach_m: 300, relay: true, sense: false },
                Radio { kind: RadioKind::WiFi { band: "5GHz".into(), protocol: "WiFi 6".into() }, reach_m: 30, relay: true, sense: true },
            ],
            role: "TRANSPONDER — 10GbE SFP+, 60 TOPS NPU, universal relay".into(),
        },

        // === VALVE WIFI 6 DONGLE ===
        TriadDevice {
            name: "Valve WiFi 6 Dongle (28de:2432)".into(),
            triad: TriadScore { compute: 0, display: 0, signal: 85 },
            radios: vec![
                Radio { kind: RadioKind::UsbDongle { name: "Valve WiFi 6 2.4Gbps".into() }, reach_m: 15, relay: true, sense: false },
            ],
            role: "CASTER — direct 2.4Gbps WiFi 6 link, Steam RP bridge".into(),
        },

        // === STEAM LINK ===
        TriadDevice {
            name: "Steam Link (relay)".into(),
            triad: TriadScore { compute: 5, display: 50, signal: 60 },
            radios: vec![
                Radio { kind: RadioKind::Ethernet { speed: "100Mbps".into() }, reach_m: 100, relay: true, sense: false },
                Radio { kind: RadioKind::WiFi { band: "5GHz".into(), protocol: "WiFi 5".into() }, reach_m: 20, relay: true, sense: true },
                Radio { kind: RadioKind::Bluetooth { version: "4.0".into() }, reach_m: 10, relay: false, sense: true },
            ],
            role: "TAIL BEACON — HDMI relay, Steam RP receiver, BT controller bridge".into(),
        },

        // === 10G SPINE ===
        TriadDevice {
            name: "10G Fiber Spine (MikroTik↔JetStream)".into(),
            triad: TriadScore { compute: 0, display: 0, signal: 100 },
            radios: vec![
                Radio { kind: RadioKind::Fiber10g, reach_m: 80, relay: true, sense: false },
            ],
            role: "SPINE — 80m fiber, 0.87ms latency, House 1 ↔ House 2".into(),
        },
    ]
}

/// Display the triad model.
pub fn show_triad() {
    let devices = all_triads();

    let total_radios: usize = devices.iter().map(|d| d.radios.len()).sum();
    let relay_radios: usize = devices.iter()
        .flat_map(|d| d.radios.iter())
        .filter(|r| r.relay)
        .count();
    let sense_radios: usize = devices.iter()
        .flat_map(|d| d.radios.iter())
        .filter(|r| r.sense)
        .count();

    println!("🔺 inkFish Triad — COMPUTE × DISPLAY × SIGNAL");
    println!("═══════════════════════════════════════════════════════════════════════");
    println!();
    println!("  Every device is THREE things at once.");
    println!("  Every radio is a beacon. Every sensor is a signal.");
    println!("  The third line — SIGNAL — is the nervous system.");
    println!();

    for device in &devices {
        let t = &device.triad;

        let c_bar = "█".repeat(t.compute as usize / 10);
        let d_bar = "█".repeat(t.display as usize / 10);
        let s_bar = "█".repeat(t.signal as usize / 10);

        let balance = t.balance();
        let shape = if balance > 0.7 {
            "▲ balanced"
        } else if t.signal > t.compute && t.signal > t.display {
            "📡 signal-heavy"
        } else if t.compute > t.display && t.compute > t.signal {
            "⚡ compute-heavy"
        } else {
            "🖥  display-heavy"
        };

        println!("  {:<42} [{:>3}] {}", device.name, t.total(), shape);
        println!("    C [{:<10}] {:>3}   D [{:<10}] {:>3}   S [{:<10}] {:>3}",
            c_bar, t.compute, d_bar, t.display, s_bar, t.signal);

        let radio_str: Vec<String> = device.radios.iter().map(|r| {
            let flag = match (r.relay, r.sense) {
                (true, true) => "↔",
                (true, false) => "→",
                (false, true) => "←",
                (false, false) => "·",
            };
            let name = match &r.kind {
                RadioKind::WiFi { band, .. } => format!("WiFi {band}"),
                RadioKind::Bluetooth { version } => format!("BT {version}"),
                RadioKind::UsbDongle { name } => name.clone(),
                RadioKind::Ethernet { speed } => format!("Eth {speed}"),
                RadioKind::Fiber10g => "10G Fiber".into(),
                RadioKind::Infrared => "IR".into(),
                RadioKind::Nfc => "NFC".into(),
                RadioKind::Cellular { generation } => generation.clone(),
                RadioKind::SteamRpUdp => "Steam-RP".into(),
                RadioKind::AdbWifi => "ADB".into(),
                RadioKind::SensorArray { count } => format!("{count} sensors"),
                RadioKind::Camera { count } => format!("{count} cams"),
                RadioKind::Microphone => "mic".into(),
            };
            format!("{flag}{name}")
        }).collect();

        println!("    📡 {}", radio_str.join("  "));
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════════════");
    println!("  {} devices | {} radios ({} relay, {} sense)",
        devices.len(), total_radios, relay_radios, sense_radios);
    println!();
    println!("  THE THREE LINES:");
    println!("    COMPUTE — GPU/CPU renders and processes");
    println!("    DISPLAY — screen/speaker/haptic outputs");
    println!("    SIGNAL  — WiFi/BT/USB/sensor/camera INGESTS and RELAYS");
    println!();
    println!("  ALL INGEST ALL. Every device is a radio tower.");
    println!("  The Frame is a 22-camera sensor array that happens to display games.");
    println!("  The Flints are WiFi ears that triangulate Kevin in space.");
    println!("  The 10G spine is an 80m fiber nerve connecting two houses.");
    println!("  Even the USB dongle is a 2.4Gbps dedicated signal bridge.");
}
