//! Query engine — all read operations for cognition models

use crate::engine::store::CognitionStore;
use crate::inventions::*;
use crate::types::*;

/// Engine for all query/read operations
pub struct QueryEngine {
    store: CognitionStore,
}

impl QueryEngine {
    pub fn new(store: CognitionStore) -> Self {
        Self { store }
    }

    pub fn store(&self) -> &CognitionStore {
        &self.store
    }

    // --- Model Queries ---

    /// Get a model by ID
    pub fn get_model(&self, model_id: &ModelId) -> CognitionResult<LivingUserModel> {
        let file = self.store.get_model(model_id)?;
        Ok(file.model)
    }

    /// Get model vitals
    pub fn get_vitals(&self, model_id: &ModelId) -> CognitionResult<ModelVitals> {
        let file = self.store.get_model(model_id)?;
        Ok(file.model.vitals)
    }

    /// Get full model portrait (everything)
    pub fn get_portrait(&self, model_id: &ModelId) -> CognitionResult<ModelPortrait> {
        let file = self.store.get_model(model_id)?;
        Ok(ModelPortrait {
            model: file.model.clone(),
            belief_count: file.belief_graph.beliefs.len(),
            shadow_count: file.shadow.shadow_beliefs.len(),
            bias_count: file.bias_field.biases.len(),
            drift_event_count: file.drift.events.len(),
            has_fingerprint: file.fingerprint.is_some(),
        })
    }

    /// List all models
    pub fn list_models(&self) -> CognitionResult<Vec<ModelId>> {
        self.store.list_models()
    }

    // --- Belief Queries ---

    /// Get a specific belief
    pub fn get_belief(&self, model_id: &ModelId, belief_id: &BeliefId) -> CognitionResult<Belief> {
        let file = self.store.get_model(model_id)?;
        file.belief_graph
            .get_belief(belief_id)
            .cloned()
            .ok_or(CognitionError::BeliefNotFound(*belief_id))
    }

    /// List all beliefs for a model
    pub fn list_beliefs(&self, model_id: &ModelId) -> CognitionResult<Vec<Belief>> {
        let file = self.store.get_model(model_id)?;
        Ok(file.belief_graph.beliefs.values().cloned().collect())
    }

    /// List beliefs in a specific domain
    pub fn beliefs_by_domain(
        &self,
        model_id: &ModelId,
        domain: &BeliefDomain,
    ) -> CognitionResult<Vec<Belief>> {
        let file = self.store.get_model(model_id)?;
        Ok(file
            .belief_graph
            .beliefs_in_domain(domain)
            .into_iter()
            .cloned()
            .collect())
    }

    /// Get the belief graph
    pub fn get_belief_graph(&self, model_id: &ModelId) -> CognitionResult<BeliefGraph> {
        let file = self.store.get_model(model_id)?;
        Ok(file.belief_graph)
    }

    /// Get keystone beliefs
    pub fn get_keystones(&self, model_id: &ModelId) -> CognitionResult<Vec<KeystoneBelief>> {
        let file = self.store.get_model(model_id)?;
        Ok(file.belief_graph.find_keystones())
    }

    /// Get contradictions
    pub fn get_contradictions(
        &self,
        model_id: &ModelId,
    ) -> CognitionResult<Vec<BeliefContradiction>> {
        let file = self.store.get_model(model_id)?;
        Ok(file.belief_graph.find_contradictions())
    }

    /// Search beliefs by content
    pub fn search_beliefs(&self, model_id: &ModelId, query: &str) -> CognitionResult<Vec<Belief>> {
        let file = self.store.get_model(model_id)?;
        let query_lower = query.to_lowercase();
        Ok(file
            .belief_graph
            .beliefs
            .values()
            .filter(|b| b.content.to_lowercase().contains(&query_lower))
            .cloned()
            .collect())
    }

    // --- Self-Concept Queries ---

    /// Get self-concept topology
    pub fn get_topology(&self, model_id: &ModelId) -> CognitionResult<SelfConceptTopology> {
        let file = self.store.get_model(model_id)?;
        Ok(file.model.self_concept)
    }

    // --- Pattern Queries ---

    /// Get decision fingerprint
    pub fn get_fingerprint(
        &self,
        model_id: &ModelId,
    ) -> CognitionResult<Option<DecisionFingerprint>> {
        let file = self.store.get_model(model_id)?;
        Ok(file.fingerprint)
    }

    // --- Shadow Queries ---

    /// Get shadow map
    pub fn get_shadow_map(&self, model_id: &ModelId) -> CognitionResult<ShadowMap> {
        let file = self.store.get_model(model_id)?;
        Ok(file.shadow)
    }

    // --- Bias Queries ---

    /// Get bias field
    pub fn get_bias_field(&self, model_id: &ModelId) -> CognitionResult<BiasField> {
        let file = self.store.get_model(model_id)?;
        Ok(file.bias_field)
    }

    // --- Drift Queries ---

    /// Get drift timeline
    pub fn get_drift_timeline(&self, model_id: &ModelId) -> CognitionResult<DriftTimeline> {
        let file = self.store.get_model(model_id)?;
        Ok(file.drift)
    }

    // --- Invention Queries ---

    /// Perform soul reflection
    pub fn soul_reflection(&self, model_id: &ModelId) -> CognitionResult<SoulReflection> {
        let file = self.store.get_model(model_id)?;
        Ok(SoulReflector::reflect(&file.model, &file.belief_graph))
    }

    /// Predict preference
    pub fn predict_preference(
        &self,
        model_id: &ModelId,
        item: &str,
    ) -> CognitionResult<PreferencePrediction> {
        let file = self.store.get_model(model_id)?;
        Ok(PreferenceOracle::predict(
            &file.model,
            &file.belief_graph,
            &file.fingerprint,
            item,
        ))
    }

    /// Simulate decision
    pub fn simulate_decision(
        &self,
        model_id: &ModelId,
        scenario: &str,
        options: &[String],
    ) -> CognitionResult<DecisionSimulation> {
        let file = self.store.get_model(model_id)?;
        Ok(DecisionSimulator::simulate(
            &file.model,
            &file.belief_graph,
            &file.fingerprint,
            scenario,
            options,
        ))
    }

    /// Project future self
    pub fn project_future(
        &self,
        model_id: &ModelId,
        days: u32,
    ) -> CognitionResult<FutureProjection> {
        let file = self.store.get_model(model_id)?;
        Ok(FutureProjector::project(
            &file.model,
            &file.belief_graph,
            &file.drift,
            days,
        ))
    }

    /// Get consciousness state
    pub fn get_consciousness(&self, model_id: &ModelId) -> CognitionResult<ConsciousnessState> {
        let file = self.store.get_model(model_id)?;
        Ok(file.model.consciousness)
    }

    /// Get soul data
    pub fn get_soul(&self, model_id: &ModelId) -> CognitionResult<ModelSoul> {
        let file = self.store.get_model(model_id)?;
        Ok(file.model.soul)
    }
}

/// Summary portrait of a model
#[derive(Debug, Clone)]
pub struct ModelPortrait {
    pub model: LivingUserModel,
    pub belief_count: usize,
    pub shadow_count: usize,
    pub bias_count: usize,
    pub drift_event_count: usize,
    pub has_fingerprint: bool,
}
