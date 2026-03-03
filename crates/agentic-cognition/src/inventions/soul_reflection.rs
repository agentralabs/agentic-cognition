//! Invention 2: Soul Reflection — see beneath the surface to the essence

use crate::types::*;

/// Performs soul reflection on a model to discover its essence
pub struct SoulReflector;

impl SoulReflector {
    /// Perform a deep soul reflection on the model
    pub fn reflect(model: &LivingUserModel, beliefs: &BeliefGraph) -> SoulReflection {
        let reflection_id = ReflectionId::new();
        let now = Timestamp::now();

        // Analyze persistent traits from beliefs
        let core_traits: Vec<ObservedTrait> = model
            .soul
            .essence
            .persistent_traits
            .iter()
            .map(|t| ObservedTrait {
                trait_name: t.name.clone(),
                strength: t.strength,
                consistency: t.consistency,
                evidence_count: t.evidence_count,
                self_aware: true,
                perception_gap: 0.0,
            })
            .collect();

        // Extract drives
        let drives = model.soul.drives.clone();

        // Compile deep fears from soul data
        let deep_fears: Vec<String> = model.soul.essence.deepest_fear.iter().cloned().collect();

        let essence = DiscoveredEssence {
            core_traits,
            drives,
            deep_fears,
            true_optimization_target: model.soul.essence.true_optimization_target.clone(),
            core_wound: model.soul.core_wound.clone(),
            core_gift: model.soul.core_gift.clone(),
        };

        // Calculate confidence based on evidence count
        let belief_count = beliefs.beliefs.len() as f64;
        let confidence = (belief_count / 50.0).min(1.0) * 0.8 + model.vitals.confidence * 0.2;

        SoulReflection {
            reflection_id,
            model_id: model.id,
            reflected_at: now,
            essence,
            confidence,
            evidence: Vec::new(),
        }
    }
}
