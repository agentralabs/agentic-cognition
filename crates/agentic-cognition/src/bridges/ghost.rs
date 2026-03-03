//! CognitionGhostWriter -- SPEC-22: Cognition -> Ghost bridge trait.
//!
//! The Ghost system is AgenticCognition's ability to provide
//! invisible cognitive context to other sisters. When Hydra or
//! any sister needs to understand a user's cognitive patterns
//! without the user seeing the raw model, Ghost provides a
//! filtered, ethically-bounded view.
//!
//! This trait defines the contract for how cognition data flows
//! out to other systems while respecting privacy boundaries.

use crate::types::{BeliefDomain, ModelId};

// ============================================================
// GHOST OUTPUT TYPES
// ============================================================

/// A ghost-written cognitive hint -- distilled insight for other sisters.
///
/// Ghost hints are always:
/// - Anonymous (no raw belief content, only patterns)
/// - Bounded (respect consent/privacy settings)
/// - Actionable (useful for the consuming sister)
#[derive(Debug, Clone)]
pub struct GhostHint {
    /// What kind of hint this is
    pub hint_type: GhostHintType,
    /// Confidence in this hint (0.0-1.0)
    pub confidence: f64,
    /// The hint content (abstracted, not raw beliefs)
    pub content: String,
    /// Which domain this applies to
    pub domain: Option<BeliefDomain>,
    /// Freshness: how recently the underlying evidence was updated (seconds ago)
    pub freshness_secs: u64,
}

/// Categories of ghost hints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GhostHintType {
    /// User tends to prefer X over Y
    PreferenceSignal,
    /// User is sensitive about topic X
    SensitivityWarning,
    /// User's communication style hint
    StyleHint,
    /// User is likely in emotional state X
    EmotionalContext,
    /// User has strong belief about X
    ConvictionSignal,
    /// User is currently in growth/change around X
    GrowthEdge,
    /// User has a known blind spot around X
    BlindSpotAlert,
    /// User's decision-making pattern for this type of choice
    DecisionPattern,
}

/// A ghost profile -- aggregated cognitive summary for a sister.
///
/// This is a privacy-filtered snapshot of the user model,
/// suitable for passing to other sisters without exposing raw data.
#[derive(Debug, Clone)]
pub struct GhostProfile {
    /// Model this profile was generated from
    pub model_id: ModelId,
    /// Active hints (sorted by relevance)
    pub hints: Vec<GhostHint>,
    /// Overall model confidence (how well do we know this user?)
    pub model_confidence: f64,
    /// How many interactions the model is based on
    pub evidence_depth: u64,
    /// Current lifecycle stage name
    pub lifecycle: String,
    /// Domains where the model is strongest
    pub strong_domains: Vec<BeliefDomain>,
    /// Domains where the model has blind spots
    pub weak_domains: Vec<BeliefDomain>,
}

/// Filter for what ghost information to include.
#[derive(Debug, Clone, Default)]
pub struct GhostFilter {
    /// Only include hints of these types (empty = all)
    pub hint_types: Vec<GhostHintType>,
    /// Only include hints in these domains (empty = all)
    pub domains: Vec<BeliefDomain>,
    /// Minimum confidence threshold
    pub min_confidence: f64,
    /// Maximum number of hints
    pub max_hints: usize,
    /// Whether to include sensitivity warnings
    pub include_sensitivities: bool,
}

impl GhostFilter {
    pub fn new() -> Self {
        Self {
            hint_types: Vec::new(),
            domains: Vec::new(),
            min_confidence: 0.0,
            max_hints: 50,
            include_sensitivities: true,
        }
    }

    /// Only return preference signals.
    pub fn preferences_only(mut self) -> Self {
        self.hint_types = vec![GhostHintType::PreferenceSignal];
        self
    }

    /// Only return hints above a confidence threshold.
    pub fn min_confidence(mut self, threshold: f64) -> Self {
        self.min_confidence = threshold;
        self
    }

    /// Limit the number of hints.
    pub fn limit(mut self, max: usize) -> Self {
        self.max_hints = max;
        self
    }

    /// Filter to specific domains.
    pub fn in_domains(mut self, domains: Vec<BeliefDomain>) -> Self {
        self.domains = domains;
        self
    }
}

// ============================================================
// GHOST WRITER TRAIT
// ============================================================

/// The CognitionGhostWriter trait (SPEC-22).
///
/// Implemented by the cognition engine to provide filtered cognitive
/// context to other sisters. The ghost writer respects privacy
/// settings and consent boundaries.
///
/// # Privacy Contract
///
/// Ghost output MUST:
/// - Never expose raw belief content (only abstracted patterns)
/// - Respect the model's consent status and privacy settings
/// - Only provide hints the user has not explicitly restricted
/// - Include confidence levels so consumers can weight appropriately
///
/// # Usage
///
/// ```rust,ignore
/// // From another sister (e.g., Comm):
/// let ghost = cognition.ghost_writer();
/// let profile = ghost.ghost_profile(&model_id, GhostFilter::new())?;
///
/// for hint in &profile.hints {
///     match hint.hint_type {
///         GhostHintType::StyleHint => adjust_response_style(hint),
///         GhostHintType::SensitivityWarning => add_sensitivity_guard(hint),
///         _ => {}
///     }
/// }
/// ```
pub trait CognitionGhostWriter: Send + Sync {
    /// Generate a ghost profile for a model.
    ///
    /// The profile is filtered according to the GhostFilter and
    /// the model's privacy settings. Returns None if the model
    /// has opted out of ghost sharing.
    fn ghost_profile(
        &self,
        model_id: &ModelId,
        filter: GhostFilter,
    ) -> Result<Option<GhostProfile>, crate::types::CognitionError>;

    /// Get specific ghost hints relevant to a query/context.
    ///
    /// More targeted than ghost_profile -- returns only hints
    /// relevant to the given context string.
    fn ghost_hints(
        &self,
        model_id: &ModelId,
        context: &str,
        max_hints: usize,
    ) -> Result<Vec<GhostHint>, crate::types::CognitionError>;

    /// Check if a model allows ghost sharing.
    ///
    /// Returns false if the model has opted out or if consent
    /// has not been granted for ghost data.
    fn allows_ghost_sharing(&self, model_id: &ModelId) -> bool;
}

/// No-op ghost writer for standalone operation.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoOpGhostWriter;

impl CognitionGhostWriter for NoOpGhostWriter {
    fn ghost_profile(
        &self,
        _model_id: &ModelId,
        _filter: GhostFilter,
    ) -> Result<Option<GhostProfile>, crate::types::CognitionError> {
        Ok(None)
    }

    fn ghost_hints(
        &self,
        _model_id: &ModelId,
        _context: &str,
        _max_hints: usize,
    ) -> Result<Vec<GhostHint>, crate::types::CognitionError> {
        Ok(Vec::new())
    }

    fn allows_ghost_sharing(&self, _model_id: &ModelId) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ghost_filter_builder() {
        let filter = GhostFilter::new()
            .preferences_only()
            .min_confidence(0.7)
            .limit(10);

        assert_eq!(filter.hint_types.len(), 1);
        assert_eq!(filter.min_confidence, 0.7);
        assert_eq!(filter.max_hints, 10);
    }

    #[test]
    fn test_noop_ghost_writer() {
        let ghost = NoOpGhostWriter;
        let model_id = ModelId::new();

        assert!(!ghost.allows_ghost_sharing(&model_id));

        let profile = ghost.ghost_profile(&model_id, GhostFilter::new()).unwrap();
        assert!(profile.is_none());

        let hints = ghost.ghost_hints(&model_id, "test", 10).unwrap();
        assert!(hints.is_empty());
    }

    #[test]
    fn test_ghost_hint_types() {
        let hint = GhostHint {
            hint_type: GhostHintType::PreferenceSignal,
            confidence: 0.85,
            content: "Prefers concise explanations".to_string(),
            domain: Some(BeliefDomain::Work),
            freshness_secs: 300,
        };

        assert_eq!(hint.hint_type, GhostHintType::PreferenceSignal);
        assert!(hint.confidence > 0.8);
    }
}
