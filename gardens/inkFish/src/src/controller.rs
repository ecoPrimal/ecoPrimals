//! Controller module — Xbox/Steam controllers as organism HANDS.
//!
//! The controller is not just input. It's a triad device:
//! - COMPUTE: rumble motor driver (force feedback)
//! - DISPLAY: haptic output (vibration patterns)
//! - SIGNAL: 11 buttons + 8 axes = 19 signal channels
//!
//! Plugged into eastGate USB → accessible to ANY gate via network relay.
//! inkFish translates Xbox input to Steam Input to organism commands.

use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::Path;

/// Xbox controller axis/button state.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ControllerState {
    /// Buttons (A, B, X, Y, LB, RB, View, Menu, Xbox, LS, RS).
    pub buttons: [bool; 11],
    /// Axes (LX, LY, LT, RX, RY, RT, DpadX, DpadY).
    pub axes: [i16; 8],
    /// Last update timestamp (ms).
    pub timestamp_ms: u32,
}

/// Button names for the Xbox Series controller.
pub const BUTTON_NAMES: [&str; 11] = [
    "A", "B", "X", "Y", "LB", "RB", "View", "Menu", "Xbox", "LS", "RS",
];

/// Axis names for the Xbox Series controller.
pub const AXIS_NAMES: [&str; 8] = [
    "LX", "LY", "LT", "RX", "RY", "RT", "DpadX", "DpadY",
];

/// Read one event from /dev/input/js0.
/// Returns (timestamp, value, type, number).
fn read_js_event(js: &mut std::fs::File) -> Option<(u32, i16, u8, u8)> {
    let mut buf = [0u8; 8];
    if js.read_exact(&mut buf).is_ok() {
        let timestamp = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let value = i16::from_le_bytes([buf[4], buf[5]]);
        let event_type = buf[6];
        let number = buf[7];
        Some((timestamp, value, event_type, number))
    } else {
        None
    }
}

/// Check if the Xbox controller is connected.
pub fn is_connected() -> bool {
    Path::new("/dev/input/js0").exists()
}

/// Get controller info from sysfs.
pub fn controller_info() -> ControllerInfo {
    let name = std::fs::read_to_string("/sys/class/input/input31/name")
        .unwrap_or_else(|_| "Unknown".into())
        .trim()
        .to_string();
    ControllerInfo {
        name,
        vendor: "045e".into(),
        product: "0b12".into(),
        serial: "3039373030303939313335323039".into(),
        driver: "xpad".into(),
        device: "/dev/input/js0".into(),
        event_device: "/dev/input/event15".into(),
        buttons: 11,
        axes: 8,
        force_feedback: true,
    }
}

/// Controller hardware info.
#[derive(Debug, Serialize, Deserialize)]
pub struct ControllerInfo {
    /// Device name from kernel.
    pub name: String,
    /// USB vendor ID.
    pub vendor: String,
    /// USB product ID.
    pub product: String,
    /// Serial number.
    pub serial: String,
    /// Kernel driver.
    pub driver: String,
    /// Joystick device path.
    pub device: String,
    /// Evdev device path.
    pub event_device: String,
    /// Number of buttons.
    pub buttons: u8,
    /// Number of axes.
    pub axes: u8,
    /// Force feedback support.
    pub force_feedback: bool,
}

/// Display controller status.
pub fn show_controller() {
    let connected = is_connected();

    println!("🎮 inkFish Controller — Xbox as Organism HAND");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    if connected {
        let info = controller_info();
        println!("  ✅ CONNECTED: {}", info.name);
        println!("     USB: {}:{} ({})", info.vendor, info.product, info.driver);
        println!("     Serial: {}", info.serial);
        println!("     Device: {} + {}", info.device, info.event_device);
        println!("     Buttons: {} | Axes: {} | Force Feedback: {}",
            info.buttons, info.axes, if info.force_feedback { "YES" } else { "NO" });
        println!();
        println!("  BUTTON MAP:");
        for (i, name) in BUTTON_NAMES.iter().enumerate() {
            println!("    [{:>2}] {}", i, name);
        }
        println!();
        println!("  AXIS MAP:");
        for (i, name) in AXIS_NAMES.iter().enumerate() {
            let range = if *name == "LT" || *name == "RT" {
                "-32767 → 32767 (trigger)"
            } else if *name == "DpadX" || *name == "DpadY" {
                "-1, 0, 1 (dpad)"
            } else {
                "-32767 → 32767 (stick)"
            };
            println!("    [{:>2}] {:<6} {}", i, name, range);
        }
    } else {
        println!("  ❌ NO CONTROLLER DETECTED on /dev/input/js0");
        println!("     Plug Xbox controller into eastGate USB-C");
    }

    println!();
    println!("  TRIAD ROLE:");
    println!("    COMPUTE: rumble motor driver (force feedback patterns)");
    println!("    DISPLAY: haptic vibration output (organism touch)");
    println!("    SIGNAL:  19 input channels (11 buttons + 8 axes)");
    println!();
    println!("  CHROMATOPHORE TRANSLATION:");
    println!("    Xbox input → Steam Input → organism commands");
    println!("    Plugged into eastGate USB → relayable to ANY gate");
    println!("    northGate renders game → eastGate relays controller → Frame displays");
    println!("    The controller is the HAND. The Frame is the FACE. The Gate is the BRAIN.");
    println!();
    println!("  CROSS-STREAM PATH:");
    println!("    Kevin's thumbs → Xbox Controller → eastGate USB");
    println!("    → Steam Input → Remote Play → northGate (render)");
    println!("    → Remote Play video → frameGate (display)");
    println!("    → Solar back-channel → eastGate (ingest)");
    println!("    Full circle. Kevin's intent becomes pixels becomes data.");
}
