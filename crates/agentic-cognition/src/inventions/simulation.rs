//! Invention: Decision Simulation — simulate how the user would decide

use crate::types::*;

/// Simulates decision-making based on user model
pub struct DecisionSimulator;

impl DecisionSimulator {
    /// Simulate a decision scenario
    pub fn simulate(
        model: &LivingUserModel,
        beliefs: &BeliefGraph,
        fingerprint: &Option<DecisionFingerprint>,
        scenario: &str,
        options: &[String],
    ) -> DecisionSimulation {
        let id = SimulationId::new();
        let now = Timestamp::now();

        let sim_options: Vec<SimulationOption> = options
            .iter()
            .enumerate()
            .map(|(i, opt)| {
                let mut alignment = Vec::new();
                let mut resistance = Vec::new();
                let mut prob = 1.0 / options.len() as f64;

                // Check value alignment for each option
                for value in &model.soul.deep_values {
                    let opt_lower = opt.to_lowercase();
                    let val_lower = value.name.to_lowercase();
                    if opt_lower.contains(&val_lower) {
                        alignment.push(format!("Aligns with value: {}", value.name));
                        prob += 0.1 * value.importance;
                    }
                }

                // Check belief alignment
                for belief in beliefs.beliefs.values() {
                    let opt_lower = opt.to_lowercase();
                    let content_lower = belief.content.to_lowercase();
                    if opt_lower.contains(&content_lower) {
                        if belief.confidence > 0.6 {
                            alignment.push(format!("Supported by belief: {}", belief.content));
                            prob += 0.05;
                        }
                    } else if content_lower.contains("not")
                        && opt_lower.contains(&content_lower.replace("not ", ""))
                    {
                        resistance.push(format!("Conflicts with belief: {}", belief.content));
                        prob -= 0.05;
                    }
                }

                // Factor in decision traits
                if let Some(fp) = fingerprint {
                    if i == 0 {
                        // First option gets status quo bias if applicable
                        prob += fp.traits.risk_tolerance * -0.05; // risk-averse prefer first/safe option
                    }
                }

                SimulationOption {
                    description: opt.clone(),
                    predicted_probability: prob.max(0.01),
                    alignment_factors: alignment,
                    resistance_factors: resistance,
                }
            })
            .collect();

        // Normalize probabilities
        let total: f64 = sim_options.iter().map(|o| o.predicted_probability).sum();
        let sim_options: Vec<SimulationOption> = sim_options
            .into_iter()
            .map(|mut o| {
                o.predicted_probability /= total;
                o
            })
            .collect();

        // Find predicted choice
        let predicted_choice = sim_options
            .iter()
            .enumerate()
            .max_by(|a, b| {
                a.1.predicted_probability
                    .partial_cmp(&b.1.predicted_probability)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i);

        let predicted_process = vec![
            "Evaluate options against core values".to_string(),
            "Check alignment with active beliefs".to_string(),
            "Apply decision-making style preferences".to_string(),
        ];

        let confidence = model.vitals.confidence * 0.5 + 0.2;

        DecisionSimulation {
            id,
            model_id: model.id,
            scenario: scenario.to_string(),
            options: sim_options,
            predicted_choice,
            predicted_process,
            confidence: confidence.min(0.8),
            created_at: now,
        }
    }
}
