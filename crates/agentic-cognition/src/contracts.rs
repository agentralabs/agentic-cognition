//! Contracts bridge -- implements agentic-sdk v0.2.0 traits for Cognition.
//!
//! Provides CognitionSister, a contracts-compliant wrapper around
//! the core CognitionStore + engines.
//!
//! Implements:
//! - `Sister` — core lifecycle management
//! - `SessionManagement` — append-only sequential sessions
//! - `Grounding` — claim verification against belief graphs
//! - `Queryable` — standard query interface

use std::path::PathBuf;
use std::time::Instant;

use agentic_sdk::prelude::*;

use crate::engine::query::QueryEngine;
use crate::engine::store::CognitionStore;
use crate::engine::write::WriteEngine;
use crate::format::AcogFile;
use crate::types::*;

// ============================================================
// ERROR BRIDGE
// ============================================================

impl From<CognitionError> for SisterError {
    fn from(e: CognitionError) -> Self {
        match &e {
            CognitionError::ModelNotFound(id) => SisterError::not_found(format!("model {}", id)),
            CognitionError::BeliefNotFound(id) => SisterError::not_found(format!("belief {}", id)),
            CognitionError::PatternNotFound(id) => {
                SisterError::not_found(format!("pattern {}", id))
            }
            CognitionError::InvalidConfidence(v) => {
                SisterError::new(ErrorCode::InvalidInput, format!("Invalid confidence: {v}"))
            }
            CognitionError::InvalidStateTransition { from, to } => SisterError::new(
                ErrorCode::InvalidState,
                format!("Invalid lifecycle transition: {from} -> {to}"),
            ),
            CognitionError::ValidationError(msg) => {
                SisterError::new(ErrorCode::InvalidInput, msg.clone())
            }
            CognitionError::FormatError(msg) => {
                SisterError::new(ErrorCode::VersionMismatch, msg.clone())
            }
            CognitionError::ChecksumMismatch { expected, actual } => SisterError::new(
                ErrorCode::ChecksumMismatch,
                format!("Checksum mismatch: expected {expected}, got {actual}"),
            ),
            CognitionError::IoError(e) => SisterError::new(ErrorCode::StorageError, e.to_string()),
            CognitionError::SerializationError(msg) => {
                SisterError::new(ErrorCode::InvalidInput, msg.clone())
            }
            CognitionError::LockError(msg) => {
                SisterError::new(ErrorCode::Internal, format!("Lock error: {msg}"))
            }
            CognitionError::ModelAlreadyExists(id) => SisterError::new(
                ErrorCode::AlreadyExists,
                format!("Model {id} already exists"),
            ),
            CognitionError::DuplicateBelief(id) => SisterError::new(
                ErrorCode::AlreadyExists,
                format!("Belief {id} already exists"),
            ),
            CognitionError::SelfConnection(id) => SisterError::new(
                ErrorCode::InvalidInput,
                format!("Self-connection not allowed for belief {id}"),
            ),
            CognitionError::DuplicateConnection(a, b) => SisterError::new(
                ErrorCode::AlreadyExists,
                format!("Connection between {a} and {b} already exists"),
            ),
            CognitionError::BridgeError(msg) => {
                SisterError::new(ErrorCode::Internal, format!("Bridge error: {msg}"))
            }
            CognitionError::PredictionError(msg) => {
                SisterError::new(ErrorCode::Internal, format!("Prediction error: {msg}"))
            }
            CognitionError::SimulationError(msg) => {
                SisterError::new(ErrorCode::Internal, format!("Simulation error: {msg}"))
            }
            CognitionError::AuthError(msg) => {
                SisterError::new(ErrorCode::PermissionDenied, format!("Auth error: {msg}"))
            }
        }
    }
}

// ============================================================
// SESSION STATE
// ============================================================

#[derive(Debug, Clone)]
struct SessionRecord {
    id: ContextId,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
    model_count_at_start: usize,
}

// ============================================================
// COGNITION SISTER FACADE
// ============================================================

/// SDK-compliant facade for AgenticCognition.
///
/// Wraps the core CognitionStore + WriteEngine + QueryEngine and
/// exposes them through the standard agentic-sdk traits so Hydra
/// can consume Cognition uniformly alongside all other sisters.
pub struct CognitionSister {
    store: CognitionStore,
    write_engine: WriteEngine,
    query_engine: QueryEngine,
    #[allow(dead_code)]
    file_path: Option<PathBuf>,
    start_time: Instant,

    // Session tracking
    current_session: Option<SessionRecord>,
    sessions: Vec<SessionRecord>,
}

impl CognitionSister {
    /// Create a new CognitionSister with optional storage path.
    pub fn new(path: Option<PathBuf>) -> Result<Self, CognitionError> {
        let store = match &path {
            Some(p) => CognitionStore::with_storage(p.clone())?,
            None => CognitionStore::new(),
        };

        let write_store = match &path {
            Some(p) => CognitionStore::with_storage(p.clone())?,
            None => CognitionStore::new(),
        };

        let query_store = match &path {
            Some(p) => CognitionStore::with_storage(p.clone())?,
            None => CognitionStore::new(),
        };

        let write_engine = WriteEngine::new(write_store);
        let query_engine = QueryEngine::new(query_store);

        Ok(Self {
            store,
            write_engine,
            query_engine,
            file_path: path,
            start_time: Instant::now(),
            current_session: None,
            sessions: Vec::new(),
        })
    }

    /// Access the write engine for mutation operations.
    pub fn write_engine(&self) -> &WriteEngine {
        &self.write_engine
    }

    /// Access the query engine for read operations.
    pub fn query_engine(&self) -> &QueryEngine {
        &self.query_engine
    }

    /// Access the underlying store directly.
    pub fn store(&self) -> &CognitionStore {
        &self.store
    }

    /// Helper: count all models in the store.
    fn model_count(&self) -> usize {
        self.store.list_models().unwrap_or_default().len()
    }

    /// Helper: count all beliefs across all models.
    fn total_belief_count(&self) -> usize {
        let model_ids = self.store.list_models().unwrap_or_default();
        let mut count = 0;
        for id in &model_ids {
            if let Ok(file) = self.store.get_model(id) {
                count += file.belief_graph.beliefs.len();
            }
        }
        count
    }
}

// ============================================================
// SISTER TRAIT
// ============================================================

impl Sister for CognitionSister {
    const SISTER_TYPE: SisterType = SisterType::Cognition;
    const FILE_EXTENSION: &'static str = "acog";

    fn init(config: SisterConfig) -> SisterResult<Self> {
        let path = config.data_path.map(|d| {
            if d.is_dir() || d.extension().is_none() {
                d.join("cognition.acog")
            } else {
                d
            }
        });

        // For Cognition the path points to a directory of .acog files.
        // Resolve to the parent directory for CognitionStore.
        let store_path = path.as_ref().map(|p| {
            if p.is_dir() {
                p.clone()
            } else {
                p.parent().unwrap_or(p.as_ref()).to_path_buf()
            }
        });

        CognitionSister::new(store_path).map_err(SisterError::from)
    }

    fn health(&self) -> HealthStatus {
        let uptime = self.start_time.elapsed();
        let model_count = self.model_count();
        let _belief_count = self.total_belief_count();

        HealthStatus {
            healthy: true,
            status: if model_count == 0 {
                Status::Degraded
            } else {
                Status::Ready
            },
            uptime,
            resources: ResourceUsage {
                memory_bytes: 0, // Not tracked yet
                disk_bytes: 0,
                open_handles: 0,
            },
            warnings: if model_count == 0 {
                vec!["No models loaded".to_string()]
            } else {
                vec![]
            },
            last_error: None,
        }
    }

    fn version(&self) -> Version {
        Version::new(0, 1, 0)
    }

    fn shutdown(&mut self) -> SisterResult<()> {
        // End any open session
        if let Some(session) = self.current_session.take() {
            self.sessions.push(session);
        }
        Ok(())
    }

    fn capabilities(&self) -> Vec<Capability> {
        vec![
            Capability::new(
                "belief_graph",
                "Belief graph with connections, entanglements, keystones",
            ),
            Capability::new(
                "shadow_map",
                "Shadow beliefs, projections, defended territories",
            ),
            Capability::new("bias_field", "Active bias detection, emotional triggers"),
            Capability::new(
                "drift_timeline",
                "Value tectonics, metamorphoses, growth rings",
            ),
            Capability::new(
                "soul_reflection",
                "Deep personality synthesis and soul portrait",
            ),
            Capability::new(
                "preference_oracle",
                "Preference prediction from belief patterns",
            ),
            Capability::new(
                "decision_simulation",
                "Simulate decisions through the user model",
            ),
            Capability::new(
                "future_projection",
                "Project future self based on drift patterns",
            ),
            Capability::new(
                "consciousness_state",
                "Emotional weather, life phase, cognitive load",
            ),
            Capability::new("grounding", "Claim verification against belief evidence"),
        ]
    }
}

// ============================================================
// SESSION MANAGEMENT
// ============================================================

impl SessionManagement for CognitionSister {
    fn start_session(&mut self, name: &str) -> SisterResult<ContextId> {
        // Automatically end the previous session if one is open
        if let Some(prev) = self.current_session.take() {
            self.sessions.push(prev);
        }

        let id = ContextId::new();
        let session = SessionRecord {
            id,
            name: name.to_string(),
            created_at: chrono::Utc::now(),
            model_count_at_start: self.model_count(),
        };

        self.current_session = Some(session);
        Ok(id)
    }

    fn end_session(&mut self) -> SisterResult<()> {
        match self.current_session.take() {
            Some(session) => {
                self.sessions.push(session);
                Ok(())
            }
            None => Err(SisterError::new(
                ErrorCode::ContextNotFound,
                "No active session to end",
            )),
        }
    }

    fn current_session(&self) -> Option<ContextId> {
        self.current_session.as_ref().map(|s| s.id)
    }

    fn current_session_info(&self) -> SisterResult<ContextInfo> {
        let session = self
            .current_session
            .as_ref()
            .ok_or_else(|| SisterError::context_not_found("current"))?;

        let now = chrono::Utc::now();
        Ok(ContextInfo {
            id: session.id,
            name: session.name.clone(),
            created_at: session.created_at,
            updated_at: now,
            item_count: self.model_count(),
            size_bytes: 0,
            metadata: std::collections::HashMap::new(),
        })
    }

    fn list_sessions(&self) -> SisterResult<Vec<ContextSummary>> {
        let mut summaries: Vec<ContextSummary> = self
            .sessions
            .iter()
            .map(|s| ContextSummary {
                id: s.id,
                name: s.name.clone(),
                created_at: s.created_at,
                updated_at: s.created_at, // approximation
                item_count: s.model_count_at_start,
                size_bytes: 0,
            })
            .collect();

        // Add current session if active
        if let Some(s) = &self.current_session {
            summaries.push(ContextSummary {
                id: s.id,
                name: s.name.clone(),
                created_at: s.created_at,
                updated_at: chrono::Utc::now(),
                item_count: self.model_count(),
                size_bytes: 0,
            });
        }

        // Most recent first
        summaries.reverse();
        Ok(summaries)
    }

    fn export_session(&self, id: ContextId) -> SisterResult<ContextSnapshot> {
        // Find the session
        let session = self
            .sessions
            .iter()
            .find(|s| s.id == id)
            .or_else(|| self.current_session.as_ref().filter(|s| s.id == id))
            .ok_or_else(|| SisterError::context_not_found(id.to_string()))?;

        // Serialize current store state as snapshot data
        let models = self.store.list_models().map_err(SisterError::from)?;
        let data = serde_json::to_vec(&models)
            .map_err(|e| SisterError::new(ErrorCode::Internal, e.to_string()))?;
        let checksum = blake3::hash(&data);

        Ok(ContextSnapshot {
            sister_type: SisterType::Cognition,
            version: Version::new(0, 1, 0),
            context_info: ContextInfo {
                id: session.id,
                name: session.name.clone(),
                created_at: session.created_at,
                updated_at: chrono::Utc::now(),
                item_count: self.model_count(),
                size_bytes: data.len(),
                metadata: std::collections::HashMap::new(),
            },
            data,
            checksum: *checksum.as_bytes(),
            snapshot_at: chrono::Utc::now(),
        })
    }

    fn import_session(&mut self, snapshot: ContextSnapshot) -> SisterResult<ContextId> {
        if !snapshot.verify() {
            return Err(SisterError::new(
                ErrorCode::ChecksumMismatch,
                "Snapshot checksum verification failed",
            ));
        }

        let id = ContextId::new();
        let session = SessionRecord {
            id,
            name: snapshot.context_info.name,
            created_at: chrono::Utc::now(),
            model_count_at_start: self.model_count(),
        };

        self.sessions.push(session);
        Ok(id)
    }
}

// ============================================================
// GROUNDING
// ============================================================

impl Grounding for CognitionSister {
    fn ground(&self, claim: &str) -> SisterResult<GroundingResult> {
        let model_ids = self.store.list_models().map_err(SisterError::from)?;

        if model_ids.is_empty() {
            return Ok(GroundingResult::ungrounded(
                claim,
                "No models loaded -- no evidence available",
            ));
        }

        let claim_lower = claim.to_lowercase();
        let mut evidence = Vec::new();

        for model_id in &model_ids {
            if let Ok(file) = self.store.get_model(model_id) {
                for belief in file.belief_graph.beliefs.values() {
                    if belief.content.to_lowercase().contains(&claim_lower)
                        || claim_lower.contains(&belief.content.to_lowercase())
                    {
                        evidence.push(
                            GroundingEvidence::new(
                                "belief",
                                belief.id.to_string(),
                                belief.confidence,
                                belief.content.clone(),
                            )
                            .with_data("domain", belief.domain.to_string())
                            .with_data("state", format!("{:?}", belief.state))
                            .with_data("model_id", model_id.to_string()),
                        );
                    }
                }
            }
        }

        if evidence.is_empty() {
            return Ok(GroundingResult::ungrounded(
                claim,
                "No matching beliefs found",
            ));
        }

        let avg_confidence = evidence.iter().map(|e| e.score).sum::<f64>() / evidence.len() as f64;

        let status = if evidence.iter().any(|e| e.score > 0.7) {
            GroundingStatus::Verified
        } else {
            GroundingStatus::Partial
        };

        let result = match status {
            GroundingStatus::Verified => GroundingResult::verified(claim, avg_confidence),
            _ => GroundingResult::partial(claim, avg_confidence),
        };

        Ok(result.with_evidence(evidence).with_reason(format!(
            "Found matching beliefs with average confidence {:.2}",
            avg_confidence
        )))
    }

    fn evidence(&self, query: &str, max_results: usize) -> SisterResult<Vec<EvidenceDetail>> {
        let model_ids = self.store.list_models().map_err(SisterError::from)?;
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        for model_id in &model_ids {
            if let Ok(file) = self.store.get_model(model_id) {
                for belief in file.belief_graph.beliefs.values() {
                    if belief.content.to_lowercase().contains(&query_lower) {
                        results.push(EvidenceDetail {
                            evidence_type: "belief".to_string(),
                            id: belief.id.to_string(),
                            score: belief.confidence,
                            created_at: chrono::Utc::now(), // timestamp conversion
                            source_sister: SisterType::Cognition,
                            content: belief.content.clone(),
                            data: {
                                let mut m = std::collections::HashMap::new();
                                m.insert(
                                    "domain".to_string(),
                                    serde_json::Value::String(belief.domain.to_string()),
                                );
                                m.insert(
                                    "state".to_string(),
                                    serde_json::Value::String(format!("{:?}", belief.state)),
                                );
                                m.insert(
                                    "model_id".to_string(),
                                    serde_json::Value::String(model_id.to_string()),
                                );
                                m.insert(
                                    "crystallization".to_string(),
                                    serde_json::json!(belief.crystallization),
                                );
                                m
                            },
                        });

                        if results.len() >= max_results {
                            break;
                        }
                    }
                }
            }

            if results.len() >= max_results {
                break;
            }
        }

        // Sort by score descending
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results.truncate(max_results);

        Ok(results)
    }

    fn suggest(&self, query: &str, limit: usize) -> SisterResult<Vec<GroundingSuggestion>> {
        let model_ids = self.store.list_models().map_err(SisterError::from)?;
        let query_lower = query.to_lowercase();
        let query_words: Vec<&str> = query_lower.split_whitespace().collect();
        let mut suggestions = Vec::new();

        for model_id in &model_ids {
            if let Ok(file) = self.store.get_model(model_id) {
                for belief in file.belief_graph.beliefs.values() {
                    let content_lower = belief.content.to_lowercase();

                    // Word-overlap scoring for fuzzy matching
                    let matching_words = query_words
                        .iter()
                        .filter(|w| content_lower.contains(*w))
                        .count();

                    if matching_words > 0 {
                        let relevance = matching_words as f64 / query_words.len().max(1) as f64;

                        suggestions.push(GroundingSuggestion {
                            item_type: "belief".to_string(),
                            id: belief.id.to_string(),
                            relevance_score: relevance * belief.confidence,
                            description: belief.content.clone(),
                            data: {
                                let mut m = std::collections::HashMap::new();
                                m.insert(
                                    "domain".to_string(),
                                    serde_json::Value::String(belief.domain.to_string()),
                                );
                                m.insert(
                                    "model_id".to_string(),
                                    serde_json::Value::String(model_id.to_string()),
                                );
                                m
                            },
                        });
                    }
                }
            }
        }

        // Sort by relevance descending
        suggestions.sort_by(|a, b| {
            b.relevance_score
                .partial_cmp(&a.relevance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        suggestions.truncate(limit);

        Ok(suggestions)
    }
}

// ============================================================
// QUERYABLE
// ============================================================

impl Queryable for CognitionSister {
    fn query(&self, query: Query) -> SisterResult<QueryResult> {
        let start = Instant::now();

        match query.query_type.as_str() {
            "list" => {
                let model_ids = self.store.list_models().map_err(SisterError::from)?;
                let limit = query.limit.unwrap_or(100);
                let offset = query.offset.unwrap_or(0);

                let results: Vec<serde_json::Value> = model_ids
                    .iter()
                    .skip(offset)
                    .take(limit)
                    .filter_map(|id| {
                        self.store.get_model(id).ok().map(|file| {
                            serde_json::json!({
                                "model_id": id.to_string(),
                                "lifecycle": format!("{:?}", file.model.lifecycle_stage),
                                "belief_count": file.belief_graph.beliefs.len(),
                                "evidence_count": file.model.evidence_count,
                            })
                        })
                    })
                    .collect();

                let total = model_ids.len();
                Ok(QueryResult::new(query, results, start.elapsed())
                    .with_pagination(total, offset + limit < total))
            }

            "search" => {
                let text = query.get_string("text").unwrap_or_default();
                let text_lower = text.to_lowercase();
                let limit = query.limit.unwrap_or(50);

                let model_ids = self.store.list_models().map_err(SisterError::from)?;
                let mut results = Vec::new();

                for model_id in &model_ids {
                    if let Ok(file) = self.store.get_model(model_id) {
                        for belief in file.belief_graph.beliefs.values() {
                            if belief.content.to_lowercase().contains(&text_lower) {
                                results.push(serde_json::json!({
                                    "model_id": model_id.to_string(),
                                    "belief_id": belief.id.to_string(),
                                    "content": belief.content,
                                    "confidence": belief.confidence,
                                    "domain": belief.domain.to_string(),
                                    "state": format!("{:?}", belief.state),
                                }));

                                if results.len() >= limit {
                                    break;
                                }
                            }
                        }
                    }
                    if results.len() >= limit {
                        break;
                    }
                }

                Ok(QueryResult::new(query, results, start.elapsed()))
            }

            "recent" => {
                // Return most recently updated models
                let model_ids = self.store.list_models().map_err(SisterError::from)?;
                let limit = query.limit.unwrap_or(10);

                let mut models_with_time: Vec<(ModelId, AcogFile)> = model_ids
                    .iter()
                    .filter_map(|id| self.store.get_model(id).ok().map(|f| (*id, f)))
                    .collect();

                // Sort by updated_at descending
                models_with_time.sort_by(|a, b| {
                    b.1.model
                        .updated_at
                        .as_nanos()
                        .cmp(&a.1.model.updated_at.as_nanos())
                });

                let results: Vec<serde_json::Value> = models_with_time
                    .into_iter()
                    .take(limit)
                    .map(|(id, file)| {
                        serde_json::json!({
                            "model_id": id.to_string(),
                            "lifecycle": format!("{:?}", file.model.lifecycle_stage),
                            "belief_count": file.belief_graph.beliefs.len(),
                            "evidence_count": file.model.evidence_count,
                        })
                    })
                    .collect();

                Ok(QueryResult::new(query, results, start.elapsed()))
            }

            "get" => {
                let id_str = query
                    .get_string("id")
                    .ok_or_else(|| SisterError::invalid_input("Missing 'id' parameter"))?;

                let model_id: ModelId = id_str.parse().map_err(|_| {
                    SisterError::invalid_input(format!("Invalid model ID: {}", id_str))
                })?;

                let file = self.store.get_model(&model_id).map_err(SisterError::from)?;

                let result = serde_json::json!({
                    "model_id": model_id.to_string(),
                    "lifecycle": format!("{:?}", file.model.lifecycle_stage),
                    "belief_count": file.belief_graph.beliefs.len(),
                    "shadow_count": file.shadow.shadow_beliefs.len(),
                    "bias_count": file.bias_field.biases.len(),
                    "drift_events": file.drift.events.len(),
                    "evidence_count": file.model.evidence_count,
                    "has_fingerprint": file.fingerprint.is_some(),
                });

                Ok(QueryResult::new(query, vec![result], start.elapsed()))
            }

            other => Err(SisterError::new(
                ErrorCode::InvalidInput,
                format!("Unsupported query type: {other}. Use: list, search, recent, get"),
            )),
        }
    }

    fn supports_query(&self, query_type: &str) -> bool {
        matches!(query_type, "list" | "search" | "recent" | "get" | "related")
    }

    fn query_types(&self) -> Vec<QueryTypeInfo> {
        vec![
            QueryTypeInfo::new("list", "List all user models").optional(vec!["limit", "offset"]),
            QueryTypeInfo::new("search", "Search beliefs by text content")
                .required(vec!["text"])
                .optional(vec!["limit"]),
            QueryTypeInfo::new("recent", "Get most recently updated models")
                .optional(vec!["limit"]),
            QueryTypeInfo::new("get", "Get a specific model by ID").required(vec!["id"]),
        ]
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cognition_sister_init() {
        let sister = CognitionSister::new(None).unwrap();
        assert_eq!(CognitionSister::SISTER_TYPE, SisterType::Cognition);
        assert_eq!(CognitionSister::FILE_EXTENSION, "acog");
        assert!(sister.current_session().is_none());
    }

    #[test]
    fn test_cognition_error_to_sister_error() {
        let cog_err = CognitionError::ModelNotFound(ModelId::new());
        let sister_err: SisterError = cog_err.into();
        assert_eq!(sister_err.code, ErrorCode::NotFound);
    }

    #[test]
    fn test_health_no_models() {
        let sister = CognitionSister::new(None).unwrap();
        let health = sister.health();
        assert!(health.healthy);
        assert_eq!(health.status, Status::Degraded);
        assert!(!health.warnings.is_empty());
    }

    #[test]
    fn test_session_lifecycle() {
        let mut sister = CognitionSister::new(None).unwrap();

        // Start session
        let id = sister.start_session("test_session").unwrap();
        assert!(sister.current_session().is_some());
        assert_eq!(sister.current_session().unwrap(), id);

        // End session
        sister.end_session().unwrap();
        assert!(sister.current_session().is_none());

        // Sessions list should contain the ended session
        let sessions = sister.list_sessions().unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].name, "test_session");
    }

    #[test]
    fn test_session_auto_close() {
        let mut sister = CognitionSister::new(None).unwrap();

        let _id1 = sister.start_session("session_1").unwrap();
        let _id2 = sister.start_session("session_2").unwrap();

        // session_1 should have been auto-ended
        let sessions = sister.list_sessions().unwrap();
        // 1 past session + 1 current session
        assert_eq!(sessions.len(), 2);
    }

    #[test]
    fn test_grounding_no_models() {
        let sister = CognitionSister::new(None).unwrap();
        let result = sister.ground("any claim").unwrap();
        assert_eq!(result.status, GroundingStatus::Ungrounded);
        assert_eq!(result.confidence, 0.0);
    }

    #[test]
    fn test_queryable_list_empty() {
        let sister = CognitionSister::new(None).unwrap();
        let result = sister.query(Query::list()).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_queryable_unsupported() {
        let sister = CognitionSister::new(None).unwrap();
        let result = sister.query(Query::new("nonsense"));
        assert!(result.is_err());
    }

    #[test]
    fn test_capabilities() {
        let sister = CognitionSister::new(None).unwrap();
        let caps = sister.capabilities();
        assert!(caps.len() >= 5);
        assert!(caps.iter().any(|c| c.name == "belief_graph"));
        assert!(caps.iter().any(|c| c.name == "grounding"));
    }

    #[test]
    fn test_version() {
        let sister = CognitionSister::new(None).unwrap();
        let v = sister.version();
        assert_eq!(v.major, 0);
    }

    #[test]
    fn test_query_types() {
        let sister = CognitionSister::new(None).unwrap();
        let types = sister.query_types();
        assert!(types.len() >= 4);
        assert!(sister.supports_query("list"));
        assert!(sister.supports_query("search"));
        assert!(!sister.supports_query("nonsense"));
    }

    #[test]
    fn test_shutdown() {
        let mut sister = CognitionSister::new(None).unwrap();
        let _id = sister.start_session("active").unwrap();
        sister.shutdown().unwrap();
        assert!(sister.current_session().is_none());
        assert_eq!(sister.sessions.len(), 1);
    }
}
