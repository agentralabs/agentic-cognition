//! Sister bridge traits for AgenticCognition
//!
//! Each sister provides a bridge trait with default no-op implementations
//! so AgenticCognition can run standalone or with sister integrations.
//!
//! Sub-modules:
//! - `noop`   -- NoOpBridges and BridgeSet defaults
//! - `config` -- BridgeConfig + BridgeRegistry for runtime bridge wiring
//! - `ghost`  -- CognitionGhostWriter trait (SPEC-22: Cognition -> Ghost)
//! - `hydra`  -- HydraAdapter trait (SPEC-11: Cognition <-> Hydra)

pub mod config;
pub mod ghost;
pub mod hydra;
pub mod noop;

// Re-export all bridge traits and types at module level
pub use config::*;
pub use ghost::*;
pub use hydra::*;
pub use noop::*;

// ============================================================
// BRIDGE TRAITS — one per sister
// ============================================================

/// Bridge to AgenticMemory -- provides historical context and evidence
pub trait MemoryBridge: Send + Sync {
    fn search_context(&self, _query: &str, _limit: usize) -> Vec<MemoryContext> {
        Vec::new()
    }

    fn get_conversation_patterns(&self) -> Option<ConversationPatterns> {
        None
    }

    fn get_stated_beliefs(&self) -> Vec<StatedBelief> {
        Vec::new()
    }
}

/// Bridge to AgenticPlanning -- provides goals and decisions
pub trait PlanningBridge: Send + Sync {
    fn get_active_goals(&self) -> Vec<GoalSummary> {
        Vec::new()
    }

    fn get_decision_history(&self) -> Vec<DecisionRecord> {
        Vec::new()
    }

    fn get_commitment_patterns(&self) -> Option<CommitmentPatterns> {
        None
    }
}

/// Bridge to AgenticTime -- provides temporal operations
pub trait TimeBridge: Send + Sync {
    fn calculate_decay(&self, _age_nanos: i64) -> f64 {
        1.0 // no decay by default
    }

    fn get_temporal_windows(&self) -> Vec<TimeWindow> {
        Vec::new()
    }
}

/// Bridge to AgenticIdentity -- provides identity verification
pub trait IdentityBridge: Send + Sync {
    fn verify_identity(&self, _model_id: &str) -> bool {
        true
    }

    fn get_identity_claims(&self) -> Vec<IdentityClaim> {
        Vec::new()
    }
}

/// Bridge to AgenticVision -- provides visual pattern data
pub trait VisionBridge: Send + Sync {
    fn get_visual_patterns(&self) -> Vec<VisualPattern> {
        Vec::new()
    }
}

/// Bridge to AgenticCodebase -- provides code behavior patterns
pub trait CodebaseBridge: Send + Sync {
    fn get_coding_patterns(&self) -> Vec<CodingPattern> {
        Vec::new()
    }
}

/// Bridge to AgenticComm -- provides communication patterns
pub trait CommBridge: Send + Sync {
    fn get_communication_style(&self) -> Option<CommunicationStyle> {
        None
    }
}

/// Bridge to AgenticReality -- provides reality-testing feedback
pub trait RealityBridge: Send + Sync {
    fn reality_check(&self, _claim: &str) -> Option<RealityCheckResult> {
        None
    }
}

/// Bridge to AgenticContract -- provides agreement/boundary data
pub trait ContractBridge: Send + Sync {
    fn get_active_agreements(&self) -> Vec<AgreementSummary> {
        Vec::new()
    }

    fn check_boundary(&self, _action: &str) -> bool {
        true // no restrictions by default
    }
}

// ============================================================
// BRIDGE DATA TYPES
// ============================================================

#[derive(Debug, Clone)]
pub struct MemoryContext {
    pub content: String,
    pub relevance: f64,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct ConversationPatterns {
    pub topics: Vec<String>,
    pub interaction_frequency: f64,
    pub tone_patterns: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct StatedBelief {
    pub content: String,
    pub context: String,
    pub timestamp: i64,
    pub confidence_expressed: f64,
}

#[derive(Debug, Clone)]
pub struct GoalSummary {
    pub description: String,
    pub priority: f64,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct DecisionRecord {
    pub description: String,
    pub chosen_option: String,
    pub alternatives: Vec<String>,
    pub reasoning: Option<String>,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct CommitmentPatterns {
    pub follow_through_rate: f64,
    pub common_abandonments: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TimeWindow {
    pub start: i64,
    pub end: i64,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct IdentityClaim {
    pub claim_type: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct VisualPattern {
    pub pattern_type: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct CodingPattern {
    pub language: String,
    pub pattern: String,
    pub frequency: f64,
}

#[derive(Debug, Clone)]
pub struct CommunicationStyle {
    pub formality: f64,
    pub verbosity: f64,
    pub emotional_expressiveness: f64,
}

#[derive(Debug, Clone)]
pub struct RealityCheckResult {
    pub claim: String,
    pub grounded: bool,
    pub confidence: f64,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AgreementSummary {
    pub id: String,
    pub description: String,
    pub status: String,
    pub parties: Vec<String>,
}

// ============================================================
// BRIDGE SET -- collection of all bridge implementations
// ============================================================

/// Collection of all bridge implementations.
///
/// Use `BridgeSet::default()` for standalone operation (all no-ops).
/// Wire in real bridges for integrated multi-sister operation.
pub struct BridgeSet {
    pub memory: Box<dyn MemoryBridge>,
    pub planning: Box<dyn PlanningBridge>,
    pub time: Box<dyn TimeBridge>,
    pub identity: Box<dyn IdentityBridge>,
    pub vision: Box<dyn VisionBridge>,
    pub codebase: Box<dyn CodebaseBridge>,
    pub comm: Box<dyn CommBridge>,
    pub reality: Box<dyn RealityBridge>,
    pub contract: Box<dyn ContractBridge>,
}

impl Default for BridgeSet {
    fn default() -> Self {
        Self {
            memory: Box::new(NoOpBridges),
            planning: Box::new(NoOpBridges),
            time: Box::new(NoOpBridges),
            identity: Box::new(NoOpBridges),
            vision: Box::new(NoOpBridges),
            codebase: Box::new(NoOpBridges),
            comm: Box::new(NoOpBridges),
            reality: Box::new(NoOpBridges),
            contract: Box::new(NoOpBridges),
        }
    }
}

impl BridgeSet {
    /// Create a new BridgeSet with all no-op bridges.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder: replace the memory bridge.
    pub fn with_memory(mut self, bridge: Box<dyn MemoryBridge>) -> Self {
        self.memory = bridge;
        self
    }

    /// Builder: replace the planning bridge.
    pub fn with_planning(mut self, bridge: Box<dyn PlanningBridge>) -> Self {
        self.planning = bridge;
        self
    }

    /// Builder: replace the time bridge.
    pub fn with_time(mut self, bridge: Box<dyn TimeBridge>) -> Self {
        self.time = bridge;
        self
    }

    /// Builder: replace the identity bridge.
    pub fn with_identity(mut self, bridge: Box<dyn IdentityBridge>) -> Self {
        self.identity = bridge;
        self
    }

    /// Builder: replace the vision bridge.
    pub fn with_vision(mut self, bridge: Box<dyn VisionBridge>) -> Self {
        self.vision = bridge;
        self
    }

    /// Builder: replace the codebase bridge.
    pub fn with_codebase(mut self, bridge: Box<dyn CodebaseBridge>) -> Self {
        self.codebase = bridge;
        self
    }

    /// Builder: replace the comm bridge.
    pub fn with_comm(mut self, bridge: Box<dyn CommBridge>) -> Self {
        self.comm = bridge;
        self
    }

    /// Builder: replace the reality bridge.
    pub fn with_reality(mut self, bridge: Box<dyn RealityBridge>) -> Self {
        self.reality = bridge;
        self
    }

    /// Builder: replace the contract bridge.
    pub fn with_contract(mut self, bridge: Box<dyn ContractBridge>) -> Self {
        self.contract = bridge;
        self
    }
}
