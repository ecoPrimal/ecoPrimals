//! Solar Panel module — Frame ingests sensors when idle.
//!
//! The Frame has two modes:
//! - LANDER 🚀 — active gaming, RT cores render OUT
//! - SOLAR PANEL ☀️ — idle sensing, all signal is INPUT
//!
//! When Kevin picks up Frame → LANDER.
//! When Kevin sets it down → SOLAR PANEL.
//! Accelerometer detects the mode change.

use serde::{Deserialize, Serialize};

/// Solar panel sensor reading from the Frame.
#[derive(Debug, Serialize, Deserialize)]
pub struct SolarReading {
    /// Timestamp of the reading.
    pub timestamp: String,
    /// WiFi signal strength (dBm).
    pub wifi_rssi: Option<i32>,
    /// Connected WiFi SSID.
    pub wifi_ssid: Option<String>,
    /// CPU temperature in Celsius.
    pub cpu_temp_c: Option<f32>,
    /// Battery percentage.
    pub battery_percent: Option<u8>,
    /// Battery charging status.
    pub battery_status: Option<String>,
    /// Ambient audio level in dB.
    pub audio_db: Option<f32>,
    /// Accelerometer readings.
    pub accel: Option<(f32, f32, f32)>,
    /// Nearby devices count (ARP table).
    pub nearby_devices: Option<u32>,
    /// Current operating mode.
    pub mode: FrameMode,
}

/// Frame operating mode.
#[derive(Debug, Serialize, Deserialize)]
pub enum FrameMode {
    /// Active gaming — RT cores render outward.
    Lander,
    /// Idle sensing — all sensors ingest inward.
    SolarPanel,
}

/// Sensor endpoints on the Frame.
pub struct SensorPaths {
    /// Thermal zones.
    pub thermal: &'static str,
    /// Battery capacity.
    pub battery: &'static str,
    /// WiFi interface.
    pub wifi: &'static str,
}

/// Default sensor paths for Steam Frame (Snapdragon 8 Gen 3).
pub const FRAME_SENSORS: SensorPaths = SensorPaths {
    thermal: "/sys/class/thermal/thermal_zone0/temp",
    battery: "/sys/class/power_supply/BAT0/capacity",
    wifi: "wlan0",
};

/// Run the solar panel mode (monitoring and reporting).
pub async fn run_solar() -> Result<(), Box<dyn std::error::Error>> {
    println!("☀️ inkFish Solar Panel Mode");
    println!("═══════════════════════════════════════");
    println!();
    println!("  The Frame has TWO modes:");
    println!();
    println!("  🚀 LANDER (active):");
    println!("     Display → OUT (video, audio, haptics)");
    println!("     GPU → RENDERING (RT cores trace rays outward)");
    println!("     Network → PULLING (streaming from Gate)");
    println!();
    println!("  ☀️ SOLAR PANEL (idle):");
    println!("     Camera → IN (light levels, presence, movement)");
    println!("     Microphone → IN (ambient sound, voice detect)");
    println!("     WiFi RSSI → IN (spatial field, triangulation)");
    println!("     Accelerometer → IN (vibration, orientation)");
    println!("     Touch screen → IN (capacitive proximity field)");
    println!("     GPU → COMPUTING (RT cores process sensor data)");
    println!("     Network → PUSHING (sensor data to eastGate)");
    println!();
    println!("  Same silicon, inverted data flow.");
    println!("  All signal is input.");
    println!("  The Frame IS a solar panel when idle.");
    println!();
    println!("  Sensor feed: POST http://192.168.4.244:8084/solar");
    println!("  Deploy: curl http://192.168.4.244:5926/frame-inject/deploy.sh | bash");

    Ok(())
}
