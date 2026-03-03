//! Living User Model — the breathing representation of a human

use crate::types::consciousness::*;
use crate::types::ids::*;
use crate::types::self_concept::*;
use crate::types::soul::*;
use serde::{Deserialize, Serialize};

/// The living user model — breathes, evolves, knows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivingUserModel {
    /// Unique identifier
    pub id: ModelId,
    /// The model's soul
    pub soul: ModelSoul,
    /// Current consciousness state
    pub consciousness: ConsciousnessState,
    /// Vital signs
    pub vitals: ModelVitals,
    /// Self-concept topology
    pub self_concept: SelfConceptTopology,
    /// Identity continuity thread
    pub identity_thread: IdentityContinuityThread,
    /// Lifecycle stage
    pub lifecycle_stage: ModelLifecycleStage,
    /// Creation timestamp
    pub created_at: Timestamp,
    /// Last updated
    pub updated_at: Timestamp,
    /// Last heartbeat
    pub last_heartbeat: Timestamp,
    /// Evidence count
    pub evidence_count: u64,
    /// Consent status
    pub consent: ConsentStatus,
    /// Privacy settings
    pub privacy: PrivacySettings,
}

impl LivingUserModel {
    pub fn new() -> Self {
        let now = Timestamp::now();
        Self {
            id: ModelId::new(),
            soul: ModelSoul::default(),
            consciousness: ConsciousnessState::default(),
            vitals: ModelVitals::default(),
            self_concept: SelfConceptTopology::default(),
            identity_thread: IdentityContinuityThread::default(),
            lifecycle_stage: ModelLifecycleStage::Birth,
            created_at: now,
            updated_at: now,
            last_heartbeat: now,
            evidence_count: 0,
            consent: ConsentStatus::Pending,
            privacy: PrivacySettings::default(),
        }
    }

    pub fn with_id(mut self, id: ModelId) -> Self {
        self.id = id;
        self
    }
}

impl Default for LivingUserModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Model vital signs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVitals {
    /// Model health (0-1)
    pub health: f64,
    /// Confidence in accuracy
    pub confidence: f64,
    /// Staleness in seconds
    pub staleness_secs: u64,
    /// Evidence base size
    pub evidence_count: u64,
    /// Is model in crisis
    pub in_crisis: bool,
    /// Heartbeat interval in seconds
    pub heartbeat_interval_secs: u64,
    /// Last significant update
    pub last_significant_update: Timestamp,
    /// Prediction accuracy (rolling average)
    pub prediction_accuracy: f64,
}

impl Default for ModelVitals {
    fn default() -> Self {
        Self {
            health: 1.0,
            confidence: 0.0,
            staleness_secs: 0,
            evidence_count: 0,
            in_crisis: false,
            heartbeat_interval_secs: 3600,
            last_significant_update: Timestamp::now(),
            prediction_accuracy: 0.0,
        }
    }
}

/// Model lifecycle stages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelLifecycleStage {
    Birth,
    Infancy,
    Growth,
    Maturity,
    Stale,
    Crisis,
    Rebirth,
}

impl ModelLifecycleStage {
    pub fn can_transition_to(&self, target: &ModelLifecycleStage) -> bool {
        use ModelLifecycleStage::*;
        matches!(
            (self, target),
            (Birth, Infancy)
                | (Infancy, Growth)
                | (Growth, Maturity)
                | (Maturity, Stale)
                | (Maturity, Crisis)
                | (Stale, Crisis)
                | (Crisis, Rebirth)
                | (Rebirth, Growth)
                | (Growth, Crisis)
                | (Maturity, Growth) // evolution
        )
    }
}

/// Consent status for user modeling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentStatus {
    Pending,
    Granted,
    Revoked,
    Limited { domains_allowed: u8 },
}

/// Privacy settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    /// Allow shadow belief detection
    pub allow_shadow_detection: bool,
    /// Allow prediction
    pub allow_prediction: bool,
    /// Allow drift tracking
    pub allow_drift_tracking: bool,
    /// Domains excluded from modeling
    pub excluded_domains: Vec<String>,
    /// Data retention days (0 = forever)
    pub retention_days: u32,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            allow_shadow_detection: true,
            allow_prediction: true,
            allow_drift_tracking: true,
            excluded_domains: Vec::new(),
            retention_days: 0,
        }
    }
}

/// Identity continuity thread
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityContinuityThread {
    /// Thread ID
    pub thread_id: ThreadId,
    /// Origin point
    pub origin: Timestamp,
    /// Continuous elements
    pub continuous_elements: Vec<ContinuousElement>,
    /// Breakpoints
    pub breakpoints: Vec<IdentityBreakpoint>,
    /// Thread strength
    pub strength: f64,
}

/// Element that persists through identity change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuousElement {
    pub element_type: ContinuousElementType,
    pub persistence: f64,
    pub first_observed: Timestamp,
    pub still_present: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContinuousElementType {
    Value { name: String },
    ResponsePattern { trigger: String, response: String },
    LifeTheme { theme: String },
    Interest { domain: String },
    RelationalPattern { pattern: String },
    RecurringChallenge { challenge: String },
}

/// Identity breakpoint — a discontinuity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityBreakpoint {
    pub timestamp: Timestamp,
    pub cause: BreakpointCause,
    pub what_changed: Vec<String>,
    pub severity: f64,
    pub valence: BreakpointValence,
    pub integrated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BreakpointCause {
    LifeEvent { event: String },
    RelationshipShift { relationship: String },
    BeliefCollapse { belief: String },
    OutcomeShock { outcome: String },
    AccumulationThreshold { accumulated: String },
    ExternalCrisis { crisis: String },
    IntentionalTransformation { intention: String },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BreakpointValence {
    Growth,
    Damage,
    Neutral,
    Mixed,
}
