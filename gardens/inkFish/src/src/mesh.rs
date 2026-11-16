//! Mesh module — Steam presence detection across gates.

use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::time::Duration;

/// A gate in the organism with Steam capability.
#[derive(Debug, Serialize, Deserialize)]
pub struct SteamGate {
    /// Gate name.
    pub name: String,
    /// IP address on LAN.
    pub ip: String,
    /// GPU model.
    pub gpu: String,
    /// VRAM in GB.
    pub vram_gb: u32,
    /// RT core count.
    pub rt_cores: u32,
    /// RT generation.
    pub rt_gen: u8,
    /// Whether the gate is reachable.
    pub alive: bool,
    /// Whether Steam Remote Play is detected.
    pub remote_play: bool,
}

/// All known gates with GPU/Steam capability.
pub fn gates() -> Vec<SteamGate> {
    vec![
        SteamGate {
            name: "northGate".into(),
            ip: "192.168.4.147".into(),
            gpu: "RTX 5090".into(),
            vram_gb: 32,
            rt_cores: 192,
            rt_gen: 4,
            alive: false,
            remote_play: false,
        },
        SteamGate {
            name: "ironGate".into(),
            ip: "192.168.4.237".into(),
            gpu: "RTX 5070".into(),
            vram_gb: 12,
            rt_cores: 70,
            rt_gen: 4,
            alive: false,
            remote_play: false,
        },
        SteamGate {
            name: "eastGate".into(),
            ip: "192.168.4.244".into(),
            gpu: "RTX 4070".into(),
            vram_gb: 12,
            rt_cores: 46,
            rt_gen: 3,
            alive: false,
            remote_play: false,
        },
        SteamGate {
            name: "southGate".into(),
            ip: "192.168.4.148".into(),
            gpu: "RTX 4060".into(),
            vram_gb: 8,
            rt_cores: 34,
            rt_gen: 3,
            alive: false,
            remote_play: false,
        },
        SteamGate {
            name: "swiftGate".into(),
            ip: "192.168.4.100".into(),
            gpu: "RTX 3070".into(),
            vram_gb: 8,
            rt_cores: 46,
            rt_gen: 2,
            alive: false,
            remote_play: false,
        },
    ]
}

/// Probe a gate for Steam presence.
fn probe(gate: &mut SteamGate) {
    let timeout = Duration::from_secs(1);
    // Check SSH (alive)
    gate.alive = TcpStream::connect_timeout(
        &format!("{}:22", gate.ip).parse().unwrap_or_else(|_| "0.0.0.0:0".parse().expect("valid addr")),
        timeout,
    )
    .is_ok();

    // Check Steam Remote Play (port 27036)
    if gate.alive {
        gate.remote_play = TcpStream::connect_timeout(
            &format!("{}:27036", gate.ip).parse().unwrap_or_else(|_| "0.0.0.0:0".parse().expect("valid addr")),
            timeout,
        )
        .is_ok();
    }
}

/// Show live mesh status.
pub async fn show_mesh() -> Result<(), Box<dyn std::error::Error>> {
    let mut all_gates = gates();
    let mut total_rt = 0u32;
    let mut alive_count = 0u32;

    println!("🎮 inkFish Mesh — Steam Gates");
    println!("═══════════════════════════════════════════════════════");

    for gate in &mut all_gates {
        probe(gate);
        let status = if gate.remote_play {
            "✅ STEAM-RP"
        } else if gate.alive {
            "🟡 ALIVE"
        } else {
            "❌ DOWN"
        };

        if gate.alive {
            alive_count += 1;
            total_rt += gate.rt_cores;
        }

        println!(
            "  {:<12} .{:<4} {:<10} {:>3} RT{} {:>4}GB  {}",
            gate.name,
            gate.ip.split('.').last().unwrap_or("?"),
            gate.gpu,
            gate.rt_cores,
            gate.rt_gen,
            gate.vram_gb,
            status,
        );
    }

    println!("═══════════════════════════════════════════════════════");
    println!(
        "  {} gates alive | {} RT cores active | Family: gaia@gaming",
        alive_count, total_rt,
    );

    Ok(())
}

/// Show full organism gaming topology.
pub async fn show_topology() -> Result<(), Box<dyn std::error::Error>> {
    show_mesh().await?;

    println!();
    println!("🏠 HOUSE 1 (Polaris)         10G SPINE         🏠 HOUSE 2 (South Pole)");
    println!("  eastGate (BRAIN)     ═══════════════════     milkGate (RISC-V)");
    println!("  northGate (HEART)         ~80m fiber          southGate (RTX 4060)");
    println!("  frameGate (FACE)          1.67ms RT           swiftGate (💤)");
    println!("  sporeGate (FORGE)                             Flint 2 (ear)");
    println!();
    println!("🥽 AR Interweave: Desktop + VR on Frame's dual 2160×2160 panels");
    println!("☀️ Solar Panel: Frame ingests sensors when idle");
    println!("🔄 Cross-Stream: bidirectional Gate↔Frame = stable");

    Ok(())
}
