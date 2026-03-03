//! Self-concept topology types

use crate::types::ids::*;
use serde::{Deserialize, Serialize};

/// Self-concept topology map
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SelfConceptTopology {
    pub peaks: Vec<ConfidencePeak>,
    pub valleys: Vec<InsecurityValley>,
    pub blind_canyons: Vec<BlindCanyon>,
    pub defended_territories: Vec<DefendedTerritory>,
    pub growing_edges: Vec<GrowingEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidencePeak {
    pub domain: String,
    pub height: f64,
    pub stability: f64,
    pub warranted: bool,
    pub reality_gap: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsecurityValley {
    pub domain: String,
    pub depth: f64,
    pub self_aware: bool,
    pub compensation_strategy: Option<CompensationStrategy>,
    pub origin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompensationStrategy {
    Overcompensation { area: String },
    Avoidance,
    SelfDeprecatingHumor,
    Perfectionism,
    Aggression,
    PeoplePleasing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlindCanyon {
    pub blind_area: String,
    pub blindness: f64,
    pub evidence: Vec<String>,
    pub impact: BlindnessImpact,
    pub penetrability: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlindnessImpact {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefendedTerritory {
    pub territory: String,
    pub defense_strength: f64,
    pub triggers: Vec<String>,
    pub mechanisms: Vec<DefenseMechanism>,
    pub underlying_vulnerability: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefenseMechanism {
    Denial,
    Rationalization,
    Projection,
    Deflection,
    Intellectualization,
    Humor,
    Attack,
    Withdrawal,
    Minimization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowingEdge {
    pub area: String,
    pub growth_rate: f64,
    pub challenge_level: f64,
    pub support_needed: String,
    pub since: Timestamp,
}
