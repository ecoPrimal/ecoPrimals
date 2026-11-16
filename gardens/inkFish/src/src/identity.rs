//! Identity module — HOLDER/PLAYER architecture for Steam Family.

use serde::{Deserialize, Serialize};

/// A Steam identity in the organism.
#[derive(Debug, Serialize, Deserialize)]
pub struct SteamIdentity {
    /// Steam account name.
    pub account: String,
    /// Display name / persona.
    pub persona: String,
    /// Role in the organism.
    pub role: IdentityRole,
    /// Steam ID (64-bit).
    pub steam_id: u64,
    /// Account status.
    pub status: String,
}

/// Role classification in the HOLDER/PLAYER model.
#[derive(Debug, Serialize, Deserialize)]
pub enum IdentityRole {
    /// Gaia — permanent identity on all hardware. Hours active = agent metric.
    Holder,
    /// Human — transient user via Family Sharing. Play time = human metric.
    Player,
    /// Legacy — VR library contributor, games shared but account dormant.
    Legacy,
}

/// The Steam Family — gaia@gaming.
pub fn family() -> Vec<SteamIdentity> {
    vec![
        SteamIdentity {
            account: "TamisonEcoPrimal".into(),
            persona: "gaia".into(),
            role: IdentityRole::Holder,
            steam_id: 0, // ZK — not stored
            status: "Online".into(),
        },
        SteamIdentity {
            account: "gw_kevinmok".into(),
            persona: "Tamison".into(),
            role: IdentityRole::Player,
            steam_id: 76_561_198_002_558_137,
            status: "Away".into(),
        },
        SteamIdentity {
            account: "MokVive".into(),
            persona: "Napalm".into(),
            role: IdentityRole::Legacy,
            steam_id: 0, // not tracked
            status: "Offline — VR library donor".into(),
        },
        SteamIdentity {
            account: "AvatarLeo".into(),
            persona: "AvatarLeo".into(),
            role: IdentityRole::Player,
            steam_id: 0,
            status: "Invite pending".into(),
        },
    ]
}

/// Display the identity model.
pub fn show_identity() {
    println!("╔═══════════════════════════════════════════════╗");
    println!("║  inkFish Identity — gaia@gaming (4/6 slots)  ║");
    println!("╠═══════════════════════════════════════════════╣");

    for id in family() {
        let role_str = match id.role {
            IdentityRole::Holder => "HOLDER",
            IdentityRole::Player => "PLAYER",
            IdentityRole::Legacy => "LEGACY",
        };
        println!(
            "║  {:<20} {:>8}  {:<15} ║",
            id.persona, role_str, id.status
        );
    }

    println!("╠═══════════════════════════════════════════════╣");
    println!("║  Family Library: 1421 titles (9 VR)          ║");
    println!("║  HOLDER manages infra. PLAYER plays games.   ║");
    println!("║  AR = desktop + VR interweaved on Frame.     ║");
    println!("╚═══════════════════════════════════════════════╝");
}
