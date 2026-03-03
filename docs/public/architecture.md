---
status: stable
---

# Architecture

AgenticCognition is a Rust workspace composed of four crates that together implement longitudinal user modeling -- living models of human consciousness for AI agents.

## Workspace Structure

```
agentic-cognition/
  crates/
    agentic-cognition/          # Core library
    agentic-cognition-mcp/      # MCP server binary (acog-mcp)
    agentic-cognition-cli/      # CLI binary (acog)
    agentic-cognition-ffi/      # C FFI bindings
  docs/
    public/                     # Published documentation
  tests/                        # Integration tests
  benches/                      # Criterion benchmarks
```

## Crate Responsibilities

### agentic-cognition (core)

The core library contains all domain logic. It has zero runtime dependencies on networking, MCP, or CLI frameworks. Every other crate depends on this one.

Key modules:

| Module | Purpose |
|:---|:---|
| `model` | Living user model lifecycle (Birth, Infancy, Growth, Maturity, Crisis, Rebirth) |
| `belief` | Belief physics engine -- confidence, crystallization, entanglement, gravity, collapse |
| `shadow` | Shadow psychology -- projections, blindspots, defended regions, bias fields |
| `drift` | Longitudinal drift tracking, value tectonics, growth rings |
| `predict` | Preference oracle, decision simulation, future projection |
| `self_concept` | Self-topology -- peaks, valleys, edges, defended territories |
| `pattern` | Decision fingerprinting, behavioral fossils, archaeological strata |
| `store` | Storage abstraction for `.acog` file persistence |
| `format` | Binary `.acog` file I/O with BLAKE3 integrity verification |

The core exposes two primary engine types:

- **WriteEngine** -- all mutation operations (create model, add belief, heartbeat, connect beliefs)
- **QueryEngine** -- all read operations (query beliefs, graph traversal, soul reflection, predictions)

### agentic-cognition-mcp (MCP server)

The MCP server binary (`acog-mcp`) exposes 14 tools over JSON-RPC 2.0 stdio transport. It follows the compact facade pattern: each tool name maps to a single operation, not an operation-routed facade.

Responsibilities:

- Parse JSON-RPC 2.0 frames with Content-Length framing
- Route `tools/call` requests to the core WriteEngine or QueryEngine
- Expose MCP resources for model data, belief graphs, and portraits
- Expose MCP prompts for guided model creation and belief analysis
- Enforce the 8 MiB frame size limit
- Handle `initialize`, `initialized`, `shutdown` lifecycle
- Auto-start sessions on `initialized` notification

### agentic-cognition-cli (CLI)

The CLI binary (`acog`) provides 40+ commands organized into 8 groups. It is a thin wrapper over the core library, adding only argument parsing and output formatting.

Command groups: `model`, `belief`, `self`, `pattern`, `shadow`, `bias`, `drift`, `predict`.

Output formats: `json`, `table`, `text` (controlled via `--format`).

### agentic-cognition-ffi (FFI)

The FFI crate exposes a C-compatible interface for use from Python, Node.js, Swift, and any language that supports C FFI. It uses `#[no_mangle]` and `extern "C"` functions with opaque pointer handles.

Memory management follows the allocate/free pattern: the caller receives an opaque handle and must call the corresponding `_free` function when done.

## Data Flow

```
User / Agent
     |
     v
 +---------+     +---------+     +----------+
 | CLI     | --> |         | --> | .acog    |
 | (acog)  |     |  Core   |     | file     |
 +---------+     | Library |     +----------+
                 |         |
 +---------+     |         |
 | MCP     | --> |         |
 | (acog-  |     +---------+
 |  mcp)   |         ^
 +---------+         |
                     |
 +---------+         |
 | FFI     | --------+
 | (C ABI) |
 +---------+
```

All three access surfaces (CLI, MCP, FFI) converge on the core library. The core library is the only crate that touches the `.acog` file. No access surface bypasses the core.

## Design Philosophy

### Single-file persistence

One `.acog` file holds the entire living user model. No external databases, no SQLite, no cloud services. The file survives restarts, model switches, and months between sessions.

### Privacy by architecture

All data stays local. No telemetry, no cloud sync. The user owns their cognitive model completely. The architecture enforces this -- there is no networking code in the core library.

### NoOp bridges

Sister integrations (Memory, Planning, Identity, Vision, Codebase, Comm) use typed bridge traits with NoOp defaults. AgenticCognition is independently installable and operable. Bridges enhance capability but are never required.

### Compact facade MCP pattern

The MCP server exposes 14 focused tools rather than a single tool with operation routing. Each tool has a clear name and parameter schema. This follows the Agentra MCP quality standard.

### Layered access

Users can interact at three levels of abstraction:

1. **MCP tools** -- highest level, designed for AI agents
2. **CLI commands** -- human-friendly, scriptable
3. **Rust library / FFI** -- full programmatic control

Each layer provides equivalent functionality with different ergonomics.

### Integrity by default

Every `.acog` file write includes a BLAKE3 checksum. Every read verifies it. Atomic writes use temp-file-plus-rename to prevent partial write corruption. Data integrity is not optional.

## Build and Test

```bash
# Build all crates
cargo build --release

# Run all tests
cargo test --all

# Run benchmarks
cargo bench

# Build MCP server only
cargo build --release -p agentic-cognition-mcp

# Build CLI only
cargo build --release -p agentic-cognition-cli
```

## Dependency Policy

The core library minimizes dependencies. Key external crates:

| Crate | Purpose |
|:---|:---|
| `serde` / `serde_json` | Serialization |
| `blake3` | Integrity checksums |
| `uuid` | Model and belief identifiers |
| `chrono` | Timestamps |
| `tempfile` | Atomic writes |

The MCP crate adds JSON-RPC and stdio transport. The CLI crate adds `clap` for argument parsing. The FFI crate adds only `libc` for C type compatibility.
