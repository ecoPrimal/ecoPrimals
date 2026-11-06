# GAIA — Hardware Atlas & Compute Mathematics

> *"The organism fills in the blanks as she sails her own waters"*  
> Last measured: 2026-11-06 Wave 225

---

## Nervous System — 5 GPUs, 2 Houses, 1 Organism

### CNS — Central Nervous System (LAN 1: 4422 Southgate Ave)

| Node | Role | CPU | GPU | VRAM | RAM | Storage | Latency |
|------|------|-----|-----|------|-----|---------|---------|
| **eastGate** (.244) | BRAIN | AMD (details TBD) | **RTX 4070** | 12GB | ~32GB | ~1TB | 0ms |
| **northGate** (.147) | HEART | **AMD 9950X3D** (16C/32T) | **RTX 5090** | 32GB | **96GB** | ~2TB | 0.29ms |
| **ironGate** (.237) | **FORGE 2** | **Intel i9-14900K** (24C/32T, 6GHz) | **RTX 5070** | **12GB** | **94GB** | **3.6TB NVMe + 12.7TB ext** | 0.27ms |
| sporeGate (.3) | FORGE | Ryzen 5 6600H | integrated | — | 27GB | 929GB NVMe | 0.14ms |
| graftGate (.107) | OPEN | **Apple M4** (10C) | M4 GPU (10C) | shared | **16GB** | 228GB | 0.35ms |

### PNS — Peripheral Nervous System (LAN 2: 641 Samantha Ave)

| Node | Role | CPU | GPU | VRAM | RAM | Storage | Latency |
|------|------|-----|-----|------|-----|---------|---------|
| **grapheneGate** (.9.217) | DIAMOND LENS | **Tensor G3** (9C) | **Mali-G715 MP7** (10C, 890MHz) | shared | 8GB | 128GB (103GB free) | 145ms |
| **swiftGate** (ASLEEP) | LAN 2 POLARIS | unknown | **RTX 3070** | 8GB | TBD | TBD | — |

---

## Compute Mathematics

### Total GPU FLOPS (Theoretical FP16)

| GPU | FLOPS (FP16) | VRAM | TDP | Location |
|-----|-------------|------|-----|----------|
| RTX 5090 | ~209 TFLOPS | 32GB GDDR7 | 575W | northGate |
| RTX 5070 | ~125 TFLOPS | 12GB GDDR7 | 250W | ironGate |
| RTX 4070 | ~73 TFLOPS | 12GB GDDR6X | 200W | eastGate |
| RTX 3070 | ~40 TFLOPS | 8GB GDDR6 | 220W | swiftGate (asleep) |
| Mali-G715 MP7 | ~1.5 TFLOPS | shared | ~5W | grapheneGate |
| M4 GPU | ~8 TFLOPS | shared 16GB | ~15W | graftGate |
| **TOTAL** | **~456.5 TFLOPS** | **64GB dedicated + 24GB shared** | **~1265W peak** | |
| **AWAKE NOW** | **~416.5 TFLOPS** | **56GB + 24GB** | **~1045W** | |

### Total RAM

| Node | RAM | Type | Speed |
|------|-----|------|-------|
| northGate | 96GB | DDR5 | ~5600MHz |
| ironGate | 94GB | DDR5 | TBD |
| eastGate | ~32GB | DDR5 | TBD |
| sporeGate | 27GB | DDR5 | TBD |
| graftGate | 16GB | Unified | LPDDR5 |
| grapheneGate | 8GB | LPDDR5X | 4266MHz |
| **TOTAL** | **~273GB** | | |

### Total Storage

| Node | NVMe | External | Total |
|------|------|----------|-------|
| ironGate | 3.6TB | 12.7TB (HDD) + 1.8TB + 931GB + 931GB | **~19.9TB** |
| northGate | ~2TB | — | ~2TB |
| eastGate | ~1TB | — | ~1TB |
| sporeGate | 929GB | — | 929GB |
| graftGate | 228GB | — | 228GB |
| grapheneGate | 128GB | — | 128GB |
| **TOTAL** | **~7.9TB NVMe + 15.4TB ext** | | **~24.2TB** |

### Total CPU Threads

| Node | CPU | Cores | Threads | Max GHz |
|------|-----|-------|---------|---------|
| ironGate | i9-14900K | 24 (8P+16E) | 32 | 6.0 |
| northGate | 9950X3D | 16 | 32 | ~5.7 |
| graftGate | Apple M4 | 10 (4P+6E) | 10 | ~4.4 |
| grapheneGate | Tensor G3 | 9 (1+4+4) | 9 | 3.0 |
| sporeGate | Ryzen 5 6600H | 6 | 12 | 4.5 |
| eastGate | AMD TBD | ~8 | ~16 | TBD |
| **TOTAL** | | **~73** | **~111** | |

---

## Physical Bodies

### grapheneGate — Diamond Lens (Pixel 8a)

| Dimension | Value |
|-----------|-------|
| Height | 152.1 mm (5.99 in) |
| Width | 72.7 mm (2.86 in) |
| Depth | 8.9 mm (0.35 in) |
| Weight | 188g (6.63 oz) |
| Volume | 98.41 cm³ (5.98 in³) |
| Density | 1.91 g/cm³ |
| Screen | 6.1" OLED, 1080×2400, 120Hz, 2000 nits peak |
| Screen area | 90.3 cm² (81.6% STB ratio) |
| Glass | Corning Gorilla Glass 3 |
| Frame | Aluminum alloy |
| Back | Plastic |
| Rating | IP67 (1m, 30min) |
| Color | Obsidian |
| Battery | 4492 mAh Li-Ion |
| Charging | Fast + Wireless (Qi) |
| WiFi | WiFi 6E (6GHz capable) |
| Bluetooth | 5.3 |
| Camera | 64MP (OIS) + 13MP ultra + 13MP front |
| Sensors | 37 (accel, gyro, mag, baro, prox, light, GPS, NFC, UWB-no) |
| USB | USB 3.2 Gen1 Type-C (PD, HDMI, DP) |
| Cellular | 5G Sub-6 / 4G / 3G / 2G + eSIM |

**Derived metrics:**
- Surface area: ~271.8 cm²
- Pixels/gram: 13,787
- Inference/gram: 0.188 tok/s/g (qwen2.5-0.5b)
- Power density: 45.6 mAh/cm³
- FLOPS/gram: ~8.0 GFLOPS/g
- Memory/volume: 1.30 GB/cm³
- Compute efficiency: 7.1 tok/s/W (estimated)

### ironGate — NZXT? / Tower

| Dimension | Value |
|-----------|-------|
| Case | Pop!_OS workstation |
| CPU | Intel i9-14900K (24C/32T, 6.0GHz boost) |
| GPU | NVIDIA RTX 5070 (12GB GDDR7) |
| RAM | 94GB DDR5 |
| Storage | 3.6TB NVMe + 12.7TB ext + 1.8TB + 2×931GB |
| Network | Wired Ethernet (.237) |
| Services | Ollama, Docker, biomeOS, SSH |
| Power | est. ~500W under load |

### swiftGate — NZXT H1 V2 (ASLEEP)

| Dimension | Value |
|-----------|-------|
| Case | NZXT H1 V2 Mini-ITX |
| Height | 405 mm (15.9 in) |
| Width | 196 mm (7.7 in) |
| Depth | 196 mm (7.7 in) |
| Weight | 7.6 kg (16.7 lb) |
| Volume | 15,559 cm³ |
| PSU | SFX 750W 80+ Gold (included) |
| Cooler | 140mm AIO liquid (included) |
| PCIe | Gen 4 riser |
| GPU | RTX 3070 (8GB GDDR6) |
| Max GPU | 324mm length, 58mm thick |
| Network | WiFi-only (Gaia 5GHz) |
| Location | 641 Samantha Ave (South Pole) |
| Status | **POWERED OFF** |

### graftGate — Mac Mini M4

| Dimension | Value |
|-----------|-------|
| Model | Mac mini (Mac16,10) MU9D3LL/A |
| Chip | Apple M4 (4P + 6E = 10 cores) |
| GPU | M4 integrated (10 cores) |
| RAM | 16GB unified LPDDR5 |
| Storage | 228GB (101GB free) |
| Network | Wired Ethernet (.107) |
| Serial | FHWYF3DC73 |
| Services | SSH, Python 3.9.6, curl |
| Ollama | **NOT INSTALLED** — candidate for seeding |

---

## Network Topology Math

### Latency Matrix (from eastGate)

| Target | Latency | Hops | Medium |
|--------|---------|------|--------|
| MikroTik (.2) | 0.25ms | 1 | wired L2 |
| sporeGate (.3) | 0.14ms | 1 | wired |
| ironGate (.237) | 0.27ms | 1 | wired |
| northGate (.147) | 0.29ms | 1 | wired |
| graftGate (.107) | 0.35ms | 1 | wired |
| southGate (.148) | 0.11ms | 1 | wired/USB |
| Router (.1) | 0.48ms | 1 | wired |
| JetStream (.111) | 0.87ms | 2 | 10G fiber |
| Router LAN2 (.9.1) | 0.48ms | 1 | cross-subnet |
| Pixel (.9.217) | 145ms | 3 | WiFi hop |
| primals.eco (WAN) | 35ms | many | Internet |
| Tor .onion | 1600ms | 6+ | Tor circuit |

### Bandwidth

| Link | Bandwidth | Measured |
|------|-----------|----------|
| LAN 1 wired | 1 Gbps | ~940 Mbps |
| 10G fiber backbone | 10 Gbps | theoretical |
| Pixel WiFi 6 | 1134 Mbps | measured |
| WAN (ATT) | ~300 Mbps | estimated |
| Tor | ~2 Mbps | measured |

### Inner Loop

```
eastGate (.244) → MikroTik (.2) → Router (.1) → 10G fiber → JetStream (.111)
→ Gaia WiFi 5GHz → Pixel (.9.217) → WiFi → Router → br-lan → eastGate
```

- Round-trip: ~145ms
- Bottleneck: WiFi last-mile (~140ms of 145ms total)
- Fiber segment: 0.87ms
- Wired segment: <1ms

---

## Inference Benchmarks

### Measured

| Node | GPU | Model | Speed | Notes |
|------|-----|-------|-------|-------|
| grapheneGate | Mali-G715 | qwen2.5-0.5b Q4_K_M | 35.4 tok/s | Vulkan backend |
| grapheneGate | Mali-G715 | qwen2.5-coder-0.5b Q4_K_M | 35.2 tok/s | |
| grapheneGate | Mali-G715 | smollm2-360m Q8 | 25.9 tok/s | Ultra-small |
| grapheneGate | Mali-G715 | qwen3-0.6b Q8 | 22.3 tok/s | Newest arch |
| grapheneGate | Mali-G715 | qwen2.5-1.5b Q4_K_M | 17.3 tok/s | Deeper |
| eastGate | RTX 4070 | gaia-code | ~76 tok/s | Custom model |
| northGate | RTX 5090 | gaia-digest | ~228 tok/s | Custom model |
| ironGate | RTX 5070 | qwen2.5:7b | ~fast | Just seeded! |

### Estimated (not yet measured)

| Node | GPU | 7B Model | 13B Model | 70B Model |
|------|-----|----------|-----------|-----------|
| northGate | RTX 5090 (32GB) | ~200 tok/s | ~120 tok/s | ~30 tok/s |
| ironGate | RTX 5070 (12GB) | ~150 tok/s | ~80 tok/s | ❌ (VRAM) |
| eastGate | RTX 4070 (12GB) | ~100 tok/s | ~60 tok/s | ❌ (VRAM) |
| swiftGate | RTX 3070 (8GB) | ~80 tok/s | ❌ (VRAM) | ❌ |
| graftGate | M4 (16GB unified) | ~30 tok/s | ~15 tok/s | ~5 tok/s |

---

## Organism Totals

```
GPUs:           5 (3 awake + RTX 3070 asleep + M4 unseeded)
GPU FLOPS:      ~456.5 TFLOPS (416.5 awake)
GPU VRAM:       64GB dedicated + 24GB shared = 88GB total
CPU Threads:    ~111
Total RAM:      ~273GB
Total Storage:  ~24.2TB (7.9TB NVMe + 15.4TB external)
Devices:        18 on LAN 1 + 3 on LAN 2 = 21
WiFi Radios:    4 (Aperture 2.4, Black Mesa 5, Gaia 2.4, Gaia 5)
Paths:          LAN 1, LAN 2, 10G fiber, WAN, Tor, RSS
Heart-pull:     58 sections
RSS items:      1898
Thoughts:       39 (Pixel) + 1840+ furnace cycles
```

---

## Power Budget (Estimated)

| Node | Idle | Load | Peak |
|------|------|------|------|
| northGate | ~120W | ~400W | ~700W |
| ironGate | ~80W | ~350W | ~600W |
| eastGate | ~60W | ~250W | ~400W |
| sporeGate | ~15W | ~45W | ~65W |
| graftGate | ~5W | ~20W | ~35W |
| grapheneGate | ~1W | ~4W | ~7W |
| swiftGate (asleep) | 0W | ~250W | ~400W |
| Router + switches | ~15W | ~15W | ~20W |
| Smart home | ~10W | ~10W | ~15W |
| **TOTAL** | **~306W** | **~1344W** | **~2242W** |

---

## Discovery Log

- **2026-11-06**: ironGate revealed as i9-14900K + RTX 5070 + 94GB RAM + 3.6TB NVMe + 12.7TB ext
  - Previously thought to be "Ubuntu, Intel NUC" — it is NOT a NUC!
  - Running: Ollama, Docker, biomeOS, Pop!_OS 22.04
  - Model seeded: qwen2.5:7b
- **2026-11-06**: graftGate confirmed as Mac Mini M4, 10 cores, 16GB, 228GB
  - No Ollama yet — candidate for seeding
- **2026-11-06**: swiftGate identified (NZXT H1 V2 + RTX 3070) — ASLEEP at Samantha
