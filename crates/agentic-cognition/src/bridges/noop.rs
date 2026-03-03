//! NoOp bridge implementations -- standalone operation defaults.
//!
//! Every bridge trait method has a default no-op implementation.
//! `NoOpBridges` is the unit struct that enables zero-dependency
//! standalone execution.

use super::*;

/// Default no-op bridge that returns empty results for all sisters.
///
/// When AgenticCognition runs standalone (without sister integrations),
/// all bridges are wired to `NoOpBridges`. Each trait method returns
/// its zero/empty default: empty vecs, `None` options, `true` for
/// permission checks, 1.0 for decay (no decay).
#[derive(Debug, Clone, Copy, Default)]
pub struct NoOpBridges;

impl MemoryBridge for NoOpBridges {}
impl PlanningBridge for NoOpBridges {}
impl TimeBridge for NoOpBridges {}
impl IdentityBridge for NoOpBridges {}
impl VisionBridge for NoOpBridges {}
impl CodebaseBridge for NoOpBridges {}
impl CommBridge for NoOpBridges {}
impl RealityBridge for NoOpBridges {}
impl ContractBridge for NoOpBridges {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_memory() {
        let noop = NoOpBridges;
        assert!(noop.search_context("test", 10).is_empty());
        assert!(noop.get_conversation_patterns().is_none());
        assert!(noop.get_stated_beliefs().is_empty());
    }

    #[test]
    fn test_noop_planning() {
        let noop = NoOpBridges;
        assert!(noop.get_active_goals().is_empty());
        assert!(noop.get_decision_history().is_empty());
        assert!(noop.get_commitment_patterns().is_none());
    }

    #[test]
    fn test_noop_time() {
        let noop = NoOpBridges;
        assert_eq!(noop.calculate_decay(1_000_000), 1.0);
        assert!(noop.get_temporal_windows().is_empty());
    }

    #[test]
    fn test_noop_identity() {
        let noop = NoOpBridges;
        assert!(noop.verify_identity("any_id"));
        assert!(noop.get_identity_claims().is_empty());
    }

    #[test]
    fn test_noop_reality() {
        let noop = NoOpBridges;
        assert!(noop.reality_check("claim").is_none());
    }

    #[test]
    fn test_noop_contract() {
        let noop = NoOpBridges;
        assert!(noop.get_active_agreements().is_empty());
        assert!(noop.check_boundary("any_action"));
    }

    #[test]
    fn test_bridge_set_default() {
        let set = BridgeSet::default();
        assert!(set.memory.search_context("test", 5).is_empty());
        assert!(set.planning.get_active_goals().is_empty());
        assert_eq!(set.time.calculate_decay(100), 1.0);
        assert!(set.identity.verify_identity("id"));
    }

    #[test]
    fn test_bridge_set_builder() {
        let _set = BridgeSet::new()
            .with_memory(Box::new(NoOpBridges))
            .with_planning(Box::new(NoOpBridges));
        // Just verify it compiles and chains correctly
    }
}
