//! AgenticCognition — The Mirror
//!
//! Longitudinal user modeling — living models of human consciousness for AI agents.
//! Sister #9 of 25 in the Agentra ecosystem.
//!
//! ## Modules
//!
//! - `types`      — Core domain types (ids, beliefs, soul, consciousness, etc.)
//! - `engine`     — Write/query engines and the CognitionStore
//! - `inventions` — Novel algorithms (soul reflection, preference oracle, simulation)
//! - `bridges`    — Sister bridge traits and NoOp defaults
//! - `contracts`  — agentic-sdk v0.2.0 trait implementations (CognitionSister)
//! - `format`     — .acog binary file format

pub mod audit;
pub mod bridges;
pub mod contracts;
pub mod engine;
pub mod format;
pub mod inventions;
pub mod storage;
pub mod types;
pub mod validation;

// Re-export commonly used types
pub use types::{
    Belief, BeliefConnection, BeliefContradiction, BeliefCrystallization, BeliefDomain,
    BeliefEntanglement, BeliefGraph, BeliefId, BeliefState, BiasField, BiasId, BlindspotId,
    CognitionError, CognitionResult, CollapseId, ConnectionType, ConsciousnessState, ConsentStatus,
    DecisionFingerprint, DriftId, DriftTimeline, EntanglementId, FingerprintId, FossilId,
    KeystoneBelief, LivingUserModel, ModelId, ModelLifecycleStage, ModelSoul, ModelVitals,
    PatternId, PredictionId, PrivacySettings, ProjectionId, ReflectionId, SelfConceptTopology,
    ShadowMap, SimulationId, StratumId, ThreadId, Timestamp, TriggerId,
};

pub use contracts::CognitionSister;
pub use engine::{CognitionStore, QueryEngine, WriteEngine};
