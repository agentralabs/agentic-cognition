//! Phase 6: Integration tests

use agentic_cognition::types::*;
use agentic_cognition::*;
use tempfile::TempDir;

fn setup_full() -> (WriteEngine, TempDir) {
    let dir = TempDir::new().unwrap();
    let store = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let write = WriteEngine::new(store);
    (write, dir)
}

#[test]
fn test_full_model_lifecycle() {
    let (write, dir) = setup_full();

    // Create model
    let model_id = write.create_model().unwrap();

    // Add beliefs
    let b1 = write
        .add_belief(
            &model_id,
            "Honesty matters".into(),
            BeliefDomain::Values,
            0.9,
        )
        .unwrap();
    let _b2 = write
        .add_belief(&model_id, "I am creative".into(), BeliefDomain::Self_, 0.8)
        .unwrap();
    let b3 = write
        .add_belief(
            &model_id,
            "Success requires effort".into(),
            BeliefDomain::WorldModel,
            0.7,
        )
        .unwrap();

    // Connect beliefs
    write
        .connect_beliefs(&model_id, b1, b3, ConnectionType::Supports, 0.8)
        .unwrap();

    // Add self-concept
    write
        .add_peak(&model_id, "programming".into(), 0.9, true)
        .unwrap();
    write.add_valley(&model_id, "sales".into(), 0.6).unwrap();
    write
        .add_blindspot(&model_id, "emotional intelligence".into(), 0.7)
        .unwrap();

    // Add shadow
    write
        .add_shadow_belief(&model_id, "I fear failure".into(), 0.6, None)
        .unwrap();
    write
        .add_projection(&model_id, "laziness".into(), "colleague".into())
        .unwrap();

    // Add bias
    write
        .add_bias(
            &model_id,
            "Confirmation bias".into(),
            BiasType::Confirmation,
            0.5,
        )
        .unwrap();
    write
        .add_trigger(&model_id, "criticism".into(), "defensive".into(), 0.7)
        .unwrap();

    // Heartbeat
    write
        .heartbeat(&model_id, vec!["User discussed career goals".into()])
        .unwrap();

    // Query everything
    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query = QueryEngine::new(store2);

    let portrait = query.get_portrait(&model_id).unwrap();
    assert_eq!(portrait.belief_count, 3);
    assert_eq!(portrait.shadow_count, 1);
    assert_eq!(portrait.bias_count, 1);

    // Soul reflection
    let reflection = query.soul_reflection(&model_id).unwrap();
    assert!(reflection.confidence >= 0.0);

    // Preference prediction
    let pred = query.predict_preference(&model_id, "honesty").unwrap();
    assert!(pred.predicted_preference > 0.5);

    // Decision simulation
    let sim = query
        .simulate_decision(
            &model_id,
            "Should I pursue a promotion?",
            &["Yes, go for it".into(), "No, stay put".into()],
        )
        .unwrap();
    assert!(sim.predicted_choice.is_some());

    // Future projection
    let future = query.project_future(&model_id, 180).unwrap();
    assert!(!future.projected_beliefs.is_empty());

    // Topology
    let topo = query.get_topology(&model_id).unwrap();
    assert_eq!(topo.peaks.len(), 1);
    assert_eq!(topo.valleys.len(), 1);
    assert_eq!(topo.blind_canyons.len(), 1);

    // Shadow
    let shadow = query.get_shadow_map(&model_id).unwrap();
    assert_eq!(shadow.shadow_beliefs.len(), 1);
    assert_eq!(shadow.projections.len(), 1);

    // Bias
    let bias = query.get_bias_field(&model_id).unwrap();
    assert_eq!(bias.biases.len(), 1);
    assert_eq!(bias.triggers.len(), 1);

    // Consciousness
    let consciousness = query.get_consciousness(&model_id).unwrap();
    assert_eq!(consciousness.life_phase, LifePhase::Exploring);

    // Drift
    let drift = query.get_drift_timeline(&model_id).unwrap();
    // No weaken/collapse operations, so no drift events
    assert!(drift.events.is_empty());
}

#[test]
fn test_model_persistence_across_reloads() {
    let dir = TempDir::new().unwrap();
    let model_id;

    // Session 1: Create and populate
    {
        let store = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
        let write = WriteEngine::new(store);
        model_id = write.create_model().unwrap();
        write
            .add_belief(
                &model_id,
                "Persisted belief".into(),
                BeliefDomain::Values,
                0.9,
            )
            .unwrap();
        write
            .add_shadow_belief(&model_id, "Shadow persists".into(), 0.5, None)
            .unwrap();
        write
            .add_bias(
                &model_id,
                "Bias persists".into(),
                BiasType::Confirmation,
                0.4,
            )
            .unwrap();
    }

    // Session 2: Reload and verify
    {
        let store = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
        let query = QueryEngine::new(store);
        let models = query.list_models().unwrap();
        assert_eq!(models.len(), 1);
        assert_eq!(models[0], model_id);

        let beliefs = query.list_beliefs(&model_id).unwrap();
        assert_eq!(beliefs.len(), 1);
        assert_eq!(beliefs[0].content, "Persisted belief");

        let shadow = query.get_shadow_map(&model_id).unwrap();
        assert_eq!(shadow.shadow_beliefs.len(), 1);

        let bias = query.get_bias_field(&model_id).unwrap();
        assert_eq!(bias.biases.len(), 1);
    }
}

#[test]
fn test_multiple_models_isolation() {
    let (write, dir) = setup_full();

    let m1 = write.create_model().unwrap();
    let m2 = write.create_model().unwrap();

    write
        .add_belief(&m1, "Model 1 belief".into(), BeliefDomain::Values, 0.8)
        .unwrap();
    write
        .add_belief(&m2, "Model 2 belief".into(), BeliefDomain::Work, 0.7)
        .unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query = QueryEngine::new(store2);

    let b1 = query.list_beliefs(&m1).unwrap();
    let b2 = query.list_beliefs(&m2).unwrap();
    assert_eq!(b1.len(), 1);
    assert_eq!(b2.len(), 1);
    assert_ne!(b1[0].content, b2[0].content);
    assert_eq!(b1[0].content, "Model 1 belief");
    assert_eq!(b2[0].content, "Model 2 belief");
}

#[test]
fn test_belief_strengthen_and_crystallize_flow() {
    let (write, _dir) = setup_full();
    let model_id = write.create_model().unwrap();
    let bid = write
        .add_belief(&model_id, "Strong belief".into(), BeliefDomain::Values, 0.6)
        .unwrap();

    // Strengthen multiple times
    for _ in 0..5 {
        write.strengthen_belief(&model_id, &bid, 0.05).unwrap();
    }

    // Then crystallize
    write.crystallize_belief(&model_id, &bid).unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    let belief = file.belief_graph.get_belief(&bid).unwrap();
    assert_eq!(belief.state, BeliefState::Crystallized);
    assert!(belief.confidence > 0.8);
    assert_eq!(belief.crystallization, 1.0);
}

#[test]
fn test_entangle_beliefs() {
    let (write, _dir) = setup_full();
    let model_id = write.create_model().unwrap();
    let b1 = write
        .add_belief(&model_id, "A".into(), BeliefDomain::Values, 0.8)
        .unwrap();
    let b2 = write
        .add_belief(&model_id, "B".into(), BeliefDomain::Values, 0.7)
        .unwrap();

    let ent_id = write
        .entangle_beliefs(&model_id, vec![b1, b2], EntanglementType::Correlated, 0.9)
        .unwrap();

    assert!(!ent_id.to_string().is_empty());

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.belief_graph.entanglements.len(), 1);
    assert_eq!(file.belief_graph.entanglements[0].beliefs.len(), 2);
    assert_eq!(file.belief_graph.entanglements[0].strength, 0.9);
}

#[test]
fn test_emotional_weather_update() {
    let (write, _dir) = setup_full();
    let model_id = write.create_model().unwrap();

    write
        .update_emotional_weather(&model_id, Mood::Positive, Some(Emotion::Hope))
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(
        file.model.consciousness.emotional_weather.current_mood,
        Mood::Positive
    );
    assert_eq!(
        file.model.consciousness.emotional_weather.dominant_emotion,
        Some(Emotion::Hope)
    );
}

#[test]
fn test_emotional_weather_update_no_emotion() {
    let (write, _dir) = setup_full();
    let model_id = write.create_model().unwrap();

    write
        .update_emotional_weather(&model_id, Mood::Subdued, None)
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(
        file.model.consciousness.emotional_weather.current_mood,
        Mood::Subdued
    );
    assert!(file
        .model
        .consciousness
        .emotional_weather
        .dominant_emotion
        .is_none());
}

#[test]
fn test_life_phase_update() {
    let (write, _dir) = setup_full();
    let model_id = write.create_model().unwrap();

    write
        .update_life_phase(&model_id, LifePhase::Transitioning)
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(
        file.model.consciousness.life_phase,
        LifePhase::Transitioning
    );
}

#[test]
fn test_value_tectonic() {
    let (write, _dir) = setup_full();
    let model_id = write.create_model().unwrap();

    write
        .add_value_tectonic(
            &model_id,
            "career ambition".into(),
            "decreasing".into(),
            0.3,
        )
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.drift.value_tectonics.len(), 1);
    assert_eq!(file.drift.value_tectonics[0].value, "career ambition");
    assert_eq!(file.drift.value_tectonics[0].direction, "decreasing");
    assert_eq!(file.drift.value_tectonics[0].magnitude, 0.3);
}

#[test]
fn test_metamorphosis() {
    let (write, _dir) = setup_full();
    let model_id = write.create_model().unwrap();

    write
        .add_metamorphosis(
            &model_id,
            "Career transformation".into(),
            "Burnout".into(),
            "Corporate employee".into(),
            "Freelancer".into(),
        )
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.drift.metamorphoses.len(), 1);
    assert_eq!(
        file.drift.metamorphoses[0].description,
        "Career transformation"
    );
    // Metamorphosis triggers crisis lifecycle
    assert_eq!(file.model.lifecycle_stage, ModelLifecycleStage::Crisis);
}

#[test]
fn test_collapse_and_rebuild_cycle() {
    let (write, dir) = setup_full();
    let model_id = write.create_model().unwrap();

    // Build a belief
    let bid = write
        .add_belief(
            &model_id,
            "I am always right".into(),
            BeliefDomain::Self_,
            0.9,
        )
        .unwrap();
    write.crystallize_belief(&model_id, &bid).unwrap();

    // Collapse it
    write
        .collapse_belief(
            &model_id,
            &bid,
            CollapseTrigger::UndeniableEvidence {
                evidence: "I was clearly wrong".into(),
            },
        )
        .unwrap();

    // Verify collapsed
    let file = write.store().get_model(&model_id).unwrap();
    let belief = file.belief_graph.get_belief(&bid).unwrap();
    assert_eq!(belief.state, BeliefState::Collapsed);
    assert_eq!(belief.confidence, 0.0);

    // Add a replacement belief
    let new_bid = write
        .add_belief(
            &model_id,
            "I can be wrong and that is okay".into(),
            BeliefDomain::Self_,
            0.7,
        )
        .unwrap();

    // Verify the model has both beliefs
    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query = QueryEngine::new(store2);
    let beliefs = query.list_beliefs(&model_id).unwrap();
    assert_eq!(beliefs.len(), 2);

    // The new belief should be healthy
    let new_belief = query.get_belief(&model_id, &new_bid).unwrap();
    assert_eq!(new_belief.state, BeliefState::Forming);
    assert_eq!(new_belief.confidence, 0.7);
}

#[test]
fn test_complex_belief_network() {
    let (write, dir) = setup_full();
    let model_id = write.create_model().unwrap();

    // Create a network of beliefs
    let core = write
        .add_belief(
            &model_id,
            "Hard work is important".into(),
            BeliefDomain::Values,
            0.9,
        )
        .unwrap();
    let b1 = write
        .add_belief(
            &model_id,
            "I should work overtime".into(),
            BeliefDomain::Work,
            0.7,
        )
        .unwrap();
    let b2 = write
        .add_belief(&model_id, "Rest is lazy".into(), BeliefDomain::Self_, 0.5)
        .unwrap();
    let b3 = write
        .add_belief(
            &model_id,
            "Work-life balance matters".into(),
            BeliefDomain::Values,
            0.8,
        )
        .unwrap();

    // b1 supports core, b2 implies b1
    write
        .connect_beliefs(&model_id, b1, core, ConnectionType::Supports, 0.8)
        .unwrap();
    write
        .connect_beliefs(&model_id, b2, b1, ConnectionType::Implies, 0.6)
        .unwrap();

    // b3 contradicts b2
    write
        .connect_beliefs(&model_id, b3, b2, ConnectionType::Contradicts, 0.7)
        .unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query = QueryEngine::new(store2);

    // Should find the contradiction
    let contradictions = query.get_contradictions(&model_id).unwrap();
    assert_eq!(contradictions.len(), 1);

    // Full graph should have 4 beliefs and 3 connections
    let graph = query.get_belief_graph(&model_id).unwrap();
    assert_eq!(graph.beliefs.len(), 4);
    assert_eq!(graph.connections.len(), 3);
}

#[test]
fn test_multiple_heartbeats_update_evidence_count() {
    let (write, _dir) = setup_full();
    let model_id = write.create_model().unwrap();

    write
        .heartbeat(&model_id, vec!["obs1".into(), "obs2".into()])
        .unwrap();
    write.heartbeat(&model_id, vec!["obs3".into()]).unwrap();
    write
        .heartbeat(&model_id, vec!["obs4".into(), "obs5".into(), "obs6".into()])
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.model.evidence_count, 6);
    assert_eq!(file.model.vitals.evidence_count, 6);
    // 6 observations should trigger Birth -> Infancy
    assert_eq!(file.model.lifecycle_stage, ModelLifecycleStage::Infancy);
}

#[test]
fn test_weaken_crystallized_becomes_challenged() {
    let (write, _dir) = setup_full();
    let model_id = write.create_model().unwrap();
    let bid = write
        .add_belief(&model_id, "Firm belief".into(), BeliefDomain::Values, 0.9)
        .unwrap();

    write.crystallize_belief(&model_id, &bid).unwrap();
    write.weaken_belief(&model_id, &bid, 0.1).unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    let belief = file.belief_graph.get_belief(&bid).unwrap();
    assert_eq!(belief.state, BeliefState::Challenged);
}

#[test]
fn test_full_shadow_integration() {
    let (write, dir) = setup_full();
    let model_id = write.create_model().unwrap();

    // Conscious belief
    let conscious = write
        .add_belief(&model_id, "I am confident".into(), BeliefDomain::Self_, 0.8)
        .unwrap();

    // Shadow belief that contradicts it
    write
        .add_shadow_belief(
            &model_id,
            "I doubt myself deeply".into(),
            0.7,
            Some(conscious),
        )
        .unwrap();

    // Projection
    write
        .add_projection(&model_id, "insecurity".into(), "friend".into())
        .unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query = QueryEngine::new(store2);
    let shadow = query.get_shadow_map(&model_id).unwrap();

    assert_eq!(shadow.shadow_beliefs.len(), 1);
    assert_eq!(
        shadow.shadow_beliefs[0].contradicts_conscious,
        Some(conscious)
    );
    assert_eq!(shadow.projections.len(), 1);
    assert_eq!(shadow.projections[0].disowned_trait, "insecurity");
}

#[test]
fn test_decision_fingerprint_update() {
    let (write, dir) = setup_full();
    let model_id = write.create_model().unwrap();

    let traits = DecisionTraits {
        information_appetite: 0.7,
        risk_tolerance: -0.3,
        speed_accuracy_tradeoff: 0.5,
        intuition_analysis_balance: 0.6,
        social_influence: 0.2,
        time_horizon: 0.8,
        emotional_regulation: 0.4,
        reversibility_seeking: 0.3,
    };

    write.update_fingerprint(&model_id, traits).unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query = QueryEngine::new(store2);
    let fp = query.get_fingerprint(&model_id).unwrap();
    assert!(fp.is_some());

    let fp = fp.unwrap();
    assert_eq!(fp.traits.information_appetite, 0.7);
    assert_eq!(fp.traits.risk_tolerance, -0.3);
    assert!(fp.confidence > 0.0);
}

#[test]
fn test_delete_model_removes_file() {
    let (write, dir) = setup_full();
    let model_id = write.create_model().unwrap();

    // File should exist
    let path = dir.path().join(format!("{}.acog", model_id));
    assert!(path.exists());

    // Delete model
    write.store().delete_model(&model_id).unwrap();

    // File should be gone
    assert!(!path.exists());

    // Model should not be retrievable
    let result = write.store().get_model(&model_id);
    assert!(result.is_err());
}

#[test]
fn test_store_list_after_operations() {
    let (write, _dir) = setup_full();

    let m1 = write.create_model().unwrap();
    let m2 = write.create_model().unwrap();
    let m3 = write.create_model().unwrap();

    assert_eq!(write.store().list_models().unwrap().len(), 3);

    write.store().delete_model(&m2).unwrap();
    assert_eq!(write.store().list_models().unwrap().len(), 2);

    let remaining = write.store().list_models().unwrap();
    assert!(remaining.contains(&m1));
    assert!(!remaining.contains(&m2));
    assert!(remaining.contains(&m3));
}

#[test]
fn test_index_manager_rebuild() {
    let (write, _dir) = setup_full();
    let model_id = write.create_model().unwrap();

    write
        .add_belief(
            &model_id,
            "High confidence".into(),
            BeliefDomain::Values,
            0.9,
        )
        .unwrap();
    write
        .add_belief(&model_id, "Low confidence".into(), BeliefDomain::Work, 0.3)
        .unwrap();

    let idx = write.store().get_index(&model_id).unwrap();
    assert_eq!(idx.get_by_domain(&BeliefDomain::Values).len(), 1);
    assert_eq!(idx.get_by_domain(&BeliefDomain::Work).len(), 1);
    assert_eq!(idx.get_high_confidence().len(), 1);
}
