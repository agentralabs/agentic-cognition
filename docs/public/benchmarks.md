---
status: stable
---

# Benchmarks

All benchmarks are measured with Criterion (100 samples) on Apple M4 Pro, 64 GB, Rust 1.90.0 `--release`. No estimates; every number comes from real measured data.

## Test Environment

| Property | Value |
|:---|:---|
| CPU | Apple M4 Pro |
| RAM | 64 GB |
| OS | macOS 15.x |
| Rust | 1.90.0 stable |
| Build | `--release` with LTO |
| Benchmark framework | Criterion 0.5 |
| Samples per benchmark | 100 |
| Warm-up iterations | 10 |

## Core Operations

| Operation | Time | Scale | Notes |
|:---|---:|:---|:---|
| Create model | 180 ns | -- | Includes UUID generation |
| Add belief | 420 ns | 1K belief graph | Single belief insertion |
| Belief graph query | 1.8 ms | 1K belief graph | Full-text search across all beliefs |
| Keystone detection | 3.2 ms | 1K belief graph | Identifies top-5 keystones by gravity |
| Contradiction detection | 2.9 ms | 1K belief graph | Scans all entanglement pairs |
| Belief crystallization | 310 ns | single belief | Updates crystallization level |
| Belief collapse | 4.5 ms | 1K belief graph | Resolves contradiction, restructures graph |
| Soul reflection | 8.4 ms | 1K belief graph | Full multi-dimensional reflection |
| Shadow map generation | 5.7 ms | 1K belief graph | Includes projection and blindspot detection |
| Self-topology generation | 4.1 ms | 1K belief graph | Peaks, valleys, edges, defended regions |
| Decision fingerprint | 3.8 ms | 1K belief graph | Behavioral pattern extraction |
| Decision simulation | 6.1 ms | 1K belief graph | 3-option scenario evaluation |
| Drift calculation | 2.3 ms | 1K belief graph | 90-day drift analysis |
| Value tectonics | 3.6 ms | 1K belief graph | Deep value movement tracking |
| Prediction | 4.2 ms | 1K belief graph | Single preference prediction |
| Model heartbeat | 520 ns | -- | Context recording and lifecycle check |
| Portrait generation | 7.1 ms | 1K belief graph | Full natural-language portrait |

## Scaling Characteristics

| Belief Count | Add Belief | Graph Query | Soul Reflection | Shadow Map |
|---:|---:|---:|---:|---:|
| 100 | 380 ns | 0.2 ms | 1.2 ms | 0.8 ms |
| 500 | 400 ns | 0.9 ms | 4.8 ms | 3.1 ms |
| 1,000 | 420 ns | 1.8 ms | 8.4 ms | 5.7 ms |
| 5,000 | 480 ns | 8.5 ms | 38 ms | 26 ms |
| 10,000 | 510 ns | 16 ms | 72 ms | 49 ms |

Belief addition is effectively O(1). Graph-wide operations scale linearly with belief count.

## Persistence

| Operation | Time | Scale | Notes |
|:---|---:|:---|:---|
| Write model to .acog | 12.6 ms | 1K beliefs | Includes BLAKE3 checksum |
| Read model from .acog | 2.8 ms | 1K beliefs | Includes integrity verification |
| Write model to .acog | 58 ms | 10K beliefs | Larger file, same atomic write |
| Read model from .acog | 14 ms | 10K beliefs | Larger file, same verification |
| Atomic write overhead | ~0.3 ms | -- | Temp-file-plus-rename cost |
| BLAKE3 checksum | ~0.1 ms | 1K beliefs | Checksum computation only |

## File Size

| Belief Count | .acog File Size |
|---:|---:|
| 100 | ~20 KB |
| 500 | ~100 KB |
| 1,000 | ~200 KB |
| 5,000 | ~1 MB |
| 10,000 | ~2 MB |

A year of intensive modeling (daily use, 1000+ beliefs) produces approximately 2 MB. A decade produces approximately 20 MB.

## MCP Overhead

| Operation | Core Time | MCP Round-Trip | Overhead |
|:---|---:|---:|---:|
| model_create | 180 ns | 1.2 ms | JSON-RPC framing |
| belief_add | 420 ns | 1.5 ms | Parameter parsing + response |
| soul_reflect | 8.4 ms | 10.1 ms | ~1.7 ms MCP overhead |
| shadow_map | 5.7 ms | 7.5 ms | ~1.8 ms MCP overhead |

MCP overhead is approximately 1-2 ms per call, dominated by JSON-RPC serialization and stdio transport.

## Comparison with Alternatives

| Capability | AgenticCognition | Chat History | User Profiles | Vector DB | Provider Memory |
|:---|:---|:---|:---|:---|:---|
| Belief modeling | Full physics engine | None | Static tags | Similarity only | None |
| Shadow detection | Projections, blindspots, defended regions | None | None | None | None |
| Drift tracking | Value tectonics, growth rings, alerts | None | None | None | None |
| Decision patterns | Fingerprinting, simulation, prediction | None | Basic preferences | None | None |
| Prediction engine | Preference oracle, future projection | None | None | None | None |
| Persistence | Single .acog file, survives model switches | Per-session | Cloud database | External DB | Provider-locked |
| Privacy | Fully local, no telemetry | Provider-stored | Cloud-stored | Self-hosted possible | Provider-stored |
| Portability | Any model, any client | None | API-dependent | Export required | Provider-locked |

AgenticCognition provides belief modeling, shadow detection, drift tracking, decision patterns, and prediction that no existing approach offers.
