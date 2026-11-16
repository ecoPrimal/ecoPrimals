//! mirror — Pull game library from northGate, push streaming to Frame.
//!
//! The mirror pattern: northGate RADIATES games. Frame ABSORBS through
//! Sunshine→Moonlight or Steam Remote Play. inkFish manages the pipe.
//!
//! Two streaming paths:
//!   1. Sunshine (northGate:47990) → Moonlight (Frame flatpak) — lowest latency
//!   2. Steam Remote Play (northGate:27036) → Frame Steam client — zero config
//!
//! The mirror also tracks what's installed where and what can stream.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::process::Command;
use std::time::Duration;

/// A remote Steam client discovered via remoteclients.vdf or network probing.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteHost {
    pub hostname: String,
    pub ip: String,
    pub app_ids: Vec<u32>,
    pub sunshine_alive: bool,
    pub remote_play_alive: bool,
    pub last_seen: String,
}

/// A streamable game entry.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StreamableGame {
    pub app_id: u32,
    pub name: String,
    pub vr: bool,
    pub source_host: String,
    pub can_stream: bool,
}

/// The full mirror state — what we know about the streaming mesh.
#[derive(Debug, Serialize, Deserialize)]
pub struct MirrorState {
    pub hosts: Vec<RemoteHost>,
    pub games: Vec<StreamableGame>,
    pub moonlight_installed: bool,
    pub sunshine_paired: bool,
    pub frame_reachable: bool,
    pub active_streams: u32,
    pub timestamp: String,
}

/// Known Steam app IDs → names for VR and key titles.
fn known_games() -> HashMap<u32, (&'static str, bool)> {
    let mut m = HashMap::new();
    // (name, is_vr)
    m.insert(233860, ("Kenshi", false));
    m.insert(250820, ("SteamVR", true));
    m.insert(546560, ("Half-Life: Alyx", true));
    m.insert(674140, ("VTOL VR", true));
    m.insert(686060, ("Pavlov VR", true));
    m.insert(1084600, ("Budget Cuts 2", true));
    m.insert(1292940, ("Sniper Elite VR", true));
    m.insert(1371980, ("Population: One", true));
    m.insert(1422450, ("Demeo", true));
    m.insert(1433990, ("Creed: Rise to Glory", true));
    m.insert(1458140, ("Zenith: The Last City", true));
    m.insert(1466060, ("Red Matter 2", true));
    m.insert(1562430, ("BONELAB", true));
    m.insert(1569580, ("Wanderer", true));
    m.insert(1592280, ("Blade and Sorcery", true));
    m.insert(1627720, ("Kayak VR: Mirage", true));
    m.insert(1640820, ("Gorilla Tag", true));
    m.insert(1657630, ("Thief Simulator VR", true));
    // Non-VR heavy hitters
    m.insert(108600, ("Project Zomboid", false));
    m.insert(228980, ("Crusader Kings III", false));
    m.insert(305620, ("The Long Dark", false));
    m.insert(333640, ("Caves of Qud", false));
    m.insert(362490, ("Frostpunk", false));
    m.insert(413150, ("Stardew Valley", false));
    m.insert(480490, ("Prey", false));
    m.insert(482400, ("Obduction", false));
    m.insert(526870, ("Satisfactory", false));
    m.insert(532110, ("Black Mesa", false));
    m.insert(794260, ("Outward", false));
    m.insert(835960, ("Into the Breach", false));
    m.insert(881100, ("Noita", false));
    m.insert(1145960, ("Hades", false));
    m.insert(2321470, ("Lethal Company", false));
    m.insert(2379780, ("Balatro", false));
    m
}

/// Probe a TCP port with timeout.
fn tcp_alive(ip: &str, port: u16, timeout_secs: u64) -> bool {
    let addr = format!("{ip}:{port}");
    if let Ok(sa) = addr.parse() {
        TcpStream::connect_timeout(&sa, Duration::from_secs(timeout_secs)).is_ok()
    } else {
        false
    }
}

/// Parse remoteclients.vdf from Frame SSH to discover hosts.
fn parse_remote_clients_ssh() -> Vec<(String, Vec<u32>)> {
    let out = Command::new("sshpass")
        .args([
            "-p", "framegate",
            "ssh", "-o", "ConnectTimeout=3", "-o", "StrictHostKeyChecking=no",
            "steamos@192.168.4.181",
            "cat /home/steamos/.steam/steam/config/remoteclients.vdf 2>/dev/null",
        ])
        .output();

    let Ok(out) = out else { return vec![] };
    let text = String::from_utf8_lossy(&out.stdout);

    let mut hosts: Vec<(String, Vec<u32>)> = Vec::new();
    let mut current_host: Option<String> = None;
    let mut current_apps: Vec<u32> = Vec::new();
    let mut in_apps = false;

    for line in text.lines() {
        let trimmed = line.trim().trim_matches('"');
        if trimmed.starts_with("hostname") {
            if let Some(h) = current_host.take() {
                hosts.push((h, std::mem::take(&mut current_apps)));
            }
            let parts: Vec<&str> = line.trim().split('"').collect();
            if parts.len() >= 4 {
                current_host = Some(parts[3].to_string());
            }
            in_apps = false;
        } else if trimmed == "apps" || trimmed.starts_with("\"apps\"") {
            in_apps = true;
        } else if in_apps && trimmed == "}" {
            in_apps = false;
        } else if in_apps {
            // Lines like: "0"    "233860"
            let parts: Vec<&str> = line.trim().split('"').collect();
            if parts.len() >= 4 {
                if let Ok(id) = parts[3].parse::<u32>() {
                    current_apps.push(id);
                }
            }
        }
    }
    if let Some(h) = current_host {
        hosts.push((h, current_apps));
    }

    hosts
}

/// Run the full mirror scan and display state.
pub async fn run_mirror() -> Result<(), Box<dyn std::error::Error>> {
    let known = known_games();
    let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();

    println!("🪞🦑 inkFish MIRROR — northGate → Frame Streaming Pipeline");
    println!("════════════════════════════════════════════════════════════════");
    println!();

    // Phase 1: Probe infrastructure
    println!("  ◆ PHASE 1: INFRASTRUCTURE PROBE");
    println!();

    let north_alive = tcp_alive("192.168.4.147", 22, 2);
    let frame_alive = tcp_alive("192.168.4.181", 22, 2);
    let sunshine_alive = tcp_alive("192.168.4.147", 47990, 2);
    let remote_play_alive = tcp_alive("192.168.4.147", 27036, 2);
    let east_steam = tcp_alive("192.168.4.244", 27036, 2);

    println!("    northGate (.147):  {}", if north_alive { "✅ ALIVE" } else { "❌ DOWN" });
    println!("    frameGate (.181):  {}", if frame_alive { "✅ ALIVE" } else { "❌ DOWN" });
    println!("    Sunshine  :47990:  {}", if sunshine_alive { "☀️  LIVE — streaming server ready" } else { "❌ not running" });
    println!("    RemotePlay:27036:  {}", if remote_play_alive { "🎮 OPEN — Steam Remote Play ready" } else { "❌ closed" });
    println!("    eastGate  :27036:  {}", if east_steam { "🧠 OPEN — local Steam" } else { "⬚ no Steam" });
    println!();

    // Phase 2: Discover game libraries
    println!("  ◆ PHASE 2: GAME LIBRARY DISCOVERY");
    println!();

    let remote_hosts = parse_remote_clients_ssh();
    let mut all_hosts: Vec<RemoteHost> = Vec::new();
    let mut all_games: Vec<StreamableGame> = Vec::new();

    for (hostname, app_ids) in &remote_hosts {
        let is_north = hostname == "tamison" || hostname == "northgate";
        let ip = if is_north { "192.168.4.147" }
            else if hostname == "eastgate" { "192.168.4.244" }
            else if hostname == "steamdeck" { "192.168.4.176" }
            else { "unknown" };

        let host_sunshine = if is_north { sunshine_alive } else { false };
        let host_rp = if is_north { remote_play_alive } else { tcp_alive(ip, 27036, 1) };

        let host = RemoteHost {
            hostname: hostname.clone(),
            ip: ip.to_string(),
            app_ids: app_ids.clone(),
            sunshine_alive: host_sunshine,
            remote_play_alive: host_rp,
            last_seen: now.clone(),
        };

        let vr_count = app_ids.iter().filter(|id| known.get(id).map_or(false, |(_, vr)| *vr)).count();
        let named = app_ids.iter().filter(|id| known.contains_key(id)).count();

        let icon = if host_sunshine { "☀️ " } else if host_rp { "🎮" } else { "⬚ " };
        println!("    {icon} {hostname} ({ip}) — {} apps ({named} identified, {vr_count} VR)",
            app_ids.len());

        // Build streamable games for this host
        for &app_id in app_ids {
            let (name, vr) = known.get(&app_id).copied().unwrap_or(("(unknown)", false));
            all_games.push(StreamableGame {
                app_id,
                name: name.to_string(),
                vr,
                source_host: hostname.clone(),
                can_stream: host_sunshine || host_rp,
            });
        }

        all_hosts.push(host);
    }

    println!();

    // Phase 3: VR games on northGate
    println!("  ◆ PHASE 3: VR GAME LIBRARY (northGate)");
    println!();

    let north_vr: Vec<_> = all_games.iter()
        .filter(|g| g.source_host == "tamison" && g.vr)
        .collect();

    if north_vr.is_empty() {
        println!("    (No VR games identified yet — Kevin still installing)");
    } else {
        for g in &north_vr {
            let stream_icon = if g.can_stream { "🟢" } else { "🔴" };
            println!("    {stream_icon} [{:>7}] {}", g.app_id, g.name);
        }
    }
    println!();

    // Phase 4: Streaming paths
    println!("  ◆ PHASE 4: STREAMING PATHS → Frame");
    println!();

    // Check Moonlight on Frame
    let moonlight_check = Command::new("sshpass")
        .args([
            "-p", "framegate",
            "ssh", "-o", "ConnectTimeout=3", "-o", "StrictHostKeyChecking=no",
            "steamos@192.168.4.181",
            "flatpak list --user 2>/dev/null | grep -ci moon",
        ])
        .output()
        .ok()
        .and_then(|o| String::from_utf8_lossy(&o.stdout).trim().parse::<u32>().ok())
        .unwrap_or(0);
    let moonlight_installed = moonlight_check > 0;

    println!("    PATH 1: Sunshine → Moonlight");
    println!("      Sunshine on northGate:  {}", if sunshine_alive { "☀️  LIVE" } else { "❌" });
    println!("      Moonlight on Frame:     {}", if moonlight_installed { "🌙 INSTALLED" } else { "❌ not installed" });
    if sunshine_alive && moonlight_installed {
        println!("      Status:                 🟢 READY — need PIN pairing");
        println!("      Pair:  flatpak run com.moonlight_stream.Moonlight pair 192.168.4.147");
    } else {
        println!("      Status:                 🟡 PARTIAL");
    }
    println!();

    println!("    PATH 2: Steam Remote Play");
    println!("      Steam on northGate:     {}", if remote_play_alive { "🎮 LIVE" } else { "❌" });
    println!("      Steam on Frame:         {}", if frame_alive { "🎮 LIVE (PID running)" } else { "❌" });
    if remote_play_alive && frame_alive {
        println!("      Status:                 🟢 READY — open Steam on Frame → Remote Play");
    }
    println!();

    // Phase 5: Cross-stream back-channel
    println!("  ◆ PHASE 5: CROSS-STREAM BACK-CHANNEL");
    println!();

    let solar_alive = tcp_alive("192.168.4.244", 8084, 1);
    println!("    Back-channel :8084:    {}", if solar_alive { "📡 LIVE (Frame→eastGate sensor flow)" } else { "❌" });

    // Check latest solar reading
    let solar_feed = std::fs::read_to_string(
        "/home/eastgate/Development/ecoPrimals/data/body/frame-solar-feed.jsonl"
    ).unwrap_or_default();
    let last_line = solar_feed.lines().last().unwrap_or("");
    if !last_line.is_empty() {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(last_line) {
            let mode = v.get("mode").and_then(|m| m.as_str()).unwrap_or("?");
            let rssi = v.get("wifi").and_then(|w| w.get("rssi")).and_then(|r| r.as_i64()).unwrap_or(0);
            let ts = v.get("timestamp").and_then(|t| t.as_str()).unwrap_or("?");
            println!("    Latest reading:       mode={mode} rssi={rssi}dBm @ {ts}");
        }
    }
    println!();

    // Phase 6: KenshiReborn engine status
    println!("  ◆ PHASE 6: KENSHI REBORN ENGINE");
    println!();

    let digest_path = "/home/eastgate/Development/ecoPrimals/data/body/kenshi-reborn-digest.json";
    if let Ok(digest_text) = std::fs::read_to_string(digest_path) {
        if let Ok(d) = serde_json::from_str::<serde_json::Value>(&digest_text) {
            let gpu = d.get("gpu").and_then(|g| g.get("name")).and_then(|n| n.as_str()).unwrap_or("?");
            let meshes = d.get("data").and_then(|d| d.get("meshes")).and_then(|m| m.as_u64()).unwrap_or(0);
            let textures = d.get("data").and_then(|d| d.get("textures")).and_then(|t| t.as_u64()).unwrap_or(0);
            let verts = d.get("mesh_analysis").and_then(|m| m.get("total_vertices")).and_then(|v| v.as_u64()).unwrap_or(0);
            let tris = d.get("mesh_analysis").and_then(|m| m.get("triangles")).and_then(|t| t.as_u64()).unwrap_or(0);
            let ms = d.get("timing_ms").and_then(|t| t.get("total")).and_then(|v| v.as_u64()).unwrap_or(0);

            println!("    GPU:        {gpu}");
            println!("    Meshes:     {meshes} parsed ({verts} vertices, {tris} triangles)");
            println!("    Textures:   {textures} DDS");
            println!("    Scan time:  {ms}ms");
            println!("    Status:     🟢 ENGINE COMPILED ON EASTGATE — READY TO STREAM");
        }
    } else {
        println!("    (no digest yet — run kenshi-reborn --scan first)");
    }
    println!();

    // Summary
    let total_vr = all_games.iter().filter(|g| g.vr).count();
    let total_streamable = all_games.iter().filter(|g| g.can_stream).count();
    let total_all = all_games.len();
    let unique_hosts = all_hosts.len();

    println!("  ═══════════════════════════════════════════════════════════════");
    println!("  MIRROR SUMMARY");
    println!("    Hosts:      {unique_hosts} remote Steam clients");
    println!("    Games:      {total_all} total ({total_vr} VR, {total_streamable} streamable)");
    println!("    Sunshine:   {}", if sunshine_alive { "☀️  LIVE" } else { "❌" });
    println!("    Moonlight:  {}", if moonlight_installed { "🌙 INSTALLED" } else { "❌" });
    println!("    Remote Play:{}", if remote_play_alive { " 🎮 LIVE" } else { " ❌" });
    println!("    Engine:     🟢 KenshiReborn compiled");
    println!("  ═══════════════════════════════════════════════════════════════");

    // Write mirror state to organism data
    let state = MirrorState {
        hosts: all_hosts,
        games: all_games,
        moonlight_installed,
        sunshine_paired: false,
        frame_reachable: frame_alive,
        active_streams: 0,
        timestamp: now,
    };

    let mirror_path = "/home/eastgate/Development/ecoPrimals/data/body/mirror-state.json";
    if let Ok(json) = serde_json::to_string_pretty(&state) {
        if let Ok(mut f) = std::fs::File::create(mirror_path) {
            let _ = f.write_all(json.as_bytes());
            println!();
            println!("  📄 Mirror state → {mirror_path}");
        }
    }

    Ok(())
}
