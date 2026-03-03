//! Strict validation — no silent fallbacks

use crate::types::{BeliefDomain, CognitionError, CognitionResult, ModelLifecycleStage};

/// Validator for all input data
pub struct Validator;

impl Validator {
    /// Validate confidence value is in [0.0, 1.0]
    pub fn validate_confidence(value: f64) -> CognitionResult<f64> {
        if !(0.0..=1.0).contains(&value) {
            return Err(CognitionError::InvalidConfidence(value));
        }
        Ok(value)
    }

    /// Validate a non-empty string
    pub fn validate_non_empty(field: &str, value: &str) -> CognitionResult<()> {
        if value.trim().is_empty() {
            return Err(CognitionError::ValidationError(format!(
                "{field} cannot be empty"
            )));
        }
        Ok(())
    }

    /// Validate lifecycle transition
    pub fn validate_lifecycle_transition(
        from: &ModelLifecycleStage,
        to: &ModelLifecycleStage,
    ) -> CognitionResult<()> {
        if !from.can_transition_to(to) {
            return Err(CognitionError::InvalidStateTransition {
                from: format!("{from:?}"),
                to: format!("{to:?}"),
            });
        }
        Ok(())
    }

    /// Validate domain string to BeliefDomain
    pub fn parse_domain(domain: &str) -> CognitionResult<BeliefDomain> {
        match domain.to_lowercase().as_str() {
            "self" => Ok(BeliefDomain::Self_),
            "relationships" => Ok(BeliefDomain::Relationships),
            "work" => Ok(BeliefDomain::Work),
            "politics" => Ok(BeliefDomain::Politics),
            "religion" => Ok(BeliefDomain::Religion),
            "science" => Ok(BeliefDomain::Science),
            "values" => Ok(BeliefDomain::Values),
            "world_model" | "worldmodel" => Ok(BeliefDomain::WorldModel),
            "identity" => Ok(BeliefDomain::Identity),
            "capability" => Ok(BeliefDomain::Capability),
            "worth" => Ok(BeliefDomain::Worth),
            "other" => Ok(BeliefDomain::Other),
            _ => Err(CognitionError::ValidationError(format!(
                "Unknown domain: {domain}"
            ))),
        }
    }

    /// Validate a model ID string
    pub fn validate_uuid(value: &str) -> CognitionResult<uuid::Uuid> {
        uuid::Uuid::parse_str(value)
            .map_err(|e| CognitionError::ValidationError(format!("Invalid UUID: {e}")))
    }
}
