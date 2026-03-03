//! HydraAdapter -- SPEC-11: Cognition <-> Hydra bridge trait.
//!
//! Defines how AgenticCognition integrates with the Hydra orchestrator.
//! This module provides:
//!
//! - `HydraAdapter` trait: Cognition's side of the Hydra bridge
//! - `CognitionSummary`: Token-efficient state for Hydra's context window
//! - `CognitionCommand`: Commands Hydra can issue to Cognition
//! - `CognitionCommandResult`: Results Cognition returns to Hydra
//!
//! The SDK provides the generic `HydraBridge` trait. This module provides
//! a Cognition-specific adapter that maps between Cognition's internal
//! types and the SDK's generic Hydra types.

use crate::types::{BeliefDomain, CognitionError, ModelId};
use serde::{Deserialize, Serialize};

// ============================================================
// COGNITION SUMMARY (for Hydra context window)
// ============================================================

/// Token-efficient summary of Cognition's current state.
///
/// Hydra calls this to understand what Cognition knows about
/// the user without loading the full model. Designed to fit
/// in ~200 tokens of LLM context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitionSummary {
    /// Number of active models
    pub model_count: usize,

    /// Total beliefs across all models
    pub total_beliefs: usize,

    /// Total evidence count across all models
    pub total_evidence: u64,

    /// Active model lifecycle stages
    pub lifecycle_stages: Vec<(String, String)>, // (model_id, stage)

    /// Strongest belief domains (sorted by count)
    pub top_domains: Vec<(String, usize)>,

    /// Number of detected biases
    pub bias_count: usize,

    /// Number of shadow beliefs
    pub shadow_count: usize,

    /// Whether there's an active session
    pub has_active_session: bool,

    /// Current session name
    pub session_name: Option<String>,

    /// Brief status line for LLM context
    pub status_line: String,
}

// ============================================================
// HYDRA COMMANDS (what Hydra can ask Cognition to do)
// ============================================================

/// Commands Hydra can issue to Cognition.
///
/// These represent the operations Hydra might need Cognition
/// to perform as part of orchestrated multi-sister workflows.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum CognitionCommand {
    /// Assess the user's likely response to a proposed action
    AssessReaction {
        model_id: String,
        proposed_action: String,
    },

    /// Get preference prediction for a choice
    PredictPreference { model_id: String, item: String },

    /// Simulate a decision through the user model
    SimulateDecision {
        model_id: String,
        scenario: String,
        options: Vec<String>,
    },

    /// Get ghost hints for another sister's use
    GhostHints {
        model_id: String,
        context: String,
        max_hints: usize,
    },

    /// Check if a claim is grounded in the user's beliefs
    GroundClaim { model_id: String, claim: String },

    /// Get the model's soul reflection
    Reflect { model_id: String },

    /// Project the user's future trajectory
    ProjectFuture { model_id: String, days: u32 },

    /// Get current consciousness state
    ConsciousnessState { model_id: String },

    /// Heartbeat -- feed observations into the model
    Heartbeat {
        model_id: String,
        observations: Vec<String>,
    },
}

/// Result of executing a Hydra command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitionCommandResult {
    /// Whether the command succeeded
    pub success: bool,

    /// Result data (command-specific)
    pub data: serde_json::Value,

    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Evidence IDs produced by this command (for receipt chain)
    #[serde(default)]
    pub evidence_ids: Vec<String>,
}

impl CognitionCommandResult {
    /// Create a successful result.
    pub fn ok(data: serde_json::Value) -> Self {
        Self {
            success: true,
            data,
            error: None,
            evidence_ids: Vec::new(),
        }
    }

    /// Create a failed result.
    pub fn err(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: serde_json::Value::Null,
            error: Some(message.into()),
            evidence_ids: Vec::new(),
        }
    }

    /// Add evidence IDs.
    pub fn with_evidence(mut self, ids: Vec<String>) -> Self {
        self.evidence_ids = ids;
        self
    }
}

// ============================================================
// EXECUTION GATE CONTEXT (Cognition's input to risk assessment)
// ============================================================

/// Cognition-specific risk context for Hydra's execution gate.
///
/// When Hydra evaluates whether to allow an action, Cognition can
/// provide additional context about the user's likely response
/// and emotional state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitionRiskContext {
    /// How sensitive is the user about the affected domain?
    pub domain_sensitivity: f64,

    /// Is the user currently in a vulnerable emotional state?
    pub emotional_vulnerability: f64,

    /// Does this action conflict with known user values?
    pub value_conflict_risk: f64,

    /// How much does the user trust this type of operation?
    pub trust_level: f64,

    /// Any relevant belief domains
    pub affected_domains: Vec<BeliefDomain>,
}

impl Default for CognitionRiskContext {
    fn default() -> Self {
        Self {
            domain_sensitivity: 0.0,
            emotional_vulnerability: 0.0,
            value_conflict_risk: 0.0,
            trust_level: 1.0,
            affected_domains: Vec::new(),
        }
    }
}

// ============================================================
// HYDRA ADAPTER TRAIT
// ============================================================

/// The HydraAdapter trait (SPEC-11).
///
/// Cognition-specific adapter for Hydra integration. Maps between
/// Cognition's rich internal types and Hydra's generic command interface.
///
/// This is a PREPARATION trait -- Hydra does not exist yet.
/// Implementing this now ensures Cognition is ready when Hydra arrives.
///
/// # Relationship to SDK's HydraBridge
///
/// The SDK's `HydraBridge` trait provides the generic contract.
/// `HydraAdapter` is Cognition's domain-specific implementation layer
/// that sits between the generic SDK trait and Cognition's internals.
pub trait HydraAdapter: Send + Sync {
    /// Get a token-efficient summary of Cognition's state.
    ///
    /// Called by Hydra to build its context window.
    fn cognition_summary(&self) -> Result<CognitionSummary, CognitionError>;

    /// Execute a Hydra command.
    ///
    /// Called by Hydra when it needs Cognition to perform an operation
    /// as part of an orchestrated workflow.
    fn execute_command(
        &mut self,
        command: CognitionCommand,
    ) -> Result<CognitionCommandResult, CognitionError>;

    /// Provide risk context for Hydra's execution gate.
    ///
    /// Called when Hydra is evaluating whether to approve an action
    /// that affects the user. Cognition provides context about
    /// the user's sensitivity, emotional state, and values.
    fn risk_context(
        &self,
        model_id: &ModelId,
        action_description: &str,
    ) -> Result<CognitionRiskContext, CognitionError>;

    /// Check if Cognition is ready to participate in Hydra workflows.
    fn hydra_ready(&self) -> bool;
}

/// No-op Hydra adapter for standalone operation.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoOpHydraAdapter;

impl HydraAdapter for NoOpHydraAdapter {
    fn cognition_summary(&self) -> Result<CognitionSummary, CognitionError> {
        Ok(CognitionSummary {
            model_count: 0,
            total_beliefs: 0,
            total_evidence: 0,
            lifecycle_stages: Vec::new(),
            top_domains: Vec::new(),
            bias_count: 0,
            shadow_count: 0,
            has_active_session: false,
            session_name: None,
            status_line: "Cognition: standalone (no models)".to_string(),
        })
    }

    fn execute_command(
        &mut self,
        _command: CognitionCommand,
    ) -> Result<CognitionCommandResult, CognitionError> {
        Ok(CognitionCommandResult::err(
            "Hydra adapter not configured (standalone mode)",
        ))
    }

    fn risk_context(
        &self,
        _model_id: &ModelId,
        _action_description: &str,
    ) -> Result<CognitionRiskContext, CognitionError> {
        Ok(CognitionRiskContext::default())
    }

    fn hydra_ready(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cognition_summary() {
        let summary = CognitionSummary {
            model_count: 3,
            total_beliefs: 42,
            total_evidence: 150,
            lifecycle_stages: vec![
                ("model_1".into(), "Growth".into()),
                ("model_2".into(), "Maturity".into()),
            ],
            top_domains: vec![("work".into(), 15), ("values".into(), 10)],
            bias_count: 5,
            shadow_count: 3,
            has_active_session: true,
            session_name: Some("session_42".into()),
            status_line: "Cognition: 3 models, 42 beliefs, growth phase".into(),
        };

        assert_eq!(summary.model_count, 3);
        assert_eq!(summary.total_beliefs, 42);
    }

    #[test]
    fn test_command_result() {
        let ok = CognitionCommandResult::ok(serde_json::json!({"prediction": "yes"}));
        assert!(ok.success);
        assert!(ok.error.is_none());

        let err = CognitionCommandResult::err("model not found");
        assert!(!err.success);
        assert!(err.error.is_some());
    }

    #[test]
    fn test_noop_hydra_adapter() {
        let mut adapter = NoOpHydraAdapter;

        assert!(!adapter.hydra_ready());

        let summary = adapter.cognition_summary().unwrap();
        assert_eq!(summary.model_count, 0);

        let result = adapter
            .execute_command(CognitionCommand::Reflect {
                model_id: "test".into(),
            })
            .unwrap();
        assert!(!result.success);

        let risk = adapter
            .risk_context(&ModelId::new(), "delete everything")
            .unwrap();
        assert_eq!(risk.trust_level, 1.0);
    }

    #[test]
    fn test_risk_context_default() {
        let ctx = CognitionRiskContext::default();
        assert_eq!(ctx.domain_sensitivity, 0.0);
        assert_eq!(ctx.trust_level, 1.0);
        assert!(ctx.affected_domains.is_empty());
    }

    #[test]
    fn test_command_serialization() {
        let cmd = CognitionCommand::PredictPreference {
            model_id: "model_123".into(),
            item: "dark mode".into(),
        };
        let json = serde_json::to_string(&cmd).unwrap();
        assert!(json.contains("predict_preference"));
        assert!(json.contains("dark mode"));
    }
}
