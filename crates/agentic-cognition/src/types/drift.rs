//! Drift and change types

use crate::types::ids::*;
use serde::{Deserialize, Serialize};

/// Drift timeline — how beliefs change over time
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DriftTimeline {
    pub events: Vec<DriftEvent>,
    pub value_tectonics: Vec<ValueTectonic>,
    pub metamorphoses: Vec<Metamorphosis>,
    pub growth_rings: Vec<GrowthRing>,
}

/// A drift event — belief movement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftEvent {
    pub id: DriftId,
    pub belief_id: BeliefId,
    pub timestamp: Timestamp,
    pub direction: DriftDirection,
    pub magnitude: f64,
    pub cause: DriftCause,
    pub previous_confidence: f64,
    pub new_confidence: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriftDirection {
    Strengthening,
    Weakening,
    Shifting,
    Reversing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DriftCause {
    Evidence { description: String },
    SocialInfluence { source: String },
    EmotionalExperience { emotion: String },
    LifeEvent { event: String },
    Reflection,
    TimeDecay,
    Unknown,
}

/// Value tectonic — deep value shift
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueTectonic {
    pub value: String,
    pub direction: String,
    pub magnitude: f64,
    pub started_at: Timestamp,
    pub last_observed: Timestamp,
    pub evidence: Vec<String>,
}

/// Identity metamorphosis — complete transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metamorphosis {
    pub description: String,
    pub triggered_by: String,
    pub before_identity: String,
    pub after_identity: String,
    pub started_at: Timestamp,
    pub completed_at: Option<Timestamp>,
    pub progress: f64,
}

/// Growth ring — developmental marker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthRing {
    pub period: String,
    pub lessons: Vec<String>,
    pub beliefs_formed: Vec<BeliefId>,
    pub beliefs_abandoned: Vec<BeliefId>,
    pub identity_changes: Vec<String>,
    pub started_at: Timestamp,
    pub ended_at: Option<Timestamp>,
}

/// Certainty collapse event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertaintyCollapse {
    pub id: CollapseId,
    pub collapsed_belief: BeliefId,
    pub timestamp: Timestamp,
    pub trigger: CollapseTrigger,
    pub cascade: Vec<BeliefId>,
    pub identity_impact: f64,
    pub recovery_progress: f64,
    pub replacement: Option<BeliefId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollapseTrigger {
    UndeniableEvidence { evidence: String },
    TrustedSourceContradiction { source: String },
    AccumulatedDoubts { count: u32 },
    InexplicableExperience { experience: String },
    DeliberateInvestigation,
    CascadeFrom { original: CollapseId },
}

/// Conviction gravity — how strong beliefs warp cognitive space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvictionGravity {
    pub conviction_id: BeliefId,
    pub mass: f64,
    pub influence_radius: f64,
    pub evidence_attraction: f64,
    pub evidence_deflection: f64,
    pub orbiting_beliefs: Vec<BeliefId>,
}
