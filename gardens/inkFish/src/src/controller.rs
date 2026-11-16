//! Controller module — Xbox/Switch/Steam controllers as organism HANDS.
//!
//! The controller is not just input. It's a triad device:
//! - COMPUTE: rumble motor driver (force feedback / HD rumble)
//! - DISPLAY: haptic output (vibration patterns)
//! - SIGNAL: buttons + axes + gyro/IMU = signal channels
//!
//! Plugged into eastGate USB → accessible to ANY gate via network relay.
//! inkFish translates any controller to Steam Input to organism commands.

use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::Path;

/// Controller types we support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControllerKind {
    XboxSeries,
    SwitchPro,
    SteamController,
    DualShock4,
    DualSense,
    Generic,
}

impl std::fmt::Display for ControllerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::XboxSeries => write!(f, "Xbox Series S|X"),
            Self::SwitchPro => write!(f, "Nintendo Switch Pro"),
            Self::SteamController => write!(f, "Steam Controller"),
            Self::DualShock4 => write!(f, "DualShock 4"),
            Self::DualSense => write!(f, "DualSense"),
            Self::Generic => write!(f, "Generic"),
        }
    }
}

/// Detect what controller is on js0.
fn detect_controller() -> Option<(ControllerKind, ControllerInfo)> {
    if !Path::new("/dev/input/js0").exists() {
        return None;
    }

    // Read name from sysfs — try multiple input paths
    let name = (0..40)
        .filter_map(|i| {
            let path = format!("/sys/class/input/js0/device/name");
            std::fs::read_to_string(&path).ok().map(|s| s.trim().to_string())
        })
        .next()
        .or_else(|| {
            // Fallback: read from /proc/bus/input/devices
            let devices = std::fs::read_to_string("/proc/bus/input/devices").ok()?;
            for line in devices.lines() {
                if line.contains("Pro Controller") || line.contains("Xbox") || line.contains("Steam") {
                    if line.starts_with("N: Name=") {
                        return Some(line.trim_start_matches("N: Name=").trim_matches('"').to_string());
                    }
                }
            }
            None
        })
        .unwrap_or_else(|| "Unknown".to_string());

    // Identify from USB IDs in proc
    let devices_text = std::fs::read_to_string("/proc/bus/input/devices").unwrap_or_default();
    let mut vendor = String::new();
    let mut product = String::new();
    let mut uniq = String::new();
    let mut found_section = false;

    for line in devices_text.lines() {
        if line.contains("js0") {
            found_section = true;
        }
        if found_section {
            if line.starts_with("I:") {
                // Already found js0, now look backwards
            }
        }
    }

    // Parse the device that owns js0 — walk backwards from the js0 handler line
    let lines: Vec<&str> = devices_text.lines().collect();
    let js0_line = lines.iter().position(|l| l.contains("js0"));
    if let Some(idx) = js0_line {
        // Walk backwards to find I: and U: lines for this section
        for i in (0..idx).rev() {
            let line = lines[i];
            if line.starts_with("I:") && vendor.is_empty() {
                for part in line.split_whitespace() {
                    if let Some(v) = part.strip_prefix("Vendor=") { vendor = v.to_lowercase(); }
                    if let Some(p) = part.strip_prefix("Product=") { product = p.to_lowercase(); }
                }
            }
            if line.starts_with("U: Uniq=") && uniq.is_empty() {
                uniq = line.trim_start_matches("U: Uniq=").to_string();
            }
            // Stop at blank line (section boundary)
            if line.trim().is_empty() && !vendor.is_empty() {
                break;
            }
        }
    }

    let kind = match (vendor.as_str(), product.as_str()) {
        ("057e", "2009") => ControllerKind::SwitchPro,
        ("045e", "0b12") | ("045e", "0b13") | ("045e", "0b20") => ControllerKind::XboxSeries,
        ("28de", _) => ControllerKind::SteamController,
        ("054c", "09cc") | ("054c", "05c4") => ControllerKind::DualShock4,
        ("054c", "0ce6") => ControllerKind::DualSense,
        _ => ControllerKind::Generic,
    };

    let (buttons, axes, has_imu, driver) = match kind {
        ControllerKind::SwitchPro => (14, 6, true, "hid_nintendo"),
        ControllerKind::XboxSeries => (11, 8, false, "xpad"),
        ControllerKind::SteamController => (15, 8, true, "hid_steam"),
        ControllerKind::DualShock4 => (13, 8, true, "hid_sony"),
        ControllerKind::DualSense => (13, 8, true, "hid_playstation"),
        ControllerKind::Generic => (12, 6, false, "generic"),
    };

    Some((kind, ControllerInfo {
        name,
        kind,
        vendor,
        product,
        serial: uniq,
        driver: driver.to_string(),
        device: "/dev/input/js0".into(),
        event_device: "/dev/input/event16".into(),
        imu_device: if has_imu { Some("/dev/input/event15".into()) } else { None },
        buttons,
        axes,
        has_imu,
        force_feedback: true,
    }))
}

/// Controller hardware info.
#[derive(Debug, Serialize, Deserialize)]
pub struct ControllerInfo {
    pub name: String,
    pub kind: ControllerKind,
    pub vendor: String,
    pub product: String,
    pub serial: String,
    pub driver: String,
    pub device: String,
    pub event_device: String,
    pub imu_device: Option<String>,
    pub buttons: u8,
    pub axes: u8,
    pub has_imu: bool,
    pub force_feedback: bool,
}

/// Switch Pro button names.
const SWITCH_BUTTONS: [&str; 14] = [
    "B", "A", "Y", "X", "L", "R", "ZL", "ZR",
    "Minus", "Plus", "LS", "RS", "Home", "Capture",
];

/// Switch Pro axis names.
const SWITCH_AXES: [&str; 6] = [
    "LX", "LY", "RX", "RY", "DpadX", "DpadY",
];

/// Xbox button names.
const XBOX_BUTTONS: [&str; 11] = [
    "A", "B", "X", "Y", "LB", "RB", "View", "Menu", "Xbox", "LS", "RS",
];

/// Xbox axis names.
const XBOX_AXES: [&str; 8] = [
    "LX", "LY", "LT", "RX", "RY", "RT", "DpadX", "DpadY",
];

/// Check if any controller is connected.
pub fn is_connected() -> bool {
    Path::new("/dev/input/js0").exists()
}

/// Display controller status.
pub fn show_controller() {
    println!("🎮🦑 inkFish Controller — Organism HANDS");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    match detect_controller() {
        Some((kind, info)) => {
            let icon = match kind {
                ControllerKind::SwitchPro => "🔴🔵",
                ControllerKind::XboxSeries => "🟢⬛",
                ControllerKind::SteamController => "🎮💨",
                ControllerKind::DualShock4 | ControllerKind::DualSense => "🔵⬜",
                ControllerKind::Generic => "🎮",
            };

            println!("  {icon} CONNECTED: {}", info.name);
            println!("     Type:   {kind}");
            println!("     USB:    {}:{} ({})", info.vendor, info.product, info.driver);
            println!("     BT MAC: {}", if info.serial.is_empty() { "N/A" } else { &info.serial });
            println!("     Device: {}", info.device);
            println!("     Evdev:  {}", info.event_device);
            if let Some(ref imu) = info.imu_device {
                println!("     IMU:    {} (gyroscope + accelerometer)", imu);
            }
            println!("     Buttons: {} | Axes: {} | IMU: {} | FF: {}",
                info.buttons, info.axes,
                if info.has_imu { "YES" } else { "NO" },
                if info.force_feedback { "YES" } else { "NO" });
            println!();

            // Button map
            let (btn_names, axis_names): (&[&str], &[&str]) = match kind {
                ControllerKind::SwitchPro => (&SWITCH_BUTTONS, &SWITCH_AXES),
                _ => (&XBOX_BUTTONS, &XBOX_AXES),
            };

            println!("  BUTTON MAP ({}):", kind);
            for (i, name) in btn_names.iter().enumerate() {
                println!("    [{:>2}] {}", i, name);
            }
            println!();
            println!("  AXIS MAP:");
            for (i, name) in axis_names.iter().enumerate() {
                println!("    [{:>2}] {}", i, name);
            }

            // Controller-specific features
            match kind {
                ControllerKind::SwitchPro => {
                    println!();
                    println!("  SWITCH PRO FEATURES:");
                    println!("    🌀 6-axis IMU (gyroscope + accelerometer)");
                    println!("    📳 HD Rumble (linear resonant actuator, frequency control)");
                    println!("    📡 NFC reader (amiibo support)");
                    println!("    🔋 BT capable (A0:5A:5C:0C:E2:A2) — can pair wirelessly");
                    println!("    🔌 USB-C wired (currently connected)");
                    println!();
                    println!("  CHROMATOPHORE TRANSLATION:");
                    println!("    Switch Pro → Steam Input → SDL2 gamepad mapping");
                    println!("    B↔A, A↔B swapped to match Steam convention");
                    println!("    Gyro → Steam Input gyro aim (if game supports)");
                    println!("    Capture button → screenshot / organism snapshot");
                }
                ControllerKind::XboxSeries => {
                    println!();
                    println!("  XBOX FEATURES:");
                    println!("    📳 Impulse triggers (left/right trigger vibration)");
                    println!("    📡 Xbox Wireless + BT 5.0");
                    println!("    🔌 USB-C wired");
                    println!("    ⚠️  BT needs battery (currently MISSING)");
                }
                _ => {}
            }
        }
        None => {
            println!("  ❌ NO CONTROLLER DETECTED on /dev/input/js0");
            println!();
            println!("  Plug a controller into eastGate USB-C:");
            println!("    • Nintendo Switch Pro Controller (USB-C)");
            println!("    • Xbox Series Controller (USB-C, needs battery for BT)");
            println!("    • Steam Controller (USB dongle or BT)");
            println!("    • DualShock 4 / DualSense (USB-C or BT)");
        }
    }

    println!();
    println!("  ═══════════════════════════════════════════════════════════════");
    println!("  TRIAD ROLE:");
    println!("    COMPUTE: rumble/HD-rumble motor driver (force feedback)");
    println!("    DISPLAY: haptic vibration output (organism touch sense)");
    println!("    SIGNAL:  buttons + axes + IMU = multi-channel input");
    println!();
    println!("  CROSS-STREAM PATH:");
    println!("    Kevin's thumbs → Controller → eastGate USB");
    println!("    → Steam Input → Remote Play → northGate (render)");
    println!("    → Remote Play video → frameGate (display)");
    println!("    → Solar back-channel → eastGate (ingest)");
    println!("    Full circle. Kevin's intent becomes pixels becomes data.");
}
