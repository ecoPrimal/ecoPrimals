//! discoball — Inverted Discoball Signal Topology
//!
//! # The Physics
//!
//! A normal discoball scatters photons outward from a central point.
//! The INVERTED discoball does the opposite: signal converges to a
//! null point, collapses to zero volume, then re-scatters through
//! the spine.
//!
//! Two identical Flint GL-MT6000s form a CHIRAL PAIR:
//! - Same hardware (mirror images)
//! - Different frequencies (Ch2/56 vs Ch8/60 = left hand / right hand)
//! - Signal flows INWARD toward the 10G spine (null point)
//! - The spine is the singularity where photons reach zero volume
//! - After the null, signal re-emerges on the other side, inverted
//!
//! # Chirality
//!
//! Chiral: cannot be superimposed on its mirror image (like hands).
//! Achiral: CAN be superimposed (like a sphere).
//!
//! The Flints are chiral because they broadcast on DIFFERENT channels.
//! But the organism NEEDS them achiral — interchangeable, either one
//! can serve any device. BT beacons are the mechanism that TILTS
//! the mirror to make them functionally achiral.
//!
//! # The Steam Caster as Black Hole
//!
//! Cast devices (.170 Chromecast, .152 Vizio, .197 Alexa) are SINKS.
//! They absorb signal (streams) and convert photons to visible light.
//! In the inverted discoball, they're the event horizon — once signal
//! crosses into a Cast device, it becomes display. No escape.
//!
//! # bingoCube as Coordination Seed
//!
//! bingoCube (9.3MB, coordination primal) negotiates which Flint
//! serves which client. The "discoball" is bingoCube's facet map
//! INVERTED — instead of assigning outward reflections, it assigns
//! inward absorption paths. Each facet is a client↔Flint binding.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// A lighthouse beacon — one Flint radio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lighthouse {
    pub name: String,
    pub ip: String,
    pub house: u8,
    pub radios: Vec<Radio>,
    pub bt_visible: Vec<String>,
    /// Chirality sign: +1 for Flint 1, -1 for Flint 2.
    pub chirality: i8,
}

/// A single radio on a Flint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Radio {
    pub interface: String,
    pub ssid: String,
    pub channel: u16,
    pub frequency_mhz: u32,
    pub bandwidth: String,
    pub max_rate_mbps: u32,
    pub clients: Vec<SignalClient>,
}

/// A WiFi client seen by a radio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalClient {
    pub mac: String,
    pub identity: String,
    pub rssi_dbm: i16,
    pub rate_mbps: u32,
    /// Which house the client is physically in.
    pub house: u8,
}

/// A signal sink (Cast device, display, printer).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlackHole {
    pub name: String,
    pub ip: String,
    pub mac: String,
    /// Absorption type: Cast, Print, Display, Speak.
    pub absorption: AbsorptionKind,
    /// Event horizon ports (Cast:8009, Print:631, etc).
    pub ports: Vec<u16>,
    /// Signal strength to nearest Flint.
    pub rssi_to_flint: i16,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AbsorptionKind {
    /// Google Cast — absorbs video/audio stream into photons (display).
    Cast,
    /// IPP Print — absorbs data into physical atoms (paper).
    Print,
    /// Speaker — absorbs data into pressure waves (sound).
    Speak,
    /// Display — absorbs frames into photons (screen).
    Display,
    /// Scan — REVERSE black hole: absorbs photons into data.
    Scan,
}

impl std::fmt::Display for AbsorptionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cast => write!(f, "Cast (photons)"),
            Self::Print => write!(f, "Print (atoms)"),
            Self::Speak => write!(f, "Speak (pressure)"),
            Self::Display => write!(f, "Display (photons)"),
            Self::Scan => write!(f, "Scan (anti-hole)"),
        }
    }
}

/// The inverted discoball — the full signal topology.
#[derive(Debug, Serialize, Deserialize)]
pub struct InvertedDiscoball {
    /// The two chiral lighthouses.
    pub lighthouses: [Lighthouse; 2],
    /// Signal sinks (black holes).
    pub black_holes: Vec<BlackHole>,
    /// The null point (spine singularity).
    pub null_point: NullPoint,
    /// Total facets (client↔Flint bindings).
    pub facets: u32,
    /// Volume: null (0) or negative (-N) depending on convergence.
    pub volume: f64,
    /// BT cant angle — how much the mirrors are tilted.
    pub bt_cant_degrees: f64,
}

/// The null point where signal converges to zero.
#[derive(Debug, Serialize, Deserialize)]
pub struct NullPoint {
    /// Physical medium: 10G fiber spine.
    pub medium: String,
    /// Latency through the null (spine crossing time).
    pub latency_ms: f64,
    /// Bandwidth at null: 10Gbps.
    pub bandwidth_gbps: f64,
    /// The null point is where chirality flips.
    /// Signal entering from Flint 1 (ch2/56) exits as Flint 2 (ch8/60).
    pub chirality_flip: bool,
}

/// Compute the "volume" of the inverted discoball.
///
/// Normal discoball: volume = (4/3)πr³ (positive, scattering outward)
/// Inverted discoball: volume = -(4/3)πr³ (negative, converging inward)
///
/// r = average RSSI distance (higher signal = smaller radius = more collapsed)
/// When all signals are at 0 dBm (touching the source), volume = 0 (null).
fn compute_volume(clients: &[(i16, u32)]) -> f64 {
    if clients.is_empty() {
        return 0.0;
    }
    // Convert RSSI to "distance" (more negative = further away)
    // At -20dBm = close (small r), at -80dBm = far (large r)
    let avg_rssi: f64 = clients.iter().map(|(r, _)| *r as f64).sum::<f64>() / clients.len() as f64;
    // Normalize: -20 → r=1, -80 → r=4
    let r = (avg_rssi.abs() - 20.0).max(0.0) / 15.0 + 1.0;
    // Inverted: negative volume
    -(4.0 / 3.0) * PI * r * r * r
}

/// Compute BT cant angle from BT beacon visibility.
///
/// If both Flints see the SAME BT devices → mirrors parallel → cant = 0°
/// If they see DIFFERENT BT devices → mirrors angled → cant > 0°
/// Maximum cant at 90° means perfect separation (each house isolated).
fn compute_bt_cant(flint1_bt: &[String], flint2_bt: &[String]) -> f64 {
    if flint1_bt.is_empty() && flint2_bt.is_empty() {
        return 0.0;
    }
    let total = flint1_bt.len() + flint2_bt.len();
    let shared = flint1_bt.iter().filter(|b| flint2_bt.contains(b)).count();
    if total == 0 { return 0.0; }
    // 0 shared → 90°, all shared → 0°
    let separation = 1.0 - (shared as f64 * 2.0 / total as f64);
    separation * 90.0
}

/// Build the full inverted discoball from live data.
pub fn build_discoball() -> InvertedDiscoball {
    let flint1 = Lighthouse {
        name: "Flint 1 (Router)".into(),
        ip: "192.168.4.1".into(),
        house: 1,
        chirality: 1,
        radios: vec![
            Radio {
                interface: "ra0".into(), ssid: "Aperture Science".into(),
                channel: 2, frequency_mhz: 2417, bandwidth: "HE20".into(),
                max_rate_mbps: 573,
                clients: vec![
                    SignalClient { mac: "A0:D0:DC:EE:EE:39".into(), identity: "Alexa".into(), rssi_dbm: -17, rate_mbps: 173, house: 1 },
                    SignalClient { mac: "D4:80:8B:1B:9F:01".into(), identity: "Epson ET-2400".into(), rssi_dbm: -22, rate_mbps: 72, house: 1 },
                    SignalClient { mac: "E8:38:A0:53:EA:D8".into(), identity: "Chromecast".into(), rssi_dbm: -25, rate_mbps: 200, house: 1 },
                    SignalClient { mac: "AC:41:6A:5C:B4:CF".into(), identity: "Vizio TV".into(), rssi_dbm: -27, rate_mbps: 229, house: 1 },
                    SignalClient { mac: "64:16:66:A4:82:0A".into(), identity: "Nest".into(), rssi_dbm: -29, rate_mbps: 72, house: 1 },
                ],
            },
            Radio {
                interface: "rax1".into(), ssid: "GAIA".into(),
                channel: 56, frequency_mhz: 5280, bandwidth: "HE160".into(),
                max_rate_mbps: 4803,
                clients: vec![
                    SignalClient { mac: "90:82:C3:74:12:CA".into(), identity: "steamFRAME".into(), rssi_dbm: -28, rate_mbps: 2401, house: 1 },
                    SignalClient { mac: "C8:5E:A9:C9:6F:80".into(), identity: "eastGate".into(), rssi_dbm: -38, rate_mbps: 864, house: 1 },
                    SignalClient { mac: "D2:49:F5:8A:71:16".into(), identity: "Unknown .190".into(), rssi_dbm: -23, rate_mbps: 866, house: 1 },
                    SignalClient { mac: "1E:0D:9C:8D:CF:FE".into(), identity: "Unknown WiFi6".into(), rssi_dbm: -26, rate_mbps: 1201, house: 1 },
                ],
            },
        ],
        bt_visible: vec![
            "ET-2400 (D6:80:8B)".into(),
            "steamFRAME (90:82:C3)".into(),
        ],
    };

    let flint2 = Lighthouse {
        name: "Flint 2 (Bridge)".into(),
        ip: "192.168.4.9".into(),
        house: 2,
        chirality: -1,
        radios: vec![
            Radio {
                interface: "ra0".into(), ssid: "Aperture Science".into(),
                channel: 8, frequency_mhz: 2447, bandwidth: "HE40".into(),
                max_rate_mbps: 1147,
                clients: vec![
                    SignalClient { mac: "10:F6:0A:54:57:CC".into(), identity: "Xiaomi IoT".into(), rssi_dbm: -18, rate_mbps: 541, house: 2 },
                ],
            },
            Radio {
                interface: "rax1".into(), ssid: "GAIA".into(),
                channel: 60, frequency_mhz: 5300, bandwidth: "HE160".into(),
                max_rate_mbps: 4803,
                clients: vec![
                    SignalClient { mac: "C0:8B:27:9C:63:EC".into(), identity: "milkGate".into(), rssi_dbm: -34, rate_mbps: 720, house: 2 },
                    SignalClient { mac: "68:54:5A:D0:69:A2".into(), identity: "ironGate WiFi".into(), rssi_dbm: -40, rate_mbps: 1921, house: 2 },
                ],
            },
        ],
        bt_visible: vec![
            "steamFRAME (90:82:C3)".into(),
        ],
    };

    // Collect all client signal data for volume computation
    let all_clients: Vec<(i16, u32)> = flint1.radios.iter()
        .chain(flint2.radios.iter())
        .flat_map(|r| r.clients.iter().map(|c| (c.rssi_dbm, c.rate_mbps)))
        .collect();

    let volume = compute_volume(&all_clients);
    let bt_cant = compute_bt_cant(&flint1.bt_visible, &flint2.bt_visible);

    let black_holes = vec![
        BlackHole {
            name: "Chromecast 'Basement'".into(), ip: "192.168.4.170".into(),
            mac: "E8:38:A0:53:EA:D8".into(), absorption: AbsorptionKind::Cast,
            ports: vec![8008, 8009, 8443], rssi_to_flint: -25,
        },
        BlackHole {
            name: "Vizio SmartCast TV".into(), ip: "192.168.4.152".into(),
            mac: "AC:41:6A:5C:B4:CF".into(), absorption: AbsorptionKind::Display,
            ports: vec![8009], rssi_to_flint: -27,
        },
        BlackHole {
            name: "Alexa Echo".into(), ip: "192.168.4.197".into(),
            mac: "A0:D0:DC:EE:EE:39".into(), absorption: AbsorptionKind::Speak,
            ports: vec![8009, 55443], rssi_to_flint: -17,
        },
        BlackHole {
            name: "Epson ET-2400".into(), ip: "192.168.4.200".into(),
            mac: "D4:80:8B:1B:9F:01".into(), absorption: AbsorptionKind::Print,
            ports: vec![631], rssi_to_flint: -22,
        },
        BlackHole {
            name: "Epson ES-400II".into(), ip: "USB".into(),
            mac: "N/A".into(), absorption: AbsorptionKind::Scan,
            ports: vec![], rssi_to_flint: 0,
        },
    ];

    let facets = all_clients.len() as u32;

    InvertedDiscoball {
        lighthouses: [flint1, flint2],
        black_holes,
        null_point: NullPoint {
            medium: "10G fiber spine, MikroTik(.2) ↔ JetStream(.111)".into(),
            latency_ms: 0.9,
            bandwidth_gbps: 10.0,
            chirality_flip: true,
        },
        facets,
        volume,
        bt_cant_degrees: bt_cant,
    }
}

/// Display the inverted discoball.
pub fn show_discoball() {
    let db = build_discoball();

    println!("🪩🔄 INVERTED DISCOBALL — Signal Topology");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("  Normal discoball: photons scatter OUTWARD (positive volume)");
    println!("  Inverted discoball: signal converges to NULL then scatters");
    println!("  Volume: {:.2} (NEGATIVE — converging inward)", db.volume);
    println!("  BT cant angle: {:.1}° (mirror tilt between houses)", db.bt_cant_degrees);
    println!();

    // Lighthouses
    for (i, lh) in db.lighthouses.iter().enumerate() {
        let chirality = if lh.chirality > 0 { "L (left-handed)" } else { "R (right-handed)" };
        let icon = if i == 0 { "🏠" } else { "🏚️" };
        println!("  {icon} {} — House {} — chirality: {chirality}", lh.name, lh.house);
        for r in &lh.radios {
            let client_count = r.clients.len();
            println!("      📡 {} '{}' Ch{} {}MHz {} ({} Mbps peak)",
                r.interface, r.ssid, r.channel, r.frequency_mhz, r.bandwidth, r.max_rate_mbps);
            for c in &r.clients {
                let bar = signal_bar(c.rssi_dbm);
                println!("         {bar} {} ({}) {}dBm {}Mbps",
                    c.identity, c.mac, c.rssi_dbm, c.rate_mbps);
            }
            if client_count == 0 {
                println!("         (no clients)");
            }
        }
        println!("      🔵 BT visible: {:?}", lh.bt_visible);
        println!();
    }

    // Null point
    println!("  ⬛ NULL POINT (spine singularity)");
    println!("      Medium:    {}", db.null_point.medium);
    println!("      Latency:   {}ms (chirality flip: {})",
        db.null_point.latency_ms, db.null_point.chirality_flip);
    println!("      Bandwidth: {} Gbps", db.null_point.bandwidth_gbps);
    println!("      Signal entering as Ch2/56 exits as Ch8/60 (frequency inversion)");
    println!();

    // Black holes
    println!("  🕳️  BLACK HOLES (signal sinks — event horizons)");
    println!();
    for bh in &db.black_holes {
        let bar = signal_bar(bh.rssi_to_flint);
        println!("      {bar} {} @ {} — {}", bh.name, bh.ip, bh.absorption);
        println!("         MAC: {} | Ports: {:?} | RSSI: {}dBm",
            bh.mac, bh.ports, bh.rssi_to_flint);
    }
    println!();

    // Topology diagram
    println!("  ═══════════════════════════════════════════════════════════════");
    println!("  INVERTED DISCOBALL TOPOLOGY:");
    println!();
    println!("                  🕳️ Cast sinks     🕳️ Cast sinks");
    println!("                  (Chromecast,       (Alexa speaks,");
    println!("                   Vizio displays)    printer atoms)");
    println!("                       ↑                  ↑");
    println!("                       │ absorb           │ absorb");
    println!("        ┌──────────────┴──────────────────┴──────────────┐");
    println!("        │                                                │");
    println!("   🏠 FLINT 1         ⬛ NULL POINT          🏚️ FLINT 2  ");
    println!("   Ch2 / Ch56  ───→  10G SPINE  ←───  Ch8 / Ch60");
    println!("   L-handed    ───→  (0 volume) ←───  R-handed");
    println!("   5 IoT + 4 fast    0.9ms flip       1 IoT + 2 fast");
    println!("        │                                                │");
    println!("        └──────────────┬──────────────────┬──────────────┘");
    println!("                       │ scatter          │ scatter");
    println!("                       ↓                  ↓");
    println!("                  🔵 BT beacons     🔵 BT beacons");
    println!("                  (cant the mirror   (tilt to steer");
    println!("                   for achirality)    signal focus)");
    println!();
    println!("  CHIRAL → ACHIRAL via BT:");
    println!("    Cant angle = {:.1}° — higher = more spatial separation", db.bt_cant_degrees);
    println!("    At 0°: both Flints functionally identical (achiral)");
    println!("    At 90°: perfect house isolation (maximum chirality)");
    println!("    BT beacons tilt the mirror between these extremes");
    println!();
    println!("  {facets} facets (client↔Flint bindings) on the inverted surface", facets = db.facets);

    // Write state
    let json = serde_json::to_string_pretty(&db).unwrap_or_default();
    let path = "/home/eastgate/Development/ecoPrimals/data/body/discoball-state.json";
    if let Ok(mut f) = std::fs::File::create(path) {
        use std::io::Write;
        let _ = f.write_all(json.as_bytes());
        println!();
        println!("  📄 Discoball state → {path}");
    }
}

/// Signal strength bar visualization.
fn signal_bar(rssi: i16) -> &'static str {
    match rssi {
        0 => "█████",
        -20..=0 => "████▒",
        -30..=-20 => "███▒▒",
        -40..=-30 => "██▒▒▒",
        -60..=-40 => "█▒▒▒▒",
        _ => "▒▒▒▒▒",
    }
}
