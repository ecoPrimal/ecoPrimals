//! Beacon module — active BT + WiFi signal scanning.
//!
//! This is inkFish's SIGNAL LINE executing. Not just displaying —
//! actually scanning, collecting, and writing beacon data to the organism.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use std::time::SystemTime;

/// A detected BT beacon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BtBeacon {
    /// MAC address.
    pub mac: String,
    /// Device name (if advertised).
    pub name: Option<String>,
    /// RSSI in dBm (signal strength = distance proxy).
    pub rssi: Option<i32>,
    /// Tx Power (if advertised).
    pub tx_power: Option<i32>,
    /// Manufacturer data key (0x004c = Apple, 0x0006 = Microsoft, etc).
    pub manufacturer: Option<String>,
    /// Whether this is a known organism device.
    pub organism: bool,
    /// Timestamp of detection.
    pub seen_at: String,
}

/// Known organism BT MACs.
fn organism_macs() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("C8:5E:A9:C9:6F:84", "eastGate");
    m.insert("90:82:C3:74:12:CB", "frameGate");
    m.insert("E8:D5:2B:51:36:39", "Pixel 8a");
    m.insert("D6:80:8B:1B:9F:01", "Epson ET-2400");
    m.insert("10:F6:0A:54:57:D0", "ironGate");
    m.insert("C8:A3:E8:FB:94:4A", "southGate");
    m.insert("C0:8B:27:9C:63:ED", "milkGate");
    m
}

/// Run a shell command and return stdout.
fn run(cmd: &str) -> String {
    Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default()
}

/// Scan BT beacons from this device.
pub fn scan_bt(duration_secs: u32) -> Vec<BtBeacon> {
    let known = organism_macs();

    // Start scan
    let _ = run("bluetoothctl scan off 2>/dev/null; sleep 0.5");
    let _ = run(&format!(
        "timeout {} bluetoothctl scan on 2>/dev/null &",
        duration_secs
    ));
    std::thread::sleep(std::time::Duration::from_secs(u64::from(duration_secs)));
    let _ = run("bluetoothctl scan off 2>/dev/null");

    // Collect devices
    let devices_out = run("bluetoothctl devices 2>/dev/null");
    let mut beacons = Vec::new();
    let now = humantime::format_rfc3339(SystemTime::now()).to_string();

    for line in devices_out.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 || parts[0] != "Device" {
            continue;
        }
        let mac = parts[1].to_string();
        let name_raw = parts[2..].join(" ");

        // Get detailed info
        let info = run(&format!("bluetoothctl info {} 2>/dev/null", mac));
        let mut rssi = None;
        let mut tx_power = None;
        let mut manufacturer = None;
        let mut real_name = None;

        for iline in info.lines() {
            let trimmed = iline.trim();
            if let Some(rest) = trimmed.strip_prefix("Name: ") {
                real_name = Some(rest.to_string());
            }
            if let Some(rest) = trimmed.strip_prefix("RSSI: ") {
                rssi = rest.trim().parse().ok();
            }
            if let Some(rest) = trimmed.strip_prefix("TxPower: ") {
                tx_power = rest.trim().parse().ok();
            }
            if trimmed.contains("ManufacturerData Key:") {
                manufacturer = trimmed.split_whitespace().last().map(String::from);
            }
        }

        let display_name = real_name.or_else(|| {
            if name_raw.contains('-') && name_raw.len() == 17 {
                None // looks like a MAC
            } else {
                Some(name_raw.clone())
            }
        });

        let is_organism = known.contains_key(mac.as_str());

        beacons.push(BtBeacon {
            mac,
            name: display_name,
            rssi,
            tx_power,
            manufacturer,
            organism: is_organism,
            seen_at: now.clone(),
        });
    }

    beacons
}

/// Scan WiFi networks visible from this device.
pub fn scan_wifi() -> Vec<WifiBeacon> {
    let scan = run("iw dev wlan0 scan 2>/dev/null || iwlist wlan0 scan 2>/dev/null");
    let mut beacons = Vec::new();
    let mut current: Option<WifiBeacon> = None;

    for line in scan.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("BSS ") || trimmed.starts_with("Cell ") {
            if let Some(b) = current.take() {
                beacons.push(b);
            }
            let mac = trimmed
                .split_whitespace()
                .find(|w| w.contains(':') && w.len() >= 17)
                .unwrap_or("")
                .trim_end_matches(')')
                .to_string();
            current = Some(WifiBeacon {
                bssid: mac,
                ssid: None,
                signal_dbm: None,
                channel: None,
                frequency_mhz: None,
            });
        }
        if let Some(ref mut b) = current {
            if let Some(rest) = trimmed.strip_prefix("SSID: ") {
                b.ssid = Some(rest.to_string());
            }
            if trimmed.contains("signal:") {
                b.signal_dbm = trimmed
                    .split("signal:")
                    .nth(1)
                    .and_then(|s| s.trim().split_whitespace().next())
                    .and_then(|s| s.parse().ok());
            }
            if let Some(rest) = trimmed.strip_prefix("freq: ") {
                b.frequency_mhz = rest.trim().parse().ok();
            }
            if trimmed.contains("Channel:") || trimmed.contains("channel") {
                b.channel = trimmed
                    .split(':')
                    .last()
                    .and_then(|s| s.trim().parse().ok());
            }
        }
    }
    if let Some(b) = current {
        beacons.push(b);
    }

    beacons
}

/// A detected WiFi beacon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiBeacon {
    /// BSSID (AP MAC).
    pub bssid: String,
    /// SSID (network name).
    pub ssid: Option<String>,
    /// Signal strength in dBm.
    pub signal_dbm: Option<f32>,
    /// Channel number.
    pub channel: Option<u32>,
    /// Frequency in MHz.
    pub frequency_mhz: Option<u32>,
}

/// Full beacon scan result.
#[derive(Debug, Serialize, Deserialize)]
pub struct BeaconScan {
    /// Timestamp.
    pub timestamp: String,
    /// Which gate performed the scan.
    pub scanner: String,
    /// BT beacons found.
    pub bt: Vec<BtBeacon>,
    /// WiFi beacons found.
    pub wifi: Vec<WifiBeacon>,
}

/// Run a full beacon scan and write results to the organism.
pub async fn run_scan(duration: u32) -> Result<(), Box<dyn std::error::Error>> {
    println!("📡 inkFish Beacon Scan — eating all signal");
    println!("  BT scan duration: {}s", duration);
    println!();

    println!("  Scanning BT...");
    let bt = scan_bt(duration);
    println!("  Found {} BT devices", bt.len());

    for b in &bt {
        let org = if b.organism { " 🟢" } else { "" };
        println!(
            "    {} {:<25} RSSI:{:>5} MFR:{}{}",
            b.mac,
            b.name.as_deref().unwrap_or("(unknown)"),
            b.rssi.map_or("?".into(), |r| format!("{r}")),
            b.manufacturer.as_deref().unwrap_or("?"),
            org,
        );
    }

    println!();
    println!("  Scanning WiFi...");
    let wifi = scan_wifi();
    println!("  Found {} WiFi networks", wifi.len());

    for w in &wifi {
        println!(
            "    {} {:<25} {:>5}dBm ch:{}",
            w.bssid,
            w.ssid.as_deref().unwrap_or("(hidden)"),
            w.signal_dbm.map_or("?".into(), |s| format!("{s}")),
            w.channel.map_or("?".into(), |c| format!("{c}")),
        );
    }

    // Write to organism data
    let scan = BeaconScan {
        timestamp: humantime::format_rfc3339(SystemTime::now()).to_string(),
        scanner: "eastGate".into(),
        bt,
        wifi,
    };

    let data_path = "/home/eastgate/Development/ecoPrimals/data/body/beacon-scan.json";
    let json = serde_json::to_string_pretty(&scan)?;
    std::fs::write(data_path, &json)?;
    println!();
    println!("  ✅ Written to {data_path}");

    // Also append to JSONL feed
    let feed_path = "/home/eastgate/Development/ecoPrimals/data/body/beacon-feed.jsonl";
    let compact = serde_json::to_string(&scan)?;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(feed_path)?;
    use std::io::Write;
    writeln!(file, "{compact}")?;
    println!("  ✅ Appended to {feed_path}");

    let organism_count = scan.bt.iter().filter(|b| b.organism).count();
    let neighbor_count = scan.bt.len() - organism_count;
    println!();
    println!(
        "  📡 {} organism beacons | {} neighbors (count not track) | {} WiFi APs",
        organism_count,
        neighbor_count,
        scan.wifi.len(),
    );

    Ok(())
}
