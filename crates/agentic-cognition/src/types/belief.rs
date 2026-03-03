//! Belief system types

use crate::types::ids::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single belief
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Belief {
    pub id: BeliefId,
    pub content: String,
    pub domain: BeliefDomain,
    pub confidence: f64,
    pub centrality: f64,
    pub first_observed: Timestamp,
    pub last_reinforced: Timestamp,
    pub crystallization: f64,
    pub evidence_basis: EvidenceBasis,
    pub emotional_charge: f64,
    pub origin: BeliefOrigin,
    pub explicit: bool,
    pub state: BeliefState,
}

impl Belief {
    pub fn new(content: String, domain: BeliefDomain, confidence: f64) -> Self {
        let now = Timestamp::now();
        Self {
            id: BeliefId::new(),
            content,
            domain,
            confidence,
            centrality: 0.0,
            first_observed: now,
            last_reinforced: now,
            crystallization: 0.0,
            evidence_basis: EvidenceBasis::default(),
            emotional_charge: 0.0,
            origin: BeliefOrigin::Unknown,
            explicit: true,
            state: BeliefState::Forming,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BeliefDomain {
    #[serde(rename = "self")]
    Self_,
    Relationships,
    Work,
    Politics,
    Religion,
    Science,
    Values,
    WorldModel,
    Identity,
    Capability,
    Worth,
    Other,
}

impl std::fmt::Display for BeliefDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BeliefDomain::Self_ => write!(f, "self"),
            BeliefDomain::Relationships => write!(f, "relationships"),
            BeliefDomain::Work => write!(f, "work"),
            BeliefDomain::Politics => write!(f, "politics"),
            BeliefDomain::Religion => write!(f, "religion"),
            BeliefDomain::Science => write!(f, "science"),
            BeliefDomain::Values => write!(f, "values"),
            BeliefDomain::WorldModel => write!(f, "world_model"),
            BeliefDomain::Identity => write!(f, "identity"),
            BeliefDomain::Capability => write!(f, "capability"),
            BeliefDomain::Worth => write!(f, "worth"),
            BeliefDomain::Other => write!(f, "other"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EvidenceBasis {
    pub experiences: u32,
    pub testimonials: u32,
    pub reasoning: u32,
    pub intuition: u32,
    pub strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BeliefOrigin {
    Family { who: String },
    Culture { culture: String },
    Experience { experience: String },
    Reasoning,
    Emotional { trigger: String },
    Authority { authority: String },
    Social { group: String },
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BeliefState {
    Forming,
    Strengthening,
    Crystallized,
    Challenged,
    Collapsing,
    Collapsed,
}

/// Connection between two beliefs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefConnection {
    pub from: BeliefId,
    pub to: BeliefId,
    pub connection_type: ConnectionType,
    pub strength: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionType {
    Supports,
    Contradicts,
    Requires,
    Implies,
    Associated,
    Generalizes,
    Specializes,
}

/// A keystone belief
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeystoneBelief {
    pub belief_id: BeliefId,
    pub dependents: Vec<BeliefId>,
    pub collapse_radius: f64,
    pub stability: f64,
}

/// Contradiction between beliefs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefContradiction {
    pub belief_a: BeliefId,
    pub belief_b: BeliefId,
    pub contradiction_type: ContradictionType,
    pub severity: f64,
    pub conscious: bool,
    pub resolution_strategy: Option<ContradictionStrategy>,
    pub tension: f64,
    pub detected_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContradictionType {
    Direct,
    Practical,
    ValueConflict,
    TemporalShift,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContradictionStrategy {
    Compartmentalization,
    Rationalization,
    Denial,
    Integration,
    ValueHierarchy,
    ContextDependence,
}

/// Crystallization tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefCrystallization {
    pub belief_id: BeliefId,
    pub level: f64,
    pub factors: CrystallizationFactors,
    pub modification_energy: f64,
    pub brittleness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystallizationFactors {
    pub age_days: u32,
    pub reinforcement_count: u32,
    pub emotional_investment: f64,
    pub identity_integration: f64,
    pub social_reinforcement: f64,
    pub defense_count: u32,
    pub public_commitment: f64,
}

/// Belief entanglement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefEntanglement {
    pub id: EntanglementId,
    pub beliefs: Vec<BeliefId>,
    pub entanglement_type: EntanglementType,
    pub strength: f64,
    pub conscious: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntanglementType {
    Correlated,
    AntiCorrelated,
    Protective,
    MutuallyDefining,
    Hidden { surface_distance: f64 },
}

/// The full belief graph
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeliefGraph {
    pub beliefs: HashMap<BeliefId, Belief>,
    pub connections: Vec<BeliefConnection>,
    pub entanglements: Vec<BeliefEntanglement>,
}

impl BeliefGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_belief(&mut self, belief: Belief) {
        self.beliefs.insert(belief.id, belief);
    }

    pub fn get_belief(&self, id: &BeliefId) -> Option<&Belief> {
        self.beliefs.get(id)
    }

    pub fn get_belief_mut(&mut self, id: &BeliefId) -> Option<&mut Belief> {
        self.beliefs.get_mut(id)
    }

    pub fn add_connection(&mut self, connection: BeliefConnection) {
        self.connections.push(connection);
    }

    pub fn beliefs_in_domain(&self, domain: &BeliefDomain) -> Vec<&Belief> {
        self.beliefs
            .values()
            .filter(|b| &b.domain == domain)
            .collect()
    }

    pub fn find_contradictions(&self) -> Vec<BeliefContradiction> {
        let mut contradictions = Vec::new();
        for conn in &self.connections {
            if conn.connection_type == ConnectionType::Contradicts {
                contradictions.push(BeliefContradiction {
                    belief_a: conn.from,
                    belief_b: conn.to,
                    contradiction_type: ContradictionType::Direct,
                    severity: conn.strength,
                    conscious: true,
                    resolution_strategy: None,
                    tension: conn.strength,
                    detected_at: Timestamp::now(),
                });
            }
        }
        contradictions
    }

    pub fn find_keystones(&self) -> Vec<KeystoneBelief> {
        let mut dependency_count: HashMap<BeliefId, Vec<BeliefId>> = HashMap::new();
        for conn in &self.connections {
            if conn.connection_type == ConnectionType::Requires {
                dependency_count.entry(conn.to).or_default().push(conn.from);
            }
        }
        dependency_count
            .into_iter()
            .filter(|(_, deps)| deps.len() >= 2)
            .map(|(id, deps)| {
                let belief = self.beliefs.get(&id);
                KeystoneBelief {
                    belief_id: id,
                    collapse_radius: deps.len() as f64 / self.beliefs.len().max(1) as f64,
                    dependents: deps,
                    stability: belief.map(|b| b.confidence).unwrap_or(0.5),
                }
            })
            .collect()
    }
}
