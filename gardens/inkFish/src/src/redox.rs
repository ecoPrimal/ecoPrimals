//! Redox module — full organism regeneration sweep.
//!
//! Scans every reachable gate, enumerates GPUs, Ollama endpoints,
//! BT radios, Steam state, and writes the complete organism state
//! to `data/body/organism-redox-live.json`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::TcpStream;
use std::process::Command;
use std::time::{Duration, SystemTime};

/// A gate's live status.
#[derive(Debug, Serialize, Deserialize)]
struct GateStatus {
    name: String,
    ip: String,
    role: String,
    alive: bool,
    latency_ms: Option<f64>,
    gpu: Option<String>,
    ollama_models: Option<u32>,
    ollama_port: Option<u16>,
    bt_up: bool,
    bt_mac: Option<String>,
    steam: bool,
    steamvr: bool,
    remote_play: bool,
    sunshine: bool,
    inkfish_deployed: bool,
    arch: String,
}

/// Ollama model info.
#[derive(Debug, Serialize, Deserialize)]
struct OllamaModel {
    name: String,
    size_gb: f64,
}

/// Full redox state.
#[derive(Debug, Serialize, Deserialize)]
struct RedoxState {
    timestamp: String,
    scanner: String,
    gates: Vec<GateStatus>,
    total_rt_cores: u32,
    active_rt_cores: u32,
    total_vram_gb: u32,
    ollama_total_models: u32,
    bt_radios_up: u32,
    steam_titles: u32,
    arm64_binaries: Vec<String>,
    frame_manifest_ready: bool,
}

/// Run a command and return stdout.
fn run(cmd: &str) -> String {
    Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

/// Ping a host and return latency in ms.
fn ping_ms(ip: &str) -> Option<f64> {
    let out = run(&format!("ping -c 1 -W 2 {ip} 2>/dev/null"));
    out.lines()
        .find(|l| l.contains("time="))
        .and_then(|l| {
            l.split("time=")
                .nth(1)
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
        })
}

/// Check if a TCP port is open.
fn port_open(ip: &str, port: u16) -> bool {
    let addr = format!("{ip}:{port}");
    TcpStream::connect_timeout(
        &addr
            .parse()
            .unwrap_or_else(|_| "0.0.0.0:0".parse().expect("valid")),
        Duration::from_secs(2),
    )
    .is_ok()
}

/// Count Ollama models at an endpoint.
fn count_ollama(ip: &str, port: u16) -> u32 {
    let out = run(&format!(
        "curl -s --connect-timeout 2 http://{ip}:{port}/api/tags 2>/dev/null"
    ));
    serde_json::from_str::<serde_json::Value>(&out)
        .ok()
        .and_then(|v| v["models"].as_array().map(|a| a.len() as u32))
        .unwrap_or(0)
}

/// Run the full organism redox sweep.
pub async fn run_redox() -> Result<(), Box<dyn std::error::Error>> {
    let now = humantime::format_rfc3339(SystemTime::now()).to_string();

    println!("⚡ inkFish REDOX — Full Organism Regeneration Sweep");
    println!("═══════════════════════════════════════════════════════");
    println!("  {now}");
    println!();

    let gate_defs: Vec<(&str, &str, &str, &str, u32, u32, u16)> = vec![
        // (name, ip, role, gpu, rt_cores, vram_gb, ollama_port)
        (
            "eastGate",
            "192.168.4.244",
            "BRAIN",
            "RTX 4070",
            46,
            12,
            11436,
        ),
        (
            "northGate",
            "192.168.4.147",
            "HEAD",
            "RTX 5090",
            192,
            32,
            11434,
        ),
        (
            "ironGate",
            "192.168.4.237",
            "SHOULDER",
            "RTX 5070",
            70,
            12,
            11434,
        ),
        (
            "southGate",
            "192.168.4.148",
            "LEG",
            "RTX 4060",
            34,
            8,
            11434,
        ),
        (
            "milkGate",
            "192.168.4.144",
            "LEG",
            "RISC-V NPU",
            0,
            0,
            11434,
        ),
        (
            "sporeGate",
            "192.168.4.3",
            "LEFT HAND",
            "Radeon iGPU",
            0,
            0,
            0,
        ),
        (
            "frameGate",
            "192.168.4.181",
            "FACE",
            "Adreno 750",
            0,
            0,
            11434,
        ),
        (
            "swiftGate",
            "192.168.4.100",
            "HEART (H2)",
            "RTX 3070",
            46,
            8,
            11434,
        ),
    ];

    let mut gates = Vec::new();
    let mut total_rt: u32 = 0;
    let mut active_rt: u32 = 0;
    let mut total_vram: u32 = 0;
    let mut total_ollama: u32 = 0;
    let mut bt_up: u32 = 0;

    for (name, ip, role, gpu, rt, vram, ollama_port) in &gate_defs {
        print!("  Probing {name:12} ({ip})... ");

        let latency = ping_ms(ip);
        let alive = latency.is_some();

        let ollama_count = if alive && *ollama_port > 0 {
            count_ollama(ip, *ollama_port)
        } else {
            0
        };

        let remote_play = if alive {
            port_open(ip, 27036)
        } else {
            false
        };
        let sunshine = if alive {
            port_open(ip, 47990)
        } else {
            false
        };

        let status_icon = if alive { "🟢" } else { "⬛" };
        let lat_str = latency.map_or("---".into(), |l| format!("{l:.1}ms"));
        println!(
            "{status_icon} {lat_str:>8} | Ollama:{ollama_count:>3} | RP:{} | Sun:{}",
            if remote_play { "✅" } else { "❌" },
            if sunshine { "☀️" } else { "❌" },
        );

        total_rt += rt;
        if alive {
            active_rt += rt;
        }
        total_vram += vram;
        total_ollama += ollama_count;

        gates.push(GateStatus {
            name: (*name).to_string(),
            ip: (*ip).to_string(),
            role: (*role).to_string(),
            alive,
            latency_ms: latency,
            gpu: Some((*gpu).to_string()),
            ollama_models: Some(ollama_count),
            ollama_port: if *ollama_port > 0 {
                Some(*ollama_port)
            } else {
                None
            },
            bt_up: false, // filled below for local
            bt_mac: None,
            steam: false,
            steamvr: false,
            remote_play,
            sunshine,
            inkfish_deployed: *name == "eastGate" || *name == "southGate",
            arch: if *name == "milkGate" {
                "riscv64".into()
            } else if *name == "frameGate" {
                "aarch64".into()
            } else {
                "x86_64".into()
            },
        });
    }

    // Local BT check
    let bt_out = run("hciconfig 2>/dev/null");
    if bt_out.contains("UP RUNNING") {
        bt_up += 1;
        if let Some(g) = gates.iter_mut().find(|g| g.name == "eastGate") {
            g.bt_up = true;
            let mac_line = run("hciconfig hci0 2>/dev/null | grep 'BD Address'");
            g.bt_mac = mac_line
                .split_whitespace()
                .find(|w| w.contains(':') && w.len() == 17)
                .map(String::from);
        }
    }

    // Local Steam check
    let has_steam = std::path::Path::new("/usr/games/steam").exists()
        || std::path::Path::new("/usr/bin/steam").exists();
    let has_steamvr = std::path::Path::new(
        &format!(
            "{}/.steam/steam/steamapps/common/SteamVR",
            std::env::var("HOME").unwrap_or_default()
        ),
    )
    .exists();
    if let Some(g) = gates.iter_mut().find(|g| g.name == "eastGate") {
        g.steam = has_steam;
        g.steamvr = has_steamvr;
    }

    // ARM64 binaries
    let arm64_bins: Vec<String> = vec![
        "gardens/inkFish/target/aarch64-unknown-linux-gnu/release/inkfish".into(),
        "springs/rustChip/target/aarch64-unknown-linux-musl/release/flint-silicon".into(),
    ];
    let arm64_exists: Vec<String> = arm64_bins
        .into_iter()
        .filter(|p| {
            std::path::Path::new(&format!(
                "/home/eastgate/Development/ecoPrimals/{p}"
            ))
            .exists()
        })
        .collect();

    // Frame manifest
    let frame_manifest = std::path::Path::new(
        "/home/eastgate/Development/ecoPrimals/data/body/frame-regeneration-manifest.json",
    )
    .exists();

    let alive_count = gates.iter().filter(|g| g.alive).count();
    let total_count = gates.len();

    println!();
    println!("═══════════════════════════════════════════════════════");
    println!("  ORGANISM REDOX SUMMARY");
    println!("═══════════════════════════════════════════════════════");
    println!("  Gates:     {alive_count}/{total_count} alive");
    println!("  RT Cores:  {active_rt}/{total_rt} active");
    println!("  VRAM:      {total_vram} GB across fleet");
    println!("  Ollama:    {total_ollama} models across fleet");
    println!("  BT:        {bt_up} radios UP");
    println!("  Steam:     {} | SteamVR: {}", has_steam, has_steamvr);
    println!(
        "  ARM64:     {} binaries ready for Frame",
        arm64_exists.len()
    );
    println!("  Frame:     manifest {}", if frame_manifest { "READY" } else { "MISSING" });
    println!("═══════════════════════════════════════════════════════");

    let state = RedoxState {
        timestamp: now,
        scanner: "eastGate".into(),
        gates,
        total_rt_cores: total_rt,
        active_rt_cores: active_rt,
        total_vram_gb: total_vram,
        ollama_total_models: total_ollama,
        bt_radios_up: bt_up,
        steam_titles: 1421,
        arm64_binaries: arm64_exists,
        frame_manifest_ready: frame_manifest,
    };

    let out_path = "/home/eastgate/Development/ecoPrimals/data/body/organism-redox-live.json";
    let json = serde_json::to_string_pretty(&state)?;
    std::fs::write(out_path, &json)?;
    println!();
    println!("  ✅ Written: {out_path}");

    Ok(())
}
