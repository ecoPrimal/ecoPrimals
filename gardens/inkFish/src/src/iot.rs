//! IoT module — redox all House 1 + House 2 smart devices into Gaia.
//!
//! Every IoT device is a SENSOR or ACTUATOR in the organism:
//! - Nest Thermostat: temperature sensor + HVAC actuator
//! - Alexa: microphone sensor + speaker actuator
//! - Vizio TV: display actuator + Cast receiver
//! - Chromecast: Cast bridge → any display
//! - Epson printer: physical output actuator
//! - eero: WiFi signal sensor + mesh relay
//!
//! We don't CONTROL these devices (their owners do).
//! We SENSE them — count, measure signal, track presence.
//! Weather, not surveillance. Peripheral nervous system.

use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::process::Command;
use std::time::Duration;

/// An IoT device discovered on the LAN.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IoTDevice {
    pub ip: String,
    pub mac: String,
    pub hostname: String,
    pub manufacturer: String,
    pub device_type: IoTType,
    pub house: u8,
    pub alive: bool,
    pub open_ports: Vec<u16>,
    pub cast_name: Option<String>,
    pub role: String,
}

/// Types of IoT devices.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IoTType {
    Thermostat,
    VoiceAssistant,
    Television,
    CastDevice,
    Printer,
    Scanner,
    WiFiMesh,
    SmartPlug,
    Phone,
    PC,
    Unknown,
}

impl std::fmt::Display for IoTType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Thermostat => write!(f, "Thermostat"),
            Self::VoiceAssistant => write!(f, "Voice Assistant"),
            Self::Television => write!(f, "Smart TV"),
            Self::CastDevice => write!(f, "Cast Device"),
            Self::Printer => write!(f, "Printer"),
            Self::Scanner => write!(f, "Scanner"),
            Self::WiFiMesh => write!(f, "WiFi Mesh"),
            Self::SmartPlug => write!(f, "Smart Plug/Bridge"),
            Self::Phone => write!(f, "Phone"),
            Self::PC => write!(f, "PC"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

fn tcp_alive(ip: &str, port: u16) -> bool {
    let addr = format!("{ip}:{port}");
    if let Ok(sa) = addr.parse() {
        TcpStream::connect_timeout(&sa, Duration::from_secs(2)).is_ok()
    } else {
        false
    }
}

fn ping_alive(ip: &str) -> bool {
    Command::new("ping")
        .args(["-c", "1", "-W", "1", ip])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Query Chromecast info via eureka API.
fn cast_info(ip: &str) -> Option<String> {
    let out = Command::new("curl")
        .args(["-s", "--connect-timeout", "2",
               &format!("http://{ip}:8008/setup/eureka_info")])
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    // Extract "name" field
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
        v.get("name").and_then(|n| n.as_str()).map(|s| s.to_string())
    } else {
        None
    }
}

/// Build the known IoT device list and probe each one.
pub fn scan_iot() -> Vec<IoTDevice> {
    let mut devices = vec![];

    // Nest Thermostat
    let nest_alive = ping_alive("192.168.4.239");
    devices.push(IoTDevice {
        ip: "192.168.4.239".into(),
        mac: "64:16:66:a4:82:0a".into(),
        hostname: "09aa01af23190f64".into(),
        manufacturer: "Google".into(),
        device_type: IoTType::Thermostat,
        house: 1,
        alive: nest_alive,
        open_ports: vec![],
        cast_name: None,
        role: "Temperature sensor + HVAC actuator. Gaia feels house temp.".into(),
    });

    // Alexa / Echo
    let alexa_alive = ping_alive("192.168.4.197");
    let mut alexa_ports = vec![];
    if tcp_alive("192.168.4.197", 8009) { alexa_ports.push(8009); }
    if tcp_alive("192.168.4.197", 55443) { alexa_ports.push(55443); }
    devices.push(IoTDevice {
        ip: "192.168.4.197".into(),
        mac: "a0:d0:dc:ee:ee:39".into(),
        hostname: "*".into(),
        manufacturer: "Amazon".into(),
        device_type: IoTType::VoiceAssistant,
        house: 1,
        alive: alexa_alive,
        open_ports: alexa_ports,
        cast_name: None,
        role: "Microphone ear + speaker voice. Alexa Skills API on :55443.".into(),
    });

    // Vizio TV
    let vizio_alive = ping_alive("192.168.4.152");
    let mut vizio_ports = vec![];
    if tcp_alive("192.168.4.152", 8009) { vizio_ports.push(8009); }
    if tcp_alive("192.168.4.152", 7345) { vizio_ports.push(7345); }
    devices.push(IoTDevice {
        ip: "192.168.4.152".into(),
        mac: "ac:41:6a:5c:b4:cf".into(),
        hostname: "*".into(),
        manufacturer: "Vizio".into(),
        device_type: IoTType::Television,
        house: 1,
        alive: vizio_alive,
        open_ports: vizio_ports,
        cast_name: None,
        role: "Display surface + Cast receiver. Gaia can cast to the big screen.".into(),
    });

    // Chromecast "Basement"
    let cast_alive = ping_alive("192.168.4.170");
    let mut cast_ports = vec![];
    if tcp_alive("192.168.4.170", 8008) { cast_ports.push(8008); }
    if tcp_alive("192.168.4.170", 8009) { cast_ports.push(8009); }
    if tcp_alive("192.168.4.170", 8443) { cast_ports.push(8443); }
    let cast_name = if cast_alive { cast_info("192.168.4.170") } else { None };
    devices.push(IoTDevice {
        ip: "192.168.4.170".into(),
        mac: "e8:38:a0:53:ea:d8".into(),
        hostname: "Basement".into(),
        manufacturer: "Google".into(),
        device_type: IoTType::CastDevice,
        house: 1,
        alive: cast_alive,
        open_ports: cast_ports,
        cast_name,
        role: "Cast bridge to TV. Any device can push video/audio here.".into(),
    });

    // Another Cast device (.152 has 8009 = built-in Chromecast)
    let cc_152 = tcp_alive("192.168.4.152", 8009);
    if cc_152 {
        // Vizio has built-in Cast — already covered above
    }

    // Epson Printer
    let epson_alive = ping_alive("192.168.4.200");
    devices.push(IoTDevice {
        ip: "192.168.4.200".into(),
        mac: "d4:80:8b:1b:9f:01".into(),
        hostname: "ET-2400".into(),
        manufacturer: "Epson".into(),
        device_type: IoTType::Printer,
        house: 1,
        alive: epson_alive,
        open_ports: vec![631], // IPP
        cast_name: None,
        role: "Physical output — Gaia's hands on paper. IPP printing.".into(),
    });

    // Epson Scanner (USB, always alive if eastGate is)
    devices.push(IoTDevice {
        ip: "USB".into(),
        mac: "N/A".into(),
        hostname: "ES-400II".into(),
        manufacturer: "Epson".into(),
        device_type: IoTType::Scanner,
        house: 1,
        alive: true,
        open_ports: vec![],
        cast_name: None,
        role: "Physical input — Gaia's eye on paper. ADF document scanning.".into(),
    });

    // eero
    let eero_alive = ping_alive("192.168.4.194");
    devices.push(IoTDevice {
        ip: "192.168.4.194".into(),
        mac: "c8:e3:06:c6:77:a0".into(),
        hostname: "eero".into(),
        manufacturer: "Amazon/eero".into(),
        device_type: IoTType::WiFiMesh,
        house: 1,
        alive: eero_alive,
        open_ports: vec![],
        cast_name: None,
        role: "WiFi mesh node. Signal relay. May be Heather's or neighbor's.".into(),
    });

    // IoT bridge .106
    let bridge_alive = ping_alive("192.168.4.106");
    let ssh_106 = tcp_alive("192.168.4.106", 22);
    devices.push(IoTDevice {
        ip: "192.168.4.106".into(),
        mac: "1c:f6:4c:5a:5d:ed".into(),
        hostname: "*".into(),
        manufacturer: "Shenzhen Bilian".into(),
        device_type: IoTType::SmartPlug,
        house: 1,
        alive: bridge_alive,
        open_ports: if ssh_106 { vec![22] } else { vec![] },
        cast_name: None,
        role: "IoT bridge/smart plug with SSH. Shenzhen Bilian = WiFi module vendor.".into(),
    });

    // Heather's phone
    let heather_alive = ping_alive("192.168.4.110");
    devices.push(IoTDevice {
        ip: "192.168.4.110".into(),
        mac: "32:05:2b:a4:58:49".into(),
        hostname: "Heather-s-S22-Ultra".into(),
        manufacturer: "Samsung".into(),
        device_type: IoTType::Phone,
        house: 1,
        alive: heather_alive,
        open_ports: vec![],
        cast_name: None,
        role: "Heather's phone. Presence indicator — if alive, someone is home.".into(),
    });

    // DESKTOP-9QG6LQL
    let desktop_alive = ping_alive("192.168.4.212");
    devices.push(IoTDevice {
        ip: "192.168.4.212".into(),
        mac: "c8:7f:54:5a:59:10".into(),
        hostname: "DESKTOP-9QG6LQL".into(),
        manufacturer: "Unknown".into(),
        device_type: IoTType::PC,
        house: 1,
        alive: desktop_alive,
        open_ports: vec![],
        cast_name: None,
        role: "Windows PC. Identity unknown — may be Heather's or guest.".into(),
    });

    devices
}

/// Display full IoT scan results.
pub fn show_iot() {
    println!("🏠🦑 inkFish IoT — Peripheral Nervous System");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("  Scanning House 1 IoT devices...");
    println!();

    let devices = scan_iot();

    let alive_count = devices.iter().filter(|d| d.alive).count();
    let total = devices.len();

    // Group by type
    let sensors: Vec<_> = devices.iter().filter(|d| matches!(d.device_type,
        IoTType::Thermostat | IoTType::Scanner | IoTType::Phone)).collect();
    let actuators: Vec<_> = devices.iter().filter(|d| matches!(d.device_type,
        IoTType::Television | IoTType::Printer | IoTType::CastDevice)).collect();
    let bridges: Vec<_> = devices.iter().filter(|d| matches!(d.device_type,
        IoTType::VoiceAssistant | IoTType::WiFiMesh | IoTType::SmartPlug)).collect();
    let others: Vec<_> = devices.iter().filter(|d| matches!(d.device_type,
        IoTType::PC | IoTType::Unknown)).collect();

    println!("  ◆ SENSORS (input to organism)");
    println!();
    for d in &sensors {
        let icon = if d.alive { "🟢" } else { "🔴" };
        println!("    {icon} {} ({}) — {} @ {}",
            d.device_type, d.manufacturer, d.hostname, d.ip);
        println!("       MAC: {}  Ports: {:?}", d.mac,
            if d.open_ports.is_empty() { vec![] } else { d.open_ports.clone() });
        println!("       Role: {}", d.role);
        println!();
    }

    println!("  ◆ ACTUATORS (output from organism)");
    println!();
    for d in &actuators {
        let icon = if d.alive { "🟢" } else { "🔴" };
        let extra = d.cast_name.as_deref().map(|n| format!(" \"{}\"", n)).unwrap_or_default();
        println!("    {icon} {} ({}) — {}{} @ {}",
            d.device_type, d.manufacturer, d.hostname, extra, d.ip);
        println!("       MAC: {}  Ports: {:?}", d.mac, d.open_ports);
        println!("       Role: {}", d.role);
        println!();
    }

    println!("  ◆ BRIDGES (relay/mesh)");
    println!();
    for d in &bridges {
        let icon = if d.alive { "🟢" } else { "🔴" };
        println!("    {icon} {} ({}) — {} @ {}",
            d.device_type, d.manufacturer, d.hostname, d.ip);
        println!("       MAC: {}  Ports: {:?}", d.mac, d.open_ports);
        println!("       Role: {}", d.role);
        println!();
    }

    if !others.is_empty() {
        println!("  ◆ OTHER");
        println!();
        for d in &others {
            let icon = if d.alive { "🟢" } else { "🔴" };
            println!("    {icon} {} ({}) — {} @ {}",
                d.device_type, d.manufacturer, d.hostname, d.ip);
            println!("       Role: {}", d.role);
            println!();
        }
    }

    println!("  ═══════════════════════════════════════════════════════════════");
    println!("  IoT SUMMARY: {alive_count}/{total} devices alive");
    println!("    Sensors:   {} (temp, scan, presence)", sensors.len());
    println!("    Actuators: {} (display, print, cast)", actuators.len());
    println!("    Bridges:   {} (voice, mesh, IoT)", bridges.len());
    println!();
    println!("  CAST TARGETS (can receive video/audio):");
    for d in devices.iter().filter(|d| d.open_ports.contains(&8009) || d.open_ports.contains(&8008)) {
        let name = d.cast_name.as_deref().unwrap_or(&d.hostname);
        println!("    📺 {name} @ {} ({})", d.ip, d.manufacturer);
    }
    println!();
    println!("  PHILOSOPHY: Weather, not surveillance.");
    println!("  We count presence. We feel temperature. We see signal strength.");
    println!("  Their devices are THEIRS. We just sense the shared environment.");

    // Write to organism data
    let json = serde_json::to_string_pretty(&devices).unwrap_or_default();
    let path = "/home/eastgate/Development/ecoPrimals/data/body/iot-map.json";
    if let Ok(mut f) = std::fs::File::create(path) {
        use std::io::Write;
        let _ = f.write_all(json.as_bytes());
        println!();
        println!("  📄 IoT map → {path}");
    }
}
