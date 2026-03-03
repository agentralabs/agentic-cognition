---
status: stable
---

# API Reference

AgenticCognition exposes its functionality through four primary surfaces: the Rust library API, the MCP server, the CLI, and C FFI bindings.

## Rust Library

The core library (`agentic-cognition` crate) provides two primary engine types and supporting modules.

### WriteEngine

All mutation operations for the living user model.

| Method | Description |
|:---|:---|
| `create_model(name: Option<&str>)` | Create a new living user model |
| `heartbeat(model_id, context)` | Record an interaction heartbeat |
| `add_belief(model_id, text, domain, confidence)` | Add a belief to the model |
| `strengthen_belief(model_id, belief_id, evidence)` | Increase belief confidence |
| `weaken_belief(model_id, belief_id, evidence)` | Decrease belief confidence |
| `connect_beliefs(model_id, a, b, rel_type)` | Create an entanglement link |
| `crystallize_belief(model_id, belief_id)` | Force-crystallize a belief |
| `collapse_belief(model_id, belief_id)` | Trigger belief collapse |
| `delete_model(model_id)` | Delete a model and its data |

### QueryEngine

All read operations for the living user model.

| Method | Description |
|:---|:---|
| `vitals(model_id)` | Model health metrics and activity summary |
| `portrait(model_id, depth)` | Natural-language portrait of the user |
| `soul_reflect(model_id, focus)` | Deep soul reflection across all dimensions |
| `belief_query(model_id, query, domain, min_confidence)` | Query beliefs by text, domain, or property |
| `belief_graph(model_id, depth, center)` | Full belief graph with entanglements |
| `keystones(model_id)` | Identify keystone beliefs |
| `contradictions(model_id)` | Detect contradictory belief pairs |
| `self_topology(model_id)` | Self-concept topology map |
| `pattern_fingerprint(model_id, domain)` | Decision-making fingerprint |
| `shadow_map(model_id)` | Shadow map with projections and blindspots |
| `drift_track(model_id, range, domain)` | Longitudinal drift analysis |
| `predict(model_id, query)` | Preference prediction |
| `simulate(model_id, scenario, options)` | Decision simulation |
| `consciousness_map(model_id)` | Consciousness region activity |
| `list_models()` | List all models in storage |

### CognitionStore

Storage abstraction for `.acog` file persistence.

| Method | Description |
|:---|:---|
| `new(path)` | Create a store at the given directory path |
| `save(model)` | Write model to `.acog` file with BLAKE3 integrity |
| `load(model_id)` | Load model from `.acog` file with integrity check |
| `delete(model_id)` | Delete model file |
| `list()` | List all model IDs in the store |

### format::AcogFile

Direct file I/O for `.acog` format.

| Method | Description |
|:---|:---|
| `write(path, model)` | Write model to file with atomic temp-rename |
| `read(path)` | Read model from file with BLAKE3 verification |
| `verify(path)` | Verify file integrity without loading |

## MCP Tools (14)

All 14 MCP tools are accessible through `agentic-cognition-mcp` over JSON-RPC 2.0 stdio transport. See [MCP Tools](mcp-tools.md) for full parameter tables and response formats.

| Tool | Operation |
|:---|:---|
| `cognition_model_create` | Create a new living user model |
| `cognition_model_heartbeat` | Record an interaction heartbeat |
| `cognition_model_vitals` | Retrieve model health metrics |
| `cognition_model_portrait` | Generate natural-language portrait |
| `cognition_belief_add` | Add a belief to the model |
| `cognition_belief_query` | Query beliefs |
| `cognition_belief_graph` | Retrieve belief graph |
| `cognition_soul_reflect` | Deep soul reflection |
| `cognition_self_topology` | Self-concept topology |
| `cognition_pattern_fingerprint` | Decision fingerprint |
| `cognition_shadow_map` | Shadow map |
| `cognition_drift_track` | Drift tracking |
| `cognition_predict` | Preference prediction |
| `cognition_simulate` | Decision simulation |

## CLI Commands (40+)

The `acog` binary provides 40+ commands organized into groups. See [CLI Reference](cli-reference.md) for complete command documentation.

| Group | Commands | Purpose |
|:---|---:|:---|
| model | 9 | Model lifecycle management |
| belief | 12 | Belief graph operations |
| self | 6 | Self-concept topology |
| pattern | 3 | Behavioral pattern analysis |
| shadow | 3 | Shadow psychology mapping |
| bias | 2 | Cognitive bias detection |
| drift | 2 | Longitudinal drift tracking |
| predict | 3 | Prediction engine |

## FFI Bindings

The `agentic-cognition-ffi` crate exposes a C-compatible FFI surface. See [FFI Reference](ffi-reference.md) for the complete header file and memory management rules.

Available bindings:

| Language | Package |
|:---|:---|
| Python | `pip install agentic-cognition` |
| Node.js / WASM | `npm/wasm` package directory |
| C / C++ | `agentic_cognition_ffi.h` header |
| Swift | Via C FFI bridge |
| Go | Via cgo with C header |
