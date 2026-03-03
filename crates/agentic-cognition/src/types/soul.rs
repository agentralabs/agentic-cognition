//! Soul types — the essence of a person

use crate::types::ids::*;
use serde::{Deserialize, Serialize};

/// The model's soul — essence, drives, wounds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSoul {
    pub essence: IdentityEssence,
    pub deep_values: Vec<DeepValue>,
    pub drives: Vec<FundamentalDrive>,
    pub core_wound: Option<CoreWound>,
    pub core_gift: Option<CoreGift>,
    pub self_narrative: SelfNarrative,
    pub presented_self: PresentedSelf,
    pub authentic_self: AuthenticSelf,
    pub authenticity_gap: f64,
}

impl Default for ModelSoul {
    fn default() -> Self {
        Self {
            essence: IdentityEssence::default(),
            deep_values: Vec::new(),
            drives: Vec::new(),
            core_wound: None,
            core_gift: None,
            self_narrative: SelfNarrative::default(),
            presented_self: PresentedSelf::default(),
            authentic_self: AuthenticSelf::default(),
            authenticity_gap: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityEssence {
    pub persistent_traits: Vec<PersistentTrait>,
    pub true_optimization_target: String,
    pub deepest_fear: Option<String>,
    pub deepest_desire: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentTrait {
    pub name: String,
    pub strength: f64,
    pub first_observed: Timestamp,
    pub consistency: f64,
    pub evidence_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepValue {
    pub name: String,
    pub importance: f64,
    pub conscious: bool,
    pub origin: Option<String>,
    pub first_observed: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FundamentalDrive {
    Recognition { strength: f64 },
    Mastery { strength: f64, domain: String },
    Security { strength: f64 },
    Connection { strength: f64 },
    Autonomy { strength: f64 },
    Significance { strength: f64 },
    Growth { strength: f64 },
    Contribution { strength: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreWound {
    pub description: String,
    pub origin_period: LifePeriod,
    pub manifestations: Vec<String>,
    pub compensations: Vec<String>,
    pub healing_progress: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGift {
    pub description: String,
    pub expression: Vec<String>,
    pub recognition_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SelfNarrative {
    pub story: String,
    pub themes: Vec<String>,
    pub accuracy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PresentedSelf {
    pub traits: Vec<String>,
    pub values_claimed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthenticSelf {
    pub traits_observed: Vec<String>,
    pub values_demonstrated: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LifePeriod {
    EarlyChildhood,
    Childhood,
    Adolescence,
    YoungAdult,
    Adult,
    MidLife,
    Later,
    Unknown,
}

/// Soul reflection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulReflection {
    pub reflection_id: ReflectionId,
    pub model_id: ModelId,
    pub reflected_at: Timestamp,
    pub essence: DiscoveredEssence,
    pub confidence: f64,
    pub evidence: Vec<EssenceEvidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredEssence {
    pub core_traits: Vec<ObservedTrait>,
    pub drives: Vec<FundamentalDrive>,
    pub deep_fears: Vec<String>,
    pub true_optimization_target: String,
    pub core_wound: Option<CoreWound>,
    pub core_gift: Option<CoreGift>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservedTrait {
    pub trait_name: String,
    pub strength: f64,
    pub consistency: f64,
    pub evidence_count: u32,
    pub self_aware: bool,
    pub perception_gap: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EssenceEvidence {
    pub observation: String,
    pub timestamp: Timestamp,
    pub supports: String,
    pub weight: f64,
}
