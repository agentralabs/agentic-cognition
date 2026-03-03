//! Invention 1: Living User Model — lifecycle management and vitals

use crate::format::AcogFile;
use crate::types::*;

/// Living model management — lifecycle transitions and vital signs
pub struct LivingModelManager;

impl LivingModelManager {
    /// Update model vitals based on current state
    pub fn update_vitals(file: &mut AcogFile) {
        let belief_count = file.belief_graph.beliefs.len();
        let shadow_count = file.shadow.shadow_beliefs.len();
        let _drift_count = file.drift.events.len();

        // Calculate health based on model completeness
        let completeness = [
            if belief_count > 0 { 0.3 } else { 0.0 },
            if file.fingerprint.is_some() { 0.2 } else { 0.0 },
            if !file.model.soul.drives.is_empty() {
                0.2
            } else {
                0.0
            },
            if !file.model.self_concept.peaks.is_empty() {
                0.15
            } else {
                0.0
            },
            if shadow_count > 0 { 0.15 } else { 0.0 },
        ];
        file.model.vitals.health = completeness.iter().sum::<f64>().min(1.0);

        // Confidence grows with evidence
        let evidence = file.model.evidence_count as f64;
        file.model.vitals.confidence =
            (evidence / 200.0).min(1.0) * 0.7 + file.model.vitals.health * 0.3;

        file.model.vitals.evidence_count = file.model.evidence_count;

        // Check for crisis: too many contradictions or low health
        let contradiction_count = file.belief_graph.find_contradictions().len();
        if contradiction_count > 5 || file.model.vitals.health < 0.3 {
            file.model.vitals.in_crisis = true;
        }
    }

    /// Check and perform lifecycle transitions
    pub fn check_lifecycle_transition(file: &mut AcogFile) {
        let evidence = file.model.evidence_count;
        let confidence = file.model.vitals.confidence;
        let health = file.model.vitals.health;

        let new_stage = match file.model.lifecycle_stage {
            ModelLifecycleStage::Birth if evidence > 5 => Some(ModelLifecycleStage::Infancy),
            ModelLifecycleStage::Infancy if evidence > 50 && confidence > 0.3 => {
                Some(ModelLifecycleStage::Growth)
            }
            ModelLifecycleStage::Growth if evidence > 200 && confidence > 0.6 => {
                Some(ModelLifecycleStage::Maturity)
            }
            ModelLifecycleStage::Maturity if health < 0.3 => Some(ModelLifecycleStage::Crisis),
            ModelLifecycleStage::Crisis if health > 0.5 => Some(ModelLifecycleStage::Rebirth),
            ModelLifecycleStage::Rebirth if evidence > 50 => Some(ModelLifecycleStage::Growth),
            _ => None,
        };

        if let Some(stage) = new_stage {
            file.model.lifecycle_stage = stage;
        }
    }
}
