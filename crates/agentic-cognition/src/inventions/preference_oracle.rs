//! Invention: Preference Oracle — predict preferences based on the model

use crate::types::*;

/// Predicts preferences based on the user model
pub struct PreferenceOracle;

impl PreferenceOracle {
    /// Predict preference for an item given the user model
    pub fn predict(
        model: &LivingUserModel,
        beliefs: &BeliefGraph,
        fingerprint: &Option<DecisionFingerprint>,
        item: &str,
    ) -> PreferencePrediction {
        let id = PredictionId::new();
        let now = Timestamp::now();

        // Simple preference prediction based on value alignment
        let mut score = 0.5_f64;
        let mut reasoning = Vec::new();
        let item_lower = item.to_lowercase();

        // Check belief alignment
        for belief in beliefs.beliefs.values() {
            let content_lower = belief.content.to_lowercase();
            if (content_lower.contains(&item_lower) || item_lower.contains(&content_lower))
                && belief.confidence > 0.5
            {
                score += 0.1 * belief.confidence;
                reasoning.push(format!("Aligns with belief: {}", belief.content));
            }
        }

        // Check value alignment
        for value in &model.soul.deep_values {
            let val_lower = value.name.to_lowercase();
            if item_lower.contains(&val_lower) || val_lower.contains(&item_lower) {
                score += 0.15 * value.importance;
                reasoning.push(format!("Aligns with value: {}", value.name));
            }
        }

        // Factor in decision traits if available
        if let Some(fp) = fingerprint {
            // Risk tolerance affects novel items
            if reasoning.is_empty() {
                // Unknown item — risk tolerance matters
                score += fp.traits.risk_tolerance * 0.1;
                reasoning.push("No strong alignment found — using risk tolerance".into());
            }
        }

        score = score.clamp(0.0, 1.0);

        let confidence = if reasoning.is_empty() {
            0.2 // low confidence for no-match predictions
        } else {
            (reasoning.len() as f64 * 0.15).min(0.9)
        };

        PreferencePrediction {
            id,
            model_id: model.id,
            item: item.to_string(),
            predicted_preference: score,
            confidence,
            reasoning,
            created_at: now,
        }
    }
}
