//! Prediction and simulation types

use crate::types::ids::*;
use serde::{Deserialize, Serialize};

/// Preference prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferencePrediction {
    pub id: PredictionId,
    pub model_id: ModelId,
    pub item: String,
    pub predicted_preference: f64,
    pub confidence: f64,
    pub reasoning: Vec<String>,
    pub created_at: Timestamp,
}

/// Decision simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionSimulation {
    pub id: SimulationId,
    pub model_id: ModelId,
    pub scenario: String,
    pub options: Vec<SimulationOption>,
    pub predicted_choice: Option<usize>,
    pub predicted_process: Vec<String>,
    pub confidence: f64,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationOption {
    pub description: String,
    pub predicted_probability: f64,
    pub alignment_factors: Vec<String>,
    pub resistance_factors: Vec<String>,
}

/// Future self projection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureProjection {
    pub id: PredictionId,
    pub model_id: ModelId,
    pub time_horizon_days: u32,
    pub projected_beliefs: Vec<ProjectedBelief>,
    pub projected_drift: Vec<String>,
    pub branch_points: Vec<BranchPoint>,
    pub confidence: f64,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectedBelief {
    pub belief_id: BeliefId,
    pub current_confidence: f64,
    pub projected_confidence: f64,
    pub projected_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchPoint {
    pub description: String,
    pub probability: f64,
    pub if_taken: String,
    pub if_not_taken: String,
}
