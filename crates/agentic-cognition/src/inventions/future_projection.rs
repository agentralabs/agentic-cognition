//! Invention: Future Self Projection — project who the user is becoming

use crate::types::*;

/// Projects the future self based on current drift trajectories
pub struct FutureProjector;

impl FutureProjector {
    /// Create a future projection
    pub fn project(
        model: &LivingUserModel,
        beliefs: &BeliefGraph,
        drift: &DriftTimeline,
        time_horizon_days: u32,
    ) -> FutureProjection {
        let id = PredictionId::new();
        let now = Timestamp::now();

        // Project beliefs based on drift
        let projected_beliefs: Vec<ProjectedBelief> = beliefs
            .beliefs
            .values()
            .map(|belief| {
                let mut projected_confidence = belief.confidence;

                // Apply drift events
                for event in &drift.events {
                    if event.belief_id == belief.id {
                        match event.direction {
                            DriftDirection::Strengthening => {
                                projected_confidence += event.magnitude * 0.1;
                            }
                            DriftDirection::Weakening => {
                                projected_confidence -= event.magnitude * 0.1;
                            }
                            DriftDirection::Shifting => {
                                projected_confidence += (event.magnitude - 0.5) * 0.1;
                            }
                            DriftDirection::Reversing => {
                                projected_confidence = 1.0 - projected_confidence;
                            }
                        }
                    }
                }

                // Apply time decay
                let decay_factor = 1.0 - (time_horizon_days as f64 * 0.001).min(0.3);
                if belief.state == BeliefState::Forming {
                    projected_confidence *= decay_factor;
                }

                projected_confidence = projected_confidence.clamp(0.0, 1.0);

                let projected_state = if projected_confidence < 0.2 {
                    "collapsed".to_string()
                } else if projected_confidence > 0.8 && belief.crystallization > 0.7 {
                    "crystallized".to_string()
                } else {
                    "evolving".to_string()
                };

                ProjectedBelief {
                    belief_id: belief.id,
                    current_confidence: belief.confidence,
                    projected_confidence,
                    projected_state,
                }
            })
            .collect();

        // Identify branch points from active tensions
        let branch_points: Vec<BranchPoint> = model
            .consciousness
            .active_tensions
            .iter()
            .map(|tension| BranchPoint {
                description: tension.description.clone(),
                probability: 0.5,
                if_taken: format!("Resolution toward: {}", tension.between.0),
                if_not_taken: format!("Resolution toward: {}", tension.between.1),
            })
            .collect();

        // Project drift descriptions
        let projected_drift: Vec<String> = drift
            .value_tectonics
            .iter()
            .map(|vt| format!("Value '{}' continuing to shift {}", vt.value, vt.direction))
            .collect();

        let confidence = (model.vitals.confidence * 0.4 + 0.1)
            * (1.0 - (time_horizon_days as f64 / 365.0).min(0.5));

        FutureProjection {
            id,
            model_id: model.id,
            time_horizon_days,
            projected_beliefs,
            projected_drift,
            branch_points,
            confidence: confidence.max(0.05),
            created_at: now,
        }
    }
}
