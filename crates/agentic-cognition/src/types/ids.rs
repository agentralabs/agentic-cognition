//! Core identifiers for AgenticCognition

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Unique identifier for a user model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModelId(pub Uuid);

/// Unique identifier for a belief
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BeliefId(pub Uuid);

/// Unique identifier for a pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PatternId(pub Uuid);

/// Unique identifier for a drift event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DriftId(pub Uuid);

/// Unique identifier for a blindspot
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlindspotId(pub Uuid);

/// Unique identifier for a projection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectionId(pub Uuid);

/// Unique identifier for a bias
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BiasId(pub Uuid);

/// Unique identifier for a trigger
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TriggerId(pub Uuid);

/// Unique identifier for a prediction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PredictionId(pub Uuid);

/// Unique identifier for a simulation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SimulationId(pub Uuid);

/// Unique identifier for a collapse event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CollapseId(pub Uuid);

/// Unique identifier for an entanglement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntanglementId(pub Uuid);

/// Unique identifier for a fingerprint
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FingerprintId(pub Uuid);

/// Unique identifier for a thread
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThreadId(pub Uuid);

/// Unique identifier for a reflection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReflectionId(pub Uuid);

/// Unique identifier for a fossil
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FossilId(pub Uuid);

/// Unique identifier for a stratum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StratumId(pub Uuid);

/// Timestamp in nanoseconds since Unix epoch
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Timestamp(pub i64);

impl Timestamp {
    pub fn now() -> Self {
        Self(chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0))
    }

    pub fn epoch() -> Self {
        Self(0)
    }

    pub fn as_nanos(&self) -> i64 {
        self.0
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

macro_rules! impl_id {
    ($name:ident) => {
        impl $name {
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            pub fn from_uuid(uuid: Uuid) -> Self {
                Self(uuid)
            }

            pub fn as_uuid(&self) -> &Uuid {
                &self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::str::FromStr for $name {
            type Err = uuid::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(Uuid::parse_str(s)?))
            }
        }
    };
}

impl_id!(ModelId);
impl_id!(BeliefId);
impl_id!(PatternId);
impl_id!(DriftId);
impl_id!(BlindspotId);
impl_id!(ProjectionId);
impl_id!(BiasId);
impl_id!(TriggerId);
impl_id!(PredictionId);
impl_id!(SimulationId);
impl_id!(CollapseId);
impl_id!(EntanglementId);
impl_id!(FingerprintId);
impl_id!(ThreadId);
impl_id!(ReflectionId);
impl_id!(FossilId);
impl_id!(StratumId);
