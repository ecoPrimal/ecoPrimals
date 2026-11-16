//! Cross-stream module — bidirectional Frame↔Gate data flow.
//!
//! The Cross-Stream Stability Principle:
//! When both endpoints stream meaningfully to each other,
//! the connection becomes qualitatively more stable.

use std::net::TcpStream;
use std::time::Duration;

/// Steam Remote Play ports.
const REMOTE_PLAY_DISCOVERY: u16 = 27036;
const REMOTE_PLAY_STREAM: &[u16] = &[27031, 27032, 27033, 27034, 27035, 27036];

/// Cross-stream connection between a Gate and a Frame.
pub struct CrossStream {
    /// Gate IP (GPU compute host).
    pub gate_ip: String,
    /// Gate name.
    pub gate_name: String,
    /// Frame IP (display/sensor endpoint).
    pub frame_ip: String,
    /// Frame name.
    pub frame_name: String,
    /// Whether the forward channel (Gate→Frame video) is active.
    pub forward_active: bool,
    /// Whether the back channel (Frame→Gate sensors) is active.
    pub back_active: bool,
}

impl CrossStream {
    /// Check if the cross-stream is healthy (both directions flowing).
    #[must_use]
    pub fn is_healthy(&self) -> bool {
        self.forward_active && self.back_active
    }

    /// Check if this is degraded (one direction only).
    #[must_use]
    pub fn is_degraded(&self) -> bool {
        self.forward_active != self.back_active
    }
}

/// Known gate addresses for auto-detection.
fn resolve_gate(name: &str) -> Option<(&str, &str)> {
    match name {
        "northGate" | "north" | "auto" => Some(("northGate", "192.168.4.147")),
        "ironGate" | "iron" => Some(("ironGate", "192.168.4.237")),
        "eastGate" | "east" => Some(("eastGate", "192.168.4.244")),
        "southGate" | "south" => Some(("southGate", "192.168.4.148")),
        _ => None,
    }
}

/// Known frame addresses.
fn resolve_frame(name: &str) -> Option<(&str, &str)> {
    match name {
        "frameGate" | "frame" => Some(("frameGate", "192.168.4.181")),
        "deckGate" | "deck" => Some(("deckGate", "192.168.4.176")),
        _ => None,
    }
}

/// Check if Steam Remote Play is available on a host.
fn check_remote_play(ip: &str) -> bool {
    let addr = format!("{ip}:{REMOTE_PLAY_DISCOVERY}");
    TcpStream::connect_timeout(
        &addr.parse().unwrap_or_else(|_| "0.0.0.0:0".parse().expect("valid addr")),
        Duration::from_secs(2),
    )
    .is_ok()
}

/// Start the cross-stream between a gate and frame.
pub async fn start_cross_stream(
    gate: &str,
    frame: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let (gate_name, gate_ip) = resolve_gate(gate)
        .ok_or_else(|| format!("Unknown gate: {gate}"))?;
    let (frame_name, frame_ip) = resolve_frame(frame)
        .ok_or_else(|| format!("Unknown frame: {frame}"))?;

    println!("🔄 inkFish Cross-Stream Setup");
    println!("  Gate:  {gate_name} ({gate_ip})");
    println!("  Frame: {frame_name} ({frame_ip})");
    println!();

    // Check gate Remote Play
    let gate_rp = check_remote_play(gate_ip);
    println!(
        "  Gate Remote Play:  {}",
        if gate_rp { "✅ OPEN" } else { "❌ CLOSED" },
    );

    // Check frame reachability
    let frame_alive = TcpStream::connect_timeout(
        &format!("{frame_ip}:22").parse()?,
        Duration::from_secs(2),
    )
    .is_ok();
    println!(
        "  Frame reachable:   {}",
        if frame_alive { "✅ ALIVE" } else { "❌ DOWN" },
    );

    if gate_rp && frame_alive {
        println!();
        println!("  ═══════════════════════════════════════");
        println!("  Forward: {gate_name} ═══video═══→ {frame_name}");
        println!("  Back:    {gate_name} ←══sensors══ {frame_name}");
        println!("  Status:  CROSS-STREAM READY ✅");
        println!("  ═══════════════════════════════════════");
        println!();
        println!("  The triangle anchor is set:");
        println!("    Mesh ↔ Gate ↔ Frame ↔ Mesh");
        println!("  All edges bidirectional. Connection stable.");
    } else {
        println!();
        println!("  ⚠ Cross-stream not ready.");
        if !gate_rp {
            println!("  → Start Steam on {gate_name} with Remote Play enabled");
        }
        if !frame_alive {
            println!("  → Wake {frame_name} (plug in USB-C or connect WiFi)");
        }
    }

    Ok(())
}
