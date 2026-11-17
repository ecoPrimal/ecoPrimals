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
mod ludo_bridge;
mod mirror;
mod iot;
mod discoball;
mod redox;

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
    /// Show ludoSpring ↔ KenshiReborn bridge — science → game → delivery.
    Bridge,
    /// Scan all beacons — BT + WiFi, write to organism data.
    Scan {
        /// BT scan duration in seconds.
        #[arg(long, default_value = "10")]
        duration: u32,
    },
    /// Mirror — pull northGate game library, push streams to Frame.
    Mirror,
    /// Scan IoT devices — Nest, Alexa, Vizio, Chromecast, printer, eero.
    Iot,
    /// Inverted discoball — chiral Flint lighthouses, null spine, black hole sinks.
    Discoball,
    /// Run KenshiReborn scan — parse all game meshes/textures on local data.
    Kenshi {
        /// Subcommand: scan, flow, render.
        #[arg(default_value = "scan")]
        mode: String,
    },
    /// Full organism redox — regenerate ALL state, deploy to alive gates.
    Redox,
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
        Cli::Bridge => ludo_bridge::show_bridge(),
        Cli::Scan { duration } => beacon::run_scan(duration).await?,
        Cli::Mirror => mirror::run_mirror().await?,
        Cli::Iot => iot::show_iot(),
        Cli::Discoball => discoball::show_discoball(),
        Cli::Redox => redox::run_redox().await?,
        Cli::Kenshi { mode } => {
            let kenshi_bin = std::env::current_exe()?
                .parent()
                .unwrap_or(std::path::Path::new("."))
                .join("kenshi-reborn");
            if kenshi_bin.exists() {
                let flag = match mode.as_str() {
                    "scan" => "--scan",
                    "flow" => "--flow",
                    _ => "",
                };
                let mut cmd = std::process::Command::new(&kenshi_bin);
                if !flag.is_empty() { cmd.arg(flag); }
                let status = cmd.status()?;
                if !status.success() {
                    eprintln!("kenshi-reborn exited with: {status}");
                }
            } else {
                eprintln!("kenshi-reborn binary not found at {}", kenshi_bin.display());
                eprintln!("Build it: cd pin/esotericWebb/KenshiReborn/engine/kenshi-reborn && cargo build --release");
            }
        }
    }

    Ok(())
}
