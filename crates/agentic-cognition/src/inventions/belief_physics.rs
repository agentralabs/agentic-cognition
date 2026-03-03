//! Inventions 5-9: Belief Physics — crystallization, entanglement, gravity, collapse

use crate::types::*;
use std::collections::HashMap;

/// Belief physics calculator
pub struct BeliefPhysics;

impl BeliefPhysics {
    /// Calculate crystallization level for a belief based on factors
    pub fn calculate_crystallization(belief: &Belief, now: Timestamp) -> f64 {
        let age_nanos = now.as_nanos() - belief.first_observed.as_nanos();
        let age_days = (age_nanos as f64 / 86_400_000_000_000.0).max(0.0);

        // Age factor (0-0.3): beliefs older than 365 days get max
        let age_factor = (age_days / 365.0).min(1.0) * 0.3;

        // Evidence factor (0-0.3): more evidence = more crystallized
        let evidence_total = belief.evidence_basis.experiences
            + belief.evidence_basis.testimonials
            + belief.evidence_basis.reasoning
            + belief.evidence_basis.intuition;
        let evidence_factor = (evidence_total as f64 / 20.0).min(1.0) * 0.3;

        // Confidence factor (0-0.2): high confidence = more crystallized
        let confidence_factor = belief.confidence * 0.2;

        // Emotional charge factor (0-0.2): emotional beliefs crystallize faster
        let emotion_factor = belief.emotional_charge.abs() * 0.2;

        (age_factor + evidence_factor + confidence_factor + emotion_factor).min(1.0)
    }

    /// Check for entanglement between beliefs (co-occurring changes)
    pub fn detect_entanglements(
        _graph: &BeliefGraph,
        drift_events: &[DriftEvent],
    ) -> Vec<BeliefEntanglement> {
        let mut entanglements = Vec::new();
        let mut change_windows: HashMap<BeliefId, Vec<i64>> = HashMap::new();

        // Group drift events by belief with timestamps
        for event in drift_events {
            change_windows
                .entry(event.belief_id)
                .or_default()
                .push(event.timestamp.as_nanos());
        }

        // Find beliefs that change within close time windows
        let belief_ids: Vec<BeliefId> = change_windows.keys().copied().collect();
        for i in 0..belief_ids.len() {
            for j in (i + 1)..belief_ids.len() {
                let times_a = &change_windows[&belief_ids[i]];
                let times_b = &change_windows[&belief_ids[j]];

                let mut co_occurrences = 0;
                let window_nanos = 3_600_000_000_000i64; // 1 hour window

                for ta in times_a {
                    for tb in times_b {
                        if (ta - tb).abs() < window_nanos {
                            co_occurrences += 1;
                        }
                    }
                }

                if co_occurrences >= 2 {
                    let strength = (co_occurrences as f64 / 10.0).min(1.0);
                    entanglements.push(BeliefEntanglement {
                        id: EntanglementId::new(),
                        beliefs: vec![belief_ids[i], belief_ids[j]],
                        entanglement_type: EntanglementType::Correlated,
                        strength,
                        conscious: false,
                    });
                }
            }
        }

        entanglements
    }

    /// Calculate conviction gravity — how a strong belief warps nearby beliefs
    pub fn calculate_gravity(conviction: &Belief, graph: &BeliefGraph) -> ConvictionGravity {
        let mass = conviction.confidence * (1.0 + conviction.crystallization);

        // Find beliefs connected to this conviction
        let orbiting: Vec<BeliefId> = graph
            .connections
            .iter()
            .filter(|c| c.from == conviction.id || c.to == conviction.id)
            .map(|c| {
                if c.from == conviction.id {
                    c.to
                } else {
                    c.from
                }
            })
            .collect();

        let influence_radius = mass * 0.5;
        let evidence_attraction = mass * 0.3;
        let evidence_deflection = mass * 0.2;

        ConvictionGravity {
            conviction_id: conviction.id,
            mass,
            influence_radius,
            evidence_attraction,
            evidence_deflection,
            orbiting_beliefs: orbiting,
        }
    }

    /// Detect collapse precursors — beliefs at risk of collapsing
    pub fn detect_collapse_precursors(graph: &BeliefGraph) -> Vec<(BeliefId, f64)> {
        let mut risks: Vec<(BeliefId, f64)> = Vec::new();

        for (id, belief) in &graph.beliefs {
            let mut risk = 0.0;

            // Challenged beliefs have high risk
            if belief.state == BeliefState::Challenged {
                risk += 0.4;
            }

            // High crystallization + challenge = brittle
            if belief.crystallization > 0.8 && belief.state == BeliefState::Challenged {
                risk += 0.3;
            }

            // Many contradictions increase risk
            let contradictions = graph
                .connections
                .iter()
                .filter(|c| {
                    (c.from == *id || c.to == *id)
                        && c.connection_type == ConnectionType::Contradicts
                })
                .count();
            risk += contradictions as f64 * 0.1;

            // Low evidence makes beliefs fragile
            if belief.evidence_basis.strength < 0.3 {
                risk += 0.2;
            }

            if risk > 0.3 {
                risks.push((*id, risk.min(1.0)));
            }
        }

        risks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        risks
    }
}
