//! Write engine — all mutation operations for cognition models

use crate::bridges::BridgeSet;
use crate::engine::store::CognitionStore;
use crate::engine::validation::Validator;
use crate::format::AcogFile;
use crate::types::*;

/// Engine for all write operations
pub struct WriteEngine {
    store: CognitionStore,
    bridges: BridgeSet,
}

impl WriteEngine {
    pub fn new(store: CognitionStore) -> Self {
        Self {
            store,
            bridges: BridgeSet::default(),
        }
    }

    pub fn with_bridges(mut self, bridges: BridgeSet) -> Self {
        self.bridges = bridges;
        self
    }

    pub fn store(&self) -> &CognitionStore {
        &self.store
    }

    // --- Model Operations ---

    /// Create a new user model
    pub fn create_model(&self) -> CognitionResult<ModelId> {
        let model = LivingUserModel::new();
        let file = AcogFile::new(model);
        self.store.insert_model(file)
    }

    /// Heartbeat — pulse the model with observations
    pub fn heartbeat(&self, model_id: &ModelId, observations: Vec<String>) -> CognitionResult<()> {
        let now = Timestamp::now();
        self.store.update_model(model_id, |file| {
            file.model.last_heartbeat = now;
            file.model.updated_at = now;
            file.model.evidence_count += observations.len() as u64;

            // Update vitals
            file.model.vitals.evidence_count = file.model.evidence_count;
            file.model.vitals.staleness_secs = 0;
            file.model.vitals.last_significant_update = now;

            // Auto-transition lifecycle if needed
            match file.model.lifecycle_stage {
                ModelLifecycleStage::Birth if file.model.evidence_count > 5 => {
                    file.model.lifecycle_stage = ModelLifecycleStage::Infancy;
                }
                ModelLifecycleStage::Infancy if file.model.evidence_count > 50 => {
                    file.model.lifecycle_stage = ModelLifecycleStage::Growth;
                }
                ModelLifecycleStage::Growth if file.model.evidence_count > 200 => {
                    file.model.lifecycle_stage = ModelLifecycleStage::Maturity;
                }
                _ => {}
            }
        })
    }

    /// Transition lifecycle stage
    pub fn transition_lifecycle(
        &self,
        model_id: &ModelId,
        target: ModelLifecycleStage,
    ) -> CognitionResult<()> {
        let file = self.store.get_model(model_id)?;
        Validator::validate_lifecycle_transition(&file.model.lifecycle_stage, &target)?;

        self.store.update_model(model_id, |file| {
            file.model.lifecycle_stage = target;
            file.model.updated_at = Timestamp::now();
        })
    }

    // --- Belief Operations ---

    /// Add a new belief to the model
    pub fn add_belief(
        &self,
        model_id: &ModelId,
        content: String,
        domain: BeliefDomain,
        confidence: f64,
    ) -> CognitionResult<BeliefId> {
        Validator::validate_non_empty("content", &content)?;
        Validator::validate_confidence(confidence)?;

        let belief = Belief::new(content, domain, confidence);
        let belief_id = belief.id;

        self.store.update_model(model_id, |file| {
            file.belief_graph.add_belief(belief);
            file.model.updated_at = Timestamp::now();
            file.model.evidence_count += 1;
        })?;

        Ok(belief_id)
    }

    /// Strengthen a belief
    pub fn strengthen_belief(
        &self,
        model_id: &ModelId,
        belief_id: &BeliefId,
        amount: f64,
    ) -> CognitionResult<()> {
        Validator::validate_confidence(amount)?;

        self.store.update_model(model_id, |file| {
            if let Some(belief) = file.belief_graph.get_belief_mut(belief_id) {
                belief.confidence = (belief.confidence + amount).min(1.0);
                belief.last_reinforced = Timestamp::now();
                belief.evidence_basis.experiences += 1;
                belief.evidence_basis.strength = (belief.evidence_basis.strength + 0.1).min(1.0);

                if belief.confidence > 0.8 && belief.state == BeliefState::Forming {
                    belief.state = BeliefState::Strengthening;
                }
            }
            file.model.updated_at = Timestamp::now();
        })
    }

    /// Weaken a belief
    pub fn weaken_belief(
        &self,
        model_id: &ModelId,
        belief_id: &BeliefId,
        amount: f64,
    ) -> CognitionResult<()> {
        Validator::validate_confidence(amount)?;

        self.store.update_model(model_id, |file| {
            if let Some(belief) = file.belief_graph.get_belief_mut(belief_id) {
                belief.confidence = (belief.confidence - amount).max(0.0);
                belief.last_reinforced = Timestamp::now();

                if belief.confidence < 0.2 {
                    belief.state = BeliefState::Collapsing;
                } else if belief.state == BeliefState::Crystallized {
                    belief.state = BeliefState::Challenged;
                }

                // Record drift event
                file.drift.events.push(DriftEvent {
                    id: DriftId::new(),
                    belief_id: *belief_id,
                    timestamp: Timestamp::now(),
                    direction: DriftDirection::Weakening,
                    magnitude: amount,
                    cause: DriftCause::Unknown,
                    previous_confidence: belief.confidence + amount,
                    new_confidence: belief.confidence,
                });
            }
            file.model.updated_at = Timestamp::now();
        })
    }

    /// Crystallize a belief
    pub fn crystallize_belief(
        &self,
        model_id: &ModelId,
        belief_id: &BeliefId,
    ) -> CognitionResult<()> {
        self.store.update_model(model_id, |file| {
            if let Some(belief) = file.belief_graph.get_belief_mut(belief_id) {
                belief.crystallization = 1.0;
                belief.state = BeliefState::Crystallized;
            }
            file.model.updated_at = Timestamp::now();
        })
    }

    /// Collapse a belief
    pub fn collapse_belief(
        &self,
        model_id: &ModelId,
        belief_id: &BeliefId,
        trigger: CollapseTrigger,
    ) -> CognitionResult<()> {
        self.store.update_model(model_id, |file| {
            if let Some(belief) = file.belief_graph.get_belief_mut(belief_id) {
                belief.state = BeliefState::Collapsed;
                belief.confidence = 0.0;
            }

            // Record collapse
            let _collapse = CertaintyCollapse {
                id: CollapseId::new(),
                collapsed_belief: *belief_id,
                timestamp: Timestamp::now(),
                trigger,
                cascade: Vec::new(),
                identity_impact: 0.5,
                recovery_progress: 0.0,
                replacement: None,
            };

            // Check for cascade
            let dependents: Vec<BeliefId> = file
                .belief_graph
                .connections
                .iter()
                .filter(|c| c.to == *belief_id && c.connection_type == ConnectionType::Requires)
                .map(|c| c.from)
                .collect();

            for dep_id in &dependents {
                if let Some(dep) = file.belief_graph.get_belief_mut(dep_id) {
                    dep.state = BeliefState::Challenged;
                    dep.confidence *= 0.5;
                }
            }

            file.drift.events.push(DriftEvent {
                id: DriftId::new(),
                belief_id: *belief_id,
                timestamp: Timestamp::now(),
                direction: DriftDirection::Reversing,
                magnitude: 1.0,
                cause: DriftCause::Unknown,
                previous_confidence: 1.0,
                new_confidence: 0.0,
            });

            file.model.updated_at = Timestamp::now();
        })
    }

    /// Connect two beliefs
    pub fn connect_beliefs(
        &self,
        model_id: &ModelId,
        from: BeliefId,
        to: BeliefId,
        connection_type: ConnectionType,
        strength: f64,
    ) -> CognitionResult<()> {
        if from == to {
            return Err(CognitionError::SelfConnection(from));
        }
        Validator::validate_confidence(strength)?;

        self.store.update_model(model_id, |file| {
            file.belief_graph.add_connection(BeliefConnection {
                from,
                to,
                connection_type,
                strength,
            });
            file.model.updated_at = Timestamp::now();
        })
    }

    /// Entangle two beliefs
    pub fn entangle_beliefs(
        &self,
        model_id: &ModelId,
        beliefs: Vec<BeliefId>,
        entanglement_type: EntanglementType,
        strength: f64,
    ) -> CognitionResult<EntanglementId> {
        Validator::validate_confidence(strength)?;

        let ent_id = EntanglementId::new();
        self.store.update_model(model_id, |file| {
            file.belief_graph.entanglements.push(BeliefEntanglement {
                id: ent_id,
                beliefs,
                entanglement_type,
                strength,
                conscious: false,
            });
            file.model.updated_at = Timestamp::now();
        })?;

        Ok(ent_id)
    }

    // --- Self-Concept Operations ---

    /// Add a confidence peak
    pub fn add_peak(
        &self,
        model_id: &ModelId,
        domain: String,
        height: f64,
        warranted: bool,
    ) -> CognitionResult<()> {
        Validator::validate_confidence(height)?;
        Validator::validate_non_empty("domain", &domain)?;

        self.store.update_model(model_id, |file| {
            file.model.self_concept.peaks.push(ConfidencePeak {
                domain,
                height,
                stability: 0.5,
                warranted,
                reality_gap: if warranted { None } else { Some(0.3) },
            });
            file.model.updated_at = Timestamp::now();
        })
    }

    /// Add an insecurity valley
    pub fn add_valley(
        &self,
        model_id: &ModelId,
        domain: String,
        depth: f64,
    ) -> CognitionResult<()> {
        Validator::validate_confidence(depth)?;

        self.store.update_model(model_id, |file| {
            file.model.self_concept.valleys.push(InsecurityValley {
                domain,
                depth,
                self_aware: true,
                compensation_strategy: None,
                origin: None,
            });
            file.model.updated_at = Timestamp::now();
        })
    }

    /// Add a blindspot
    pub fn add_blindspot(
        &self,
        model_id: &ModelId,
        area: String,
        blindness: f64,
    ) -> CognitionResult<()> {
        Validator::validate_confidence(blindness)?;

        self.store.update_model(model_id, |file| {
            file.model.self_concept.blind_canyons.push(BlindCanyon {
                blind_area: area,
                blindness,
                evidence: Vec::new(),
                impact: BlindnessImpact::Medium,
                penetrability: 1.0 - blindness,
            });
            file.model.updated_at = Timestamp::now();
        })
    }

    /// Add a defended territory
    pub fn add_defended_territory(
        &self,
        model_id: &ModelId,
        territory: String,
        strength: f64,
        vulnerability: String,
    ) -> CognitionResult<()> {
        Validator::validate_confidence(strength)?;

        self.store.update_model(model_id, |file| {
            file.model
                .self_concept
                .defended_territories
                .push(DefendedTerritory {
                    territory,
                    defense_strength: strength,
                    triggers: Vec::new(),
                    mechanisms: Vec::new(),
                    underlying_vulnerability: vulnerability,
                });
            file.model.updated_at = Timestamp::now();
        })
    }

    /// Add a growing edge
    pub fn add_growing_edge(
        &self,
        model_id: &ModelId,
        area: String,
        growth_rate: f64,
    ) -> CognitionResult<()> {
        Validator::validate_confidence(growth_rate)?;

        self.store.update_model(model_id, |file| {
            file.model.self_concept.growing_edges.push(GrowingEdge {
                area,
                growth_rate,
                challenge_level: 0.5,
                support_needed: String::new(),
                since: Timestamp::now(),
            });
            file.model.updated_at = Timestamp::now();
        })
    }

    // --- Pattern Operations ---

    /// Update decision fingerprint
    pub fn update_fingerprint(
        &self,
        model_id: &ModelId,
        traits: DecisionTraits,
    ) -> CognitionResult<()> {
        self.store.update_model(model_id, |file| {
            let fp = file
                .fingerprint
                .get_or_insert_with(|| DecisionFingerprint::new(file.model.id));
            fp.traits = traits;
            fp.updated_at = Timestamp::now();
            fp.confidence = (fp.confidence + 0.1).min(1.0);
            file.model.updated_at = Timestamp::now();
        })
    }

    /// Add a reasoning fossil
    pub fn add_fossil(
        &self,
        model_id: &ModelId,
        pattern: String,
        origin_period: String,
        influence: f64,
    ) -> CognitionResult<FossilId> {
        Validator::validate_non_empty("pattern", &pattern)?;
        Validator::validate_confidence(influence)?;

        let fossil_id = FossilId::new();
        // Fossils are stored in the drift timeline as part of pattern archaeology
        // For now, we track them as growth rings
        self.store.update_model(model_id, |file| {
            file.drift.growth_rings.push(GrowthRing {
                period: origin_period,
                lessons: vec![pattern],
                beliefs_formed: Vec::new(),
                beliefs_abandoned: Vec::new(),
                identity_changes: Vec::new(),
                started_at: Timestamp::now(),
                ended_at: None,
            });
            file.model.updated_at = Timestamp::now();
        })?;

        Ok(fossil_id)
    }

    // --- Shadow Operations ---

    /// Add a shadow belief
    pub fn add_shadow_belief(
        &self,
        model_id: &ModelId,
        content: String,
        strength: f64,
        contradicts: Option<BeliefId>,
    ) -> CognitionResult<BeliefId> {
        Validator::validate_non_empty("content", &content)?;
        Validator::validate_confidence(strength)?;

        let shadow_id = BeliefId::new();
        self.store.update_model(model_id, |file| {
            file.shadow.shadow_beliefs.push(ShadowBelief {
                id: shadow_id,
                content,
                evidence: Vec::new(),
                strength,
                contradicts_conscious: contradicts,
                behavioral_signs: Vec::new(),
                detected_at: Timestamp::now(),
            });
            file.model.updated_at = Timestamp::now();
        })?;

        Ok(shadow_id)
    }

    /// Add a projection
    pub fn add_projection(
        &self,
        model_id: &ModelId,
        disowned_trait: String,
        projected_onto: String,
    ) -> CognitionResult<ProjectionId> {
        let proj_id = ProjectionId::new();
        self.store.update_model(model_id, |file| {
            file.shadow.projections.push(Projection {
                id: proj_id,
                disowned_trait: disowned_trait.clone(),
                projected_onto,
                strength: 0.5,
                evidence: Vec::new(),
                original_self_trait: disowned_trait,
                detected_at: Timestamp::now(),
            });
            file.model.updated_at = Timestamp::now();
        })?;

        Ok(proj_id)
    }

    // --- Bias Operations ---

    /// Add an active bias
    pub fn add_bias(
        &self,
        model_id: &ModelId,
        name: String,
        bias_type: BiasType,
        strength: f64,
    ) -> CognitionResult<BiasId> {
        Validator::validate_confidence(strength)?;

        let bias_id = BiasId::new();
        self.store.update_model(model_id, |file| {
            file.bias_field.biases.push(ActiveBias {
                id: bias_id,
                name,
                bias_type,
                strength,
                domains_affected: Vec::new(),
                evidence: Vec::new(),
                self_aware: false,
                detected_at: Timestamp::now(),
            });
            file.model.updated_at = Timestamp::now();
        })?;

        Ok(bias_id)
    }

    /// Add an emotional trigger
    pub fn add_trigger(
        &self,
        model_id: &ModelId,
        trigger: String,
        response: String,
        intensity: f64,
    ) -> CognitionResult<TriggerId> {
        Validator::validate_confidence(intensity)?;

        let trigger_id = TriggerId::new();
        self.store.update_model(model_id, |file| {
            file.bias_field.triggers.push(EmotionalTrigger {
                id: trigger_id,
                trigger,
                response_pattern: response,
                intensity,
                origin: None,
                coping_strategy: None,
                detected_at: Timestamp::now(),
            });
            file.model.updated_at = Timestamp::now();
        })?;

        Ok(trigger_id)
    }

    // --- Drift Operations ---

    /// Add a value tectonic shift
    pub fn add_value_tectonic(
        &self,
        model_id: &ModelId,
        value: String,
        direction: String,
        magnitude: f64,
    ) -> CognitionResult<()> {
        Validator::validate_confidence(magnitude)?;

        self.store.update_model(model_id, |file| {
            file.drift.value_tectonics.push(ValueTectonic {
                value,
                direction,
                magnitude,
                started_at: Timestamp::now(),
                last_observed: Timestamp::now(),
                evidence: Vec::new(),
            });
            file.model.updated_at = Timestamp::now();
        })
    }

    /// Record a metamorphosis
    pub fn add_metamorphosis(
        &self,
        model_id: &ModelId,
        description: String,
        triggered_by: String,
        before: String,
        after: String,
    ) -> CognitionResult<()> {
        self.store.update_model(model_id, |file| {
            file.drift.metamorphoses.push(Metamorphosis {
                description,
                triggered_by,
                before_identity: before,
                after_identity: after,
                started_at: Timestamp::now(),
                completed_at: None,
                progress: 0.0,
            });
            file.model.updated_at = Timestamp::now();
            file.model.lifecycle_stage = ModelLifecycleStage::Crisis;
        })
    }

    // --- Consciousness Operations ---

    /// Update emotional weather
    pub fn update_emotional_weather(
        &self,
        model_id: &ModelId,
        mood: Mood,
        dominant_emotion: Option<Emotion>,
    ) -> CognitionResult<()> {
        self.store.update_model(model_id, |file| {
            file.model.consciousness.emotional_weather.current_mood = mood;
            file.model.consciousness.emotional_weather.dominant_emotion = dominant_emotion;
            file.model.consciousness.updated_at = Timestamp::now();
            file.model.updated_at = Timestamp::now();
        })
    }

    /// Update life phase
    pub fn update_life_phase(&self, model_id: &ModelId, phase: LifePhase) -> CognitionResult<()> {
        self.store.update_model(model_id, |file| {
            file.model.consciousness.life_phase = phase;
            file.model.consciousness.updated_at = Timestamp::now();
            file.model.updated_at = Timestamp::now();
        })
    }
}
