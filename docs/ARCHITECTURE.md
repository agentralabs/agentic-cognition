# AgenticCognition Architecture

## System Overview

```
+----------------------------------------------------------+
|                  MCP Server (14 Tools)                     |
|  model | belief | soul | self | pattern | shadow | bias   |
|  drift | predict | simulate | ...                         |
+----------------------------------------------------------+
|                   Cognition Engine                         |
|  +-------------+ +-------------+ +-------------+          |
|  | Write Engine | | Query Engine| | Inventions  |          |
|  +-------------+ +-------------+ +-------------+          |
|  +-------------+ +-------------+ +-------------+          |
|  | Validation  | | Indexes     | | Bridges     |          |
|  +-------------+ +-------------+ +-------------+          |
+----------------------------------------------------------+
|                    Storage Layer                           |
|  CognitionStore -> .acog files (blake3 checksums)         |
+----------------------------------------------------------+
|                   Sister Bridges                           |
|  Memory | Planning | Time | Identity | Vision | Codebase  |
+----------------------------------------------------------+
```

## Crate Organization

| Crate | Purpose |
|-------|---------|
| `agentic-cognition` | Core library with types, engines, inventions |
| `agentic-cognition-mcp` | MCP server (14 tools, JSON-RPC stdio) |
| `agentic-cognition-cli` | CLI binary (`acog`, 40+ commands) |
| `agentic-cognition-ffi` | FFI bindings (Python, WASM) |

## Data Flow

1. **Input**: Observations arrive via CLI, MCP, or bridges
2. **Validation**: Strict validation (no silent fallbacks)
3. **Write Engine**: Mutations applied atomically
4. **Indexes**: Updated after each write
5. **Persistence**: Atomic write to .acog file (temp + rename)
6. **Query Engine**: Read operations against in-memory store

## .acog File Format

```
[MAGIC: "ACOG"] [VERSION: u16] [FLAGS: u16] [BODY_LEN: u32]
[BLAKE3_CHECKSUM: 32 bytes]
[JSON_BODY: variable]
```

## Invention Architecture

The 24 inventions are organized in 5 priority tiers (P0-P4) and implemented across the type system, engines, and dedicated invention modules.

## Bridge Architecture

Each sister provides a trait with default no-op implementations:

```rust
pub trait MemoryBridge: Send + Sync {
    fn search_context(&self, query: &str, limit: usize) -> Vec<MemoryContext> {
        Vec::new()
    }
}
```

A `BridgeSet` collects all bridges. Default is `NoOpBridges` for standalone operation.
