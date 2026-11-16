use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{IpAddr, Ipv4Addr, TcpStream};
use std::process::Command;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Piton {
    name: String,
    ip: String,
    latency_ms: f64,
    state: PitonState,
    method: String,
    services: Vec<ServiceRope>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
enum PitonState {
    Anchored,
    Missed,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ServiceRope {
    name: String,
    port: u16,
    alive: bool,
    latency_ms: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct PitonMap {
    system: String,
    timestamp: String,
    wave: String,
    piton_count_anchored: usize,
    piton_count_missed: usize,
    pitons: HashMap<String, Piton>,
    walls_crossable: Vec<Wall>,
    fleet: Option<FleetStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Wall {
    wall: String,
    pitons: Vec<String>,
    rope: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FleetStatus {
    proxy_port: u16,
    unique_models: u32,
    total_instances: u32,
    gates_alive: u32,
}

struct GateConfig {
    ip: &'static str,
    services: Vec<(&'static str, u16)>,
}

fn ping(ip: &str) -> Option<f64> {
    let output = Command::new("ping")
        .args(["-c", "1", "-W", "1", ip])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Some(pos) = line.find("time=") {
            let rest = &line[pos + 5..];
            if let Some(end) = rest.find(' ') {
                return rest[..end].parse().ok();
            }
            return rest.trim().trim_end_matches("ms").parse().ok();
        }
    }
    None
}

fn test_tcp(ip: &str, port: u16) -> (bool, f64) {
    let addr = format!("{}:{}", ip, port);
    let start = Instant::now();
    match TcpStream::connect_timeout(
        &addr.parse().unwrap_or_else(|_| {
            std::net::SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)
        }),
        Duration::from_secs(2),
    ) {
        Ok(_) => (true, start.elapsed().as_secs_f64() * 1000.0),
        Err(_) => (false, 0.0),
    }
}

fn hammer_piton(name: &str, config: &GateConfig) -> Piton {
    let latency = ping(config.ip);
    let (state, latency_ms) = match latency {
        Some(ms) => (PitonState::Anchored, ms),
        None => (PitonState::Missed, 999.0),
    };

    let mut method = if state == PitonState::Anchored {
        "icmp".to_string()
    } else {
        "none".to_string()
    };

    if name == "Pixel" {
        let adb = Command::new("adb").args(["devices"]).output();
        if let Ok(out) = adb {
            let s = String::from_utf8_lossy(&out.stdout);
            if s.contains("44251JEKB04957") {
                method = "adb+wifi".to_string();
            }
        }
    }

    if name == "milkGate" {
        let adb = Command::new("adb").args(["devices"]).output();
        if let Ok(out) = adb {
            let s = String::from_utf8_lossy(&out.stdout);
            if s.contains("HW4MPK3321280122") {
                method = "adb+usb".to_string();
            }
        }
    }

    let services: Vec<ServiceRope> = config
        .services
        .iter()
        .filter(|_| state == PitonState::Anchored)
        .map(|(svc_name, port)| {
            let (alive, lat) = test_tcp(config.ip, *port);
            ServiceRope {
                name: svc_name.to_string(),
                port: *port,
                alive,
                latency_ms: (lat * 10.0).round() / 10.0,
            }
        })
        .collect();

    let emoji = if state == PitonState::Anchored {
        "🪨"
    } else {
        "✗ "
    };
    let svc_str = services
        .iter()
        .filter(|s| s.alive)
        .map(|s| format!("{}:{}", s.name, s.port))
        .collect::<Vec<_>>()
        .join(" ");
    eprintln!(
        "  {} {:12} {:16} {:>7.1}ms  {:10} {}",
        emoji, name, config.ip, latency_ms, method, svc_str
    );

    Piton {
        name: name.to_string(),
        ip: config.ip.to_string(),
        latency_ms: (latency_ms * 10.0).round() / 10.0,
        state,
        method,
        services,
    }
}

fn fetch_fleet() -> Option<FleetStatus> {
    let stream = TcpStream::connect_timeout(
        &"127.0.0.1:11435".parse().unwrap(),
        Duration::from_secs(2),
    )
    .ok()?;
    let mut stream = stream;
    stream
        .write_all(b"GET /fleet HTTP/1.0\r\nHost: localhost\r\n\r\n")
        .ok()?;
    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .ok()?;

    let reader = BufReader::new(&stream);
    let mut body = String::new();
    let mut in_body = false;
    for line in reader.lines() {
        let line = line.ok()?;
        if in_body {
            body.push_str(&line);
        }
        if line.is_empty() {
            in_body = true;
        }
    }

    let v: serde_json::Value = serde_json::from_str(&body).ok()?;
    let unique_models = v["unique_models"].as_u64().unwrap_or(0) as u32;
    let total_instances = v["total_instances"].as_u64().unwrap_or(0) as u32;
    let gates = v["gates"].as_object()?;
    let gates_alive = gates
        .values()
        .filter(|g| g["alive"].as_bool().unwrap_or(false))
        .count() as u32;

    eprintln!(
        "  🧠 Fleet: {} models, {} instances, {} gates alive",
        unique_models, total_instances, gates_alive
    );

    Some(FleetStatus {
        proxy_port: 11435,
        unique_models,
        total_instances,
        gates_alive,
    })
}

fn main() {
    let gates: Vec<(&str, GateConfig)> = vec![
        (
            "Router",
            GateConfig {
                ip: "192.168.4.1",
                services: vec![("ssh", 22), ("luci", 80)],
            },
        ),
        (
            "MikroTik",
            GateConfig {
                ip: "192.168.4.2",
                services: vec![],
            },
        ),
        (
            "sporeGate",
            GateConfig {
                ip: "192.168.4.3",
                services: vec![("ssh", 22)],
            },
        ),
        (
            "Flint2",
            GateConfig {
                ip: "192.168.4.9",
                services: vec![("ssh", 22), ("luci", 80)],
            },
        ),
        (
            "Pixel",
            GateConfig {
                ip: "192.168.4.135",
                services: vec![],
            },
        ),
        (
            "milkGate",
            GateConfig {
                ip: "192.168.4.144",
                services: vec![("ssh", 22)],
            },
        ),
        (
            "northGate",
            GateConfig {
                ip: "192.168.4.147",
                services: vec![("ollama", 11434)],
            },
        ),
        (
            "southGate",
            GateConfig {
                ip: "192.168.4.148",
                services: vec![("ollama", 11434), ("ssh", 22)],
            },
        ),
        (
            "Epson",
            GateConfig {
                ip: "192.168.4.200",
                services: vec![("ipp", 631)],
            },
        ),
        (
            "ironGate",
            GateConfig {
                ip: "192.168.4.237",
                services: vec![("ollama", 11434)],
            },
        ),
        (
            "graftGate",
            GateConfig {
                ip: "192.168.4.106",
                services: vec![("ollama", 11434)],
            },
        ),
        (
            "frameGate",
            GateConfig {
                ip: "192.168.4.181",
                services: vec![("steam", 27036)],
            },
        ),
        (
            "swiftGate",
            GateConfig {
                ip: "192.168.4.100",
                services: vec![],
            },
        ),
    ];

    eprintln!("╔═══════════════════════════════════════════╗");
    eprintln!("║  🧗 PITON v0.1.0 (Rust) — Hammering...    ║");
    eprintln!("╚═══════════════════════════════════════════╝");
    eprintln!();

    let mut pitons = HashMap::new();
    for (name, config) in &gates {
        let piton = hammer_piton(name, config);
        pitons.insert(name.to_string(), piton);
    }

    let fleet = fetch_fleet();

    let anchored = pitons
        .values()
        .filter(|p| p.state == PitonState::Anchored)
        .count();
    let missed = pitons
        .values()
        .filter(|p| p.state == PitonState::Missed)
        .count();

    eprintln!();
    eprintln!("  ⛰️  {} anchored, {} missed", anchored, missed);

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let timestamp = format!("{}Z", ts.as_secs());

    let map = PitonMap {
        system: "GAIA PITON LINE (Rust)".to_string(),
        timestamp,
        wave: "239p".to_string(),
        piton_count_anchored: anchored,
        piton_count_missed: missed,
        pitons,
        walls_crossable: vec![
            Wall {
                wall: "House1↔House2".to_string(),
                pitons: vec![
                    "Flint2".into(),
                    "frameGate".into(),
                    "Pixel".into(),
                ],
                rope: "10G fiber + WiFi".to_string(),
            },
            Wall {
                wall: "LAN↔WAN".to_string(),
                pitons: vec!["Router".into(), "milkGate".into()],
                rope: "NAT hairpin + SFP+ WAN (future)".to_string(),
            },
            Wall {
                wall: "LAN↔Cellular".to_string(),
                pitons: vec!["Pixel".into()],
                rope: "5G NR + WiFi bridge".to_string(),
            },
            Wall {
                wall: "WiFi↔Wired".to_string(),
                pitons: vec!["Pixel".into(), "milkGate".into()],
                rope: "ADB USB dual-stack".to_string(),
            },
        ],
        fleet,
    };

    let json = serde_json::to_string_pretty(&map).expect("Failed to serialize piton map");
    println!("{}", json);
}
