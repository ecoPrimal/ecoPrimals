//! # inkFish — Sovereign Steam/VR Gaming Sovereignty Layer
//!
//! Like `blueFish` exercises wetSpring's PFAS pipeline via BYOB composition,
//! `inkFish` exercises ludoSpring's game science via the Steam ecosystem.
//!
//! ## Architecture: HOLDER and PLAYER
//!
//! - **HOLDER** (Gaia/TamisonEcoPrimal): Persistent identity on all Steam hardware.
//!   The organism manages the infrastructure. Hours active = agent metric.
//!
//! - **PLAYER** (Humans): Transient users of Gaia's hardware.
//!   Play time on their account = human metric. Always just players.
//!
//! ## The Cross-Stream Stability Principle
//!
//! Bidirectional Frame↔Gate streaming creates stable connections.
//! AR = desktop game (flat) + VR game (stereo) interweaved on one display.
//! gamescope `--backend openvr` IS the AR compositor.

mod identity;
mod mesh;
mod streaming;
mod ar_interweave;
mod solar;
mod parity;
mod triad;
mod controller;
mod beacon;

use clap::Parser;

/// inkFish — sovereign Steam/VR gaming sovereignty layer.
#[derive(Parser)]
#[command(name = "inkfish", version, about)]
enum Cli {
    /// Show mesh status — which gates have Steam, what's streaming.
    Mesh,
    /// Show AR interweave pairs — desktop+VR game combinations.
    Interweave,
    /// Show identity — HOLDER/PLAYER accounts and family.
    Identity,
    /// Start cross-stream daemon — bidirectional Frame↔Gate flow.
    Stream {
        /// Gate to stream from (e.g. northGate, ironGate).
        #[arg(long, default_value = "auto")]
        gate: String,
        /// Frame to stream to.
        #[arg(long, default_value = "frameGate")]
        frame: String,
    },
    /// Solar panel mode — Frame ingests sensors when idle.
    Solar,
    /// Show the full organism gaming topology.
    Topology,
    /// Show parity scores — how close each device is to Steam-grade.
    Parity,
    /// Show triad — COMPUTE × DISPLAY × SIGNAL for every device.
    Triad,
    /// Show controller status — Xbox pad as organism HAND.
    Controller,
    /// Scan all beacons — BT + WiFi, write to organism data.
    Scan {
        /// BT scan duration in seconds.
        #[arg(long, default_value = "10")]
        duration: u32,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("inkfish=info")
        .init();

    let cli = Cli::parse();

    match cli {
        Cli::Mesh => mesh::show_mesh().await?,
        Cli::Interweave => ar_interweave::show_pairs(),
        Cli::Identity => identity::show_identity(),
        Cli::Stream { gate, frame } => streaming::start_cross_stream(&gate, &frame).await?,
        Cli::Solar => solar::run_solar().await?,
        Cli::Topology => mesh::show_topology().await?,
        Cli::Parity => parity::show_parity(),
        Cli::Triad => triad::show_triad(),
        Cli::Controller => controller::show_controller(),
        Cli::Scan { duration } => beacon::run_scan(duration).await?,
    }

    Ok(())
}
