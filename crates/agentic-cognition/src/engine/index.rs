//! Index management — fast lookups by domain, confidence, state

use crate::types::*;
use std::collections::HashMap;

/// Manages indexes for fast belief/model lookups
#[derive(Debug, Default)]
pub struct IndexManager {
    pub beliefs_by_domain: HashMap<BeliefDomain, Vec<BeliefId>>,
    pub beliefs_by_state: HashMap<BeliefState, Vec<BeliefId>>,
    pub high_confidence_beliefs: Vec<BeliefId>,
    pub crystallized_beliefs: Vec<BeliefId>,
}

impl IndexManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Rebuild all indexes from the belief graph
    pub fn rebuild(&mut self, graph: &BeliefGraph) {
        self.beliefs_by_domain.clear();
        self.beliefs_by_state.clear();
        self.high_confidence_beliefs.clear();
        self.crystallized_beliefs.clear();

        for (id, belief) in &graph.beliefs {
            self.beliefs_by_domain
                .entry(belief.domain)
                .or_default()
                .push(*id);

            self.beliefs_by_state
                .entry(belief.state)
                .or_default()
                .push(*id);

            if belief.confidence >= 0.8 {
                self.high_confidence_beliefs.push(*id);
            }

            if belief.crystallization >= 0.7 {
                self.crystallized_beliefs.push(*id);
            }
        }
    }

    /// Add a single belief to indexes
    pub fn index_belief(&mut self, belief: &Belief) {
        self.beliefs_by_domain
            .entry(belief.domain)
            .or_default()
            .push(belief.id);

        self.beliefs_by_state
            .entry(belief.state)
            .or_default()
            .push(belief.id);

        if belief.confidence >= 0.8 {
            self.high_confidence_beliefs.push(belief.id);
        }

        if belief.crystallization >= 0.7 {
            self.crystallized_beliefs.push(belief.id);
        }
    }

    /// Remove a belief from indexes
    pub fn remove_belief(&mut self, belief: &Belief) {
        if let Some(ids) = self.beliefs_by_domain.get_mut(&belief.domain) {
            ids.retain(|id| id != &belief.id);
        }
        if let Some(ids) = self.beliefs_by_state.get_mut(&belief.state) {
            ids.retain(|id| id != &belief.id);
        }
        self.high_confidence_beliefs.retain(|id| id != &belief.id);
        self.crystallized_beliefs.retain(|id| id != &belief.id);
    }

    pub fn get_by_domain(&self, domain: &BeliefDomain) -> &[BeliefId] {
        self.beliefs_by_domain
            .get(domain)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn get_by_state(&self, state: &BeliefState) -> &[BeliefId] {
        self.beliefs_by_state
            .get(state)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn get_high_confidence(&self) -> &[BeliefId] {
        &self.high_confidence_beliefs
    }

    pub fn get_crystallized(&self) -> &[BeliefId] {
        &self.crystallized_beliefs
    }
}
