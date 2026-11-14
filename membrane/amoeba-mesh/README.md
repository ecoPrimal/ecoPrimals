# Amoeba Mesh — RustDesk Config

## Quick Setup (any gate)

1. Install RustDesk
2. Copy the config for your gate to `~/.config/rustdesk/RustDesk2.toml`
3. Set permanent password: `rustdesk --password d14247c1680e3085`
4. Restart: `sudo systemctl restart rustdesk`

## Peers

| Gate | ID | IP | Role |
|---|---|---|---|
| eastGate | 19091278 | .244 | RELAY + BRAIN |
| sporeGate | 1871544996 | .3 | EYE + EAR |
| northGate | ??? | .147 | HEART + CHAIR |
| ironGate | pending | .237 | FORGE 2 |

## Relay

- Host: 192.168.4.244
- HBBS: :21116
- HBBR: :21117
- Direct: :21118
- Key: (none — LAN trust)
- Password: d14247c1680e3085
