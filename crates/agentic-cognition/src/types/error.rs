//! Error types for AgenticCognition

use crate::types::ids::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CognitionError {
    #[error("Model not found: {0}")]
    ModelNotFound(ModelId),

    #[error("Belief not found: {0}")]
    BeliefNotFound(BeliefId),

    #[error("Pattern not found: {0}")]
    PatternNotFound(PatternId),

    #[error("Invalid confidence value: {0} (must be 0.0-1.0)")]
    InvalidConfidence(f64),

    #[error("Invalid model state transition: {from:?} -> {to:?}")]
    InvalidStateTransition { from: String, to: String },

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("File format error: {0}")]
    FormatError(String),

    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Lock error: {0}")]
    LockError(String),

    #[error("Duplicate belief: {0}")]
    DuplicateBelief(BeliefId),

    #[error("Self-connection not allowed for belief: {0}")]
    SelfConnection(BeliefId),

    #[error("Connection already exists between {0} and {1}")]
    DuplicateConnection(BeliefId, BeliefId),

    #[error("Model already exists: {0}")]
    ModelAlreadyExists(ModelId),

    #[error("Bridge error: {0}")]
    BridgeError(String),

    #[error("Prediction error: {0}")]
    PredictionError(String),

    #[error("Simulation error: {0}")]
    SimulationError(String),

    #[error("Auth error: {0}")]
    AuthError(String),
}

pub type CognitionResult<T> = Result<T, CognitionError>;
