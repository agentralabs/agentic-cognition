---
status: stable
---

# Integration Guide

AgenticCognition integrates with other Agentra sisters through typed bridge traits. All bridges have NoOp defaults, so no external dependency is ever required.

## Bridge Architecture

Each bridge is a Rust trait with a NoOp default implementation. When a sister is available, the real implementation replaces the NoOp. This pattern ensures AgenticCognition compiles and runs without any sister present.

```rust
pub trait MemoryBridge: Send + Sync {
    fn recall_context(&self, topic: &str) -> Option<Vec<String>> { None }
    fn store_insight(&self, insight: &str) -> Result<()> { Ok(()) }
}
```

## Memory Bridge

Connect AgenticMemory to provide historical conversation context. Memory observations strengthen belief evidence and improve shadow detection accuracy.

**Capabilities when connected:**
- Retrieve past conversation topics to inform belief confidence
- Store cognition insights as memory facts for cross-session recall
- Link belief changes to specific conversation episodes
- Use temporal chains to track when beliefs were formed or challenged

## Planning Bridge

Connect AgenticPlanning to feed goals and decision commitments into the decision fingerprint analysis. Planning state helps predict future choices.

**Capabilities when connected:**
- Import active goals as context for decision simulation
- Feed commitment status into drift tracking
- Use goal completion patterns to refine the prediction engine
- Detect value-goal misalignment

## Identity Bridge

Connect AgenticIdentity for signed model snapshots and tamper-evident verification of `.acog` files.

**Capabilities when connected:**
- Sign model snapshots with cryptographic receipts
- Verify model integrity through the identity chain
- Attribute belief changes to specific interaction sessions
- Enable trust-gated access to sensitive shadow data

## Vision Bridge

Connect AgenticVision to incorporate visual observations into the user model.

**Capabilities when connected:**
- Use UI interaction patterns as behavioral evidence
- Detect visual attention patterns that reveal preferences
- Incorporate screenshot-based observations into belief evidence

## Codebase Bridge

Connect AgenticCodebase to model the user's technical identity and coding patterns.

**Capabilities when connected:**
- Import coding style patterns into the decision fingerprint
- Use code review preferences to strengthen work-domain beliefs
- Track technical growth through codebase evolution

## Comm Bridge

Connect AgenticComm to model communication patterns and social dynamics.

**Capabilities when connected:**
- Analyze message tone and style as behavioral evidence
- Track communication preference drift over time
- Detect projection patterns in inter-agent communication

## Cognition Bridge (self-bridge)

AgenticCognition exposes its own bridge trait for other sisters to consume.

**Capabilities exposed:**
- Predict user preferences for any sister's decision points
- Provide belief context for memory storage decisions
- Offer shadow awareness for identity trust calculations

## Standalone Usage

AgenticCognition is independently installable and operable. No sister integration is required for any core functionality. The full feature set -- belief physics, shadow detection, drift tracking, prediction -- works entirely standalone.

```bash
# Install standalone
curl -fsSL https://agentralabs.tech/install/cognition | bash

# Use without any sister
acog model create
acog belief add $MODEL_ID "I prefer clarity over cleverness" --domain values
acog shadow map $MODEL_ID
```

## MCP Integration

The `agentic-cognition-mcp` server exposes 14 tools over the Model Context Protocol. Any MCP-compatible client gains instant access to persistent user modeling. See [MCP Tools](mcp-tools.md) for the complete tool reference.

## Standalone Guarantee

AgenticCognition is independently installable and operable. Sister integrations enhance capability but are never required for core functionality. The NoOp bridge pattern ensures that missing sisters never cause errors -- they simply return neutral defaults.
