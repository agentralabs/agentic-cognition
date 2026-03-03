//! Inventions 7, 17-18: Drift tracking, value tectonics, metamorphosis

use crate::types::*;

/// Drift analysis and tracking
pub struct DriftTracker;

impl DriftTracker {
    /// Calculate belief drift over a time period
    pub fn calculate_drift(belief: &Belief, events: &[DriftEvent]) -> f64 {
        let relevant_events: Vec<&DriftEvent> =
            events.iter().filter(|e| e.belief_id == belief.id).collect();

        if relevant_events.is_empty() {
            return 0.0;
        }

        let total_drift: f64 = relevant_events
            .iter()
            .map(|e| match e.direction {
                DriftDirection::Strengthening => e.magnitude,
                DriftDirection::Weakening => -e.magnitude,
                DriftDirection::Shifting => e.magnitude * 0.5,
                DriftDirection::Reversing => e.magnitude * 2.0,
            })
            .sum();

        total_drift.abs()
    }

    /// Detect value tectonic shifts — slow deep movements in values
    pub fn detect_tectonics(beliefs: &BeliefGraph, drift: &DriftTimeline) -> Vec<ValueTectonic> {
        let mut tectonics = Vec::new();
        let now = Timestamp::now();

        // Look for beliefs in the Values domain with consistent drift direction
        for belief in beliefs.beliefs.values() {
            if belief.domain != BeliefDomain::Values {
                continue;
            }

            let events: Vec<&DriftEvent> = drift
                .events
                .iter()
                .filter(|e| e.belief_id == belief.id)
                .collect();

            if events.len() < 3 {
                continue;
            }

            // Check for consistent direction
            let strengthening = events
                .iter()
                .filter(|e| e.direction == DriftDirection::Strengthening)
                .count();
            let weakening = events
                .iter()
                .filter(|e| e.direction == DriftDirection::Weakening)
                .count();

            let total = events.len();
            if strengthening as f64 / total as f64 > 0.7 {
                tectonics.push(ValueTectonic {
                    value: belief.content.clone(),
                    direction: "strengthening".to_string(),
                    magnitude: strengthening as f64 / total as f64,
                    started_at: events.first().map(|e| e.timestamp).unwrap_or(now),
                    last_observed: events.last().map(|e| e.timestamp).unwrap_or(now),
                    evidence: events.iter().map(|e| format!("{:?}", e.cause)).collect(),
                });
            } else if weakening as f64 / total as f64 > 0.7 {
                tectonics.push(ValueTectonic {
                    value: belief.content.clone(),
                    direction: "weakening".to_string(),
                    magnitude: weakening as f64 / total as f64,
                    started_at: events.first().map(|e| e.timestamp).unwrap_or(now),
                    last_observed: events.last().map(|e| e.timestamp).unwrap_or(now),
                    evidence: events.iter().map(|e| format!("{:?}", e.cause)).collect(),
                });
            }
        }

        tectonics
    }

    /// Detect identity metamorphosis — complete transformation events
    pub fn detect_metamorphosis(drift: &DriftTimeline, threshold: f64) -> Vec<&Metamorphosis> {
        drift
            .metamorphoses
            .iter()
            .filter(|m| m.progress > threshold)
            .collect()
    }
}
