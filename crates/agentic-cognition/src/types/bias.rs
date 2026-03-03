//! Bias field types

use crate::types::ids::*;
use serde::{Deserialize, Serialize};

/// Bias field — systematic distortions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BiasField {
    pub biases: Vec<ActiveBias>,
    pub triggers: Vec<EmotionalTrigger>,
}

/// An active bias
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveBias {
    pub id: BiasId,
    pub name: String,
    pub bias_type: BiasType,
    pub strength: f64,
    pub domains_affected: Vec<String>,
    pub evidence: Vec<BiasEvidence>,
    pub self_aware: bool,
    pub detected_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiasType {
    Confirmation,
    Anchoring,
    Availability,
    SunkCost,
    Optimism,
    Pessimism,
    DunningKruger,
    Bandwagon,
    Authority,
    Framing,
    StatusQuo,
    LossAversion,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasEvidence {
    pub observation: String,
    pub timestamp: Timestamp,
    pub context: String,
}

/// Emotional trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalTrigger {
    pub id: TriggerId,
    pub trigger: String,
    pub response_pattern: String,
    pub intensity: f64,
    pub origin: Option<String>,
    pub coping_strategy: Option<String>,
    pub detected_at: Timestamp,
}
