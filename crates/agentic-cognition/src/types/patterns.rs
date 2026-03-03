//! Pattern archaeology types — decision fingerprints, fossils, strata

use crate::types::ids::*;
use serde::{Deserialize, Serialize};

/// Decision fingerprint — unique decision-making signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionFingerprint {
    pub id: FingerprintId,
    pub model_id: ModelId,
    pub updated_at: Timestamp,
    pub confidence: f64,
    pub traits: DecisionTraits,
    pub biases: Vec<DecisionBias>,
}

impl DecisionFingerprint {
    pub fn new(model_id: ModelId) -> Self {
        Self {
            id: FingerprintId::new(),
            model_id,
            updated_at: Timestamp::now(),
            confidence: 0.0,
            traits: DecisionTraits::default(),
            biases: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionTraits {
    /// -1 = fast/minimal, +1 = exhaustive
    pub information_appetite: f64,
    /// -1 = risk-seeking, +1 = risk-averse
    pub risk_tolerance: f64,
    /// -1 = fast, +1 = accurate
    pub speed_accuracy_tradeoff: f64,
    /// -1 = gut, +1 = analysis
    pub intuition_analysis_balance: f64,
    /// -1 = solo, +1 = collaborative
    pub social_influence: f64,
    /// -1 = short-term, +1 = long-term
    pub time_horizon: f64,
    /// -1 = emotion-driven, +1 = logic-driven
    pub emotional_regulation: f64,
    /// 0 = never reconsiders, 1 = always reconsidering
    pub reversibility_seeking: f64,
}

impl Default for DecisionTraits {
    fn default() -> Self {
        Self {
            information_appetite: 0.0,
            risk_tolerance: 0.0,
            speed_accuracy_tradeoff: 0.0,
            intuition_analysis_balance: 0.0,
            social_influence: 0.0,
            time_horizon: 0.0,
            emotional_regulation: 0.0,
            reversibility_seeking: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionBias {
    pub name: String,
    pub strength: f64,
    pub domains_affected: Vec<String>,
    pub evidence_count: u32,
}

/// Reasoning fossil — ancient pattern still shaping thought
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningFossil {
    pub id: FossilId,
    pub model_id: ModelId,
    pub pattern: String,
    pub origin_period: String,
    pub still_active: bool,
    pub influence_strength: f64,
    pub manifestations: Vec<String>,
    pub discovered_at: Timestamp,
}

/// Cognitive stratum — a developmental layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveStratum {
    pub id: StratumId,
    pub model_id: ModelId,
    pub name: String,
    pub period: String,
    pub depth: u32,
    pub beliefs_formed: Vec<BeliefId>,
    pub patterns_learned: Vec<String>,
    pub still_visible: bool,
}
