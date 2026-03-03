//! The 24 inventions of AgenticCognition
//!
//! Organized by priority tier:
//! - P0: Living User Model, Belief Graph, Decision Fingerprint, Soul Reflection
//! - P1: Crystallization, Self-Concept, Drift, Preference Oracle
//! - P2: Shadow Beliefs, Projections, Blindspots, Bias Field, Triggers
//! - P3: Entanglement, Gravity, Collapse, Value Tectonics, Metamorphosis
//! - P4: Fossils, Strata, Simulation, Future Projection, Identity Thread

pub mod belief_physics;
pub mod drift_tracker;
pub mod future_projection;
pub mod living_model;
pub mod preference_oracle;
pub mod shadow;
pub mod simulation;
pub mod soul_reflection;

pub use belief_physics::BeliefPhysics;
pub use drift_tracker::DriftTracker;
pub use future_projection::FutureProjector;
pub use living_model::LivingModelManager;
pub use preference_oracle::PreferenceOracle;
pub use shadow::ShadowDetector;
pub use simulation::DecisionSimulator;
pub use soul_reflection::SoulReflector;
