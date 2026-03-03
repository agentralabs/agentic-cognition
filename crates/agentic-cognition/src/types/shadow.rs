//! Shadow types — unconscious beliefs, projections, blindspots

use crate::types::ids::*;
use serde::{Deserialize, Serialize};

/// Shadow map — the unconscious landscape
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShadowMap {
    pub shadow_beliefs: Vec<ShadowBelief>,
    pub projections: Vec<Projection>,
    pub blindspots: Vec<Blindspot>,
}

/// A shadow belief — held unconsciously
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowBelief {
    pub id: BeliefId,
    pub content: String,
    pub evidence: Vec<ShadowEvidence>,
    pub strength: f64,
    pub contradicts_conscious: Option<BeliefId>,
    pub behavioral_signs: Vec<String>,
    pub detected_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowEvidence {
    pub observation: String,
    pub timestamp: Timestamp,
    pub weight: f64,
}

/// A projection — seeing disowned traits in others
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projection {
    pub id: ProjectionId,
    pub disowned_trait: String,
    pub projected_onto: String,
    pub strength: f64,
    pub evidence: Vec<String>,
    pub original_self_trait: String,
    pub detected_at: Timestamp,
}

/// A blindspot — systematic blind area
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blindspot {
    pub id: BlindspotId,
    pub area: String,
    pub blindness_level: f64,
    pub evidence: Vec<String>,
    pub impact: String,
    pub penetrability: f64,
    pub detected_at: Timestamp,
}
