//! Strict validation — no silent fallbacks (SPEC-14)

use crate::types::{BeliefDomain, CognitionError, CognitionResult};

/// Maximum lengths for validated fields
pub const MAX_BELIEF_CONTENT_LENGTH: usize = 2000;
pub const MAX_SHADOW_CONTENT_LENGTH: usize = 1000;
pub const MAX_TRIGGER_CONTENT_LENGTH: usize = 500;
pub const MAX_OBSERVATIONS_PER_HEARTBEAT: usize = 100;
pub const MAX_BELIEFS_PER_MODEL: usize = 10000;
pub const MAX_CONNECTIONS_PER_BELIEF: usize = 100;
pub const MAX_MODEL_NAME_LENGTH: usize = 200;

/// Strict validator for MCP and API inputs
pub struct McpValidator;

impl McpValidator {
    /// Validate a model_id parameter from MCP input
    pub fn validate_model_id(params: &serde_json::Value) -> CognitionResult<crate::types::ModelId> {
        let id_str = params
            .get("model_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CognitionError::ValidationError("Missing required parameter: model_id".into())
            })?;

        let uuid = uuid::Uuid::parse_str(id_str)
            .map_err(|e| CognitionError::ValidationError(format!("Invalid model_id: {e}")))?;

        Ok(crate::types::ModelId::from_uuid(uuid))
    }

    /// Validate a confidence value from MCP input
    pub fn validate_confidence(params: &serde_json::Value, field: &str) -> CognitionResult<f64> {
        let value = params.get(field).and_then(|v| v.as_f64()).ok_or_else(|| {
            CognitionError::ValidationError(format!("Missing required parameter: {field}"))
        })?;

        if !(0.0..=1.0).contains(&value) {
            return Err(CognitionError::InvalidConfidence(value));
        }

        Ok(value)
    }

    /// Validate a required string parameter
    pub fn require_string(params: &serde_json::Value, field: &str) -> CognitionResult<String> {
        let value = params.get(field).and_then(|v| v.as_str()).ok_or_else(|| {
            CognitionError::ValidationError(format!("Missing required parameter: {field}"))
        })?;

        if value.trim().is_empty() {
            return Err(CognitionError::ValidationError(format!(
                "{field} cannot be empty"
            )));
        }

        Ok(value.to_string())
    }

    /// Get an optional string parameter
    pub fn optional_string(params: &serde_json::Value, field: &str) -> Option<String> {
        params
            .get(field)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// Validate belief content (length and content checks)
    pub fn validate_belief_content(content: &str) -> CognitionResult<()> {
        if content.trim().is_empty() {
            return Err(CognitionError::ValidationError(
                "Belief content cannot be empty".into(),
            ));
        }
        if content.len() > MAX_BELIEF_CONTENT_LENGTH {
            return Err(CognitionError::ValidationError(format!(
                "Belief content too long: {} chars (max {})",
                content.len(),
                MAX_BELIEF_CONTENT_LENGTH
            )));
        }
        // Check for control characters
        if content
            .chars()
            .any(|c| c.is_control() && c != '\n' && c != '\t')
        {
            return Err(CognitionError::ValidationError(
                "Belief content contains invalid control characters".into(),
            ));
        }
        Ok(())
    }

    /// Validate a domain string
    pub fn validate_domain(
        params: &serde_json::Value,
        field: &str,
    ) -> CognitionResult<BeliefDomain> {
        let domain_str = Self::require_string(params, field)?;
        crate::engine::validation::Validator::parse_domain(&domain_str)
    }

    /// Validate an operation name against allowed operations
    pub fn validate_operation(
        params: &serde_json::Value,
        allowed: &[&str],
    ) -> CognitionResult<String> {
        let op = Self::require_string(params, "operation")?;
        if !allowed.contains(&op.as_str()) {
            return Err(CognitionError::ValidationError(format!(
                "Unknown operation: {op}. Allowed: {}",
                allowed.join(", ")
            )));
        }
        Ok(op)
    }

    /// Validate a prediction ID
    pub fn validate_prediction_id(
        params: &serde_json::Value,
    ) -> CognitionResult<crate::types::PredictionId> {
        let id_str = params
            .get("prediction_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CognitionError::ValidationError("Missing required parameter: prediction_id".into())
            })?;

        let uuid = uuid::Uuid::parse_str(id_str)
            .map_err(|e| CognitionError::ValidationError(format!("Invalid prediction_id: {e}")))?;

        Ok(crate::types::PredictionId::from_uuid(uuid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validate_model_id() {
        let params = json!({"model_id": "550e8400-e29b-41d4-a716-446655440000"});
        assert!(McpValidator::validate_model_id(&params).is_ok());

        let bad = json!({"model_id": "not-a-uuid"});
        assert!(McpValidator::validate_model_id(&bad).is_err());

        let missing = json!({});
        assert!(McpValidator::validate_model_id(&missing).is_err());
    }

    #[test]
    fn test_validate_confidence() {
        let params = json!({"confidence": 0.8});
        assert!(McpValidator::validate_confidence(&params, "confidence").is_ok());

        let too_high = json!({"confidence": 1.5});
        assert!(McpValidator::validate_confidence(&too_high, "confidence").is_err());

        let negative = json!({"confidence": -0.1});
        assert!(McpValidator::validate_confidence(&negative, "confidence").is_err());
    }

    #[test]
    fn test_validate_belief_content() {
        assert!(McpValidator::validate_belief_content("Valid belief").is_ok());
        assert!(McpValidator::validate_belief_content("").is_err());
        assert!(McpValidator::validate_belief_content("   ").is_err());

        let too_long = "x".repeat(MAX_BELIEF_CONTENT_LENGTH + 1);
        assert!(McpValidator::validate_belief_content(&too_long).is_err());
    }

    #[test]
    fn test_require_string() {
        let params = json!({"name": "hello"});
        assert_eq!(
            McpValidator::require_string(&params, "name").unwrap(),
            "hello"
        );

        let empty = json!({"name": ""});
        assert!(McpValidator::require_string(&empty, "name").is_err());

        let missing = json!({});
        assert!(McpValidator::require_string(&missing, "name").is_err());
    }

    #[test]
    fn test_validate_operation() {
        let params = json!({"operation": "create"});
        assert!(McpValidator::validate_operation(&params, &["create", "delete"]).is_ok());
        assert!(McpValidator::validate_operation(&params, &["delete"]).is_err());
    }
}
