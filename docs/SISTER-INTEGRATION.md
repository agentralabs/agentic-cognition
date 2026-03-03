# AgenticCognition Sister Integration

## Overview

AgenticCognition integrates with 7 sister projects via bridge traits.

## Bridge Traits

| Bridge | Sister | Purpose |
|--------|--------|---------|
| MemoryBridge | agentic-memory | Historical context, evidence |
| PlanningBridge | agentic-planning | Goals, decisions, commitments |
| TimeBridge | agentic-time | Temporal decay, scheduling |
| IdentityBridge | agentic-identity | Trust, signing, verification |
| CodebaseBridge | agentic-codebase | Code patterns, behavior |
| VisionBridge | agentic-vision | Visual patterns |
| CommBridge | agentic-comm | Communication style |

## Standalone Mode

All bridges default to `NoOpBridges` -- no dependencies required.

```rust
let engine = WriteEngine::new(store);
// Works without any sisters connected
```

## Connecting a Bridge

```rust
use agentic_cognition::bridges::*;

let bridges = BridgeSet {
    memory: Box::new(my_memory_bridge),
    ..BridgeSet::default()
};

let engine = WriteEngine::new(store).with_bridges(bridges);
```

## Hydra Integration

The `HydraAdapter` trait enables automatic discovery and orchestration:

```rust
pub trait HydraAdapter: Send + Sync {
    fn adapter_id(&self) -> &str;
    fn capabilities(&self) -> Vec<String>;
    fn handle_request(&self, method: &str, params: &str) -> Result<String, String>;
}
```
