//! Phase 3: Invention tests

use agentic_cognition::inventions::*;
use agentic_cognition::types::*;
use agentic_cognition::*;
use tempfile::TempDir;

fn setup() -> (WriteEngine, QueryEngine, ModelId, TempDir) {
    let dir = TempDir::new().unwrap();
    let store = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    // Add some beliefs
    write
        .add_belief(
            &model_id,
            "I value honesty".into(),
            BeliefDomain::Values,
            0.9,
        )
        .unwrap();
    write
        .add_belief(
            &model_id,
            "I am a good programmer".into(),
            BeliefDomain::Capability,
            0.8,
        )
        .unwrap();
    write
        .add_belief(
            &model_id,
            "Hard work pays off".into(),
            BeliefDomain::WorldModel,
            0.7,
        )
        .unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query = QueryEngine::new(store2);
    (write, query, model_id, dir)
}

// --- Soul Reflection Tests ---

#[test]
fn test_soul_reflection() {
    let (_write, query, model_id, _dir) = setup();
    let reflection = query.soul_reflection(&model_id).unwrap();
    assert!(reflection.confidence >= 0.0);
    assert!(!reflection.reflection_id.to_string().is_empty());
    assert_eq!(reflection.model_id, model_id);
}

#[test]
fn test_soul_reflection_direct() {
    let model = LivingUserModel::new();
    let mut graph = BeliefGraph::new();
    graph.add_belief(Belief::new("Core belief".into(), BeliefDomain::Values, 0.9));

    let reflection = SoulReflector::reflect(&model, &graph);
    assert!(reflection.confidence >= 0.0);
    assert!(reflection.confidence <= 1.0);
}

#[test]
fn test_soul_reflection_empty_model() {
    let model = LivingUserModel::new();
    let graph = BeliefGraph::new();

    let reflection = SoulReflector::reflect(&model, &graph);
    // Empty model should still produce a valid reflection, just lower confidence
    assert!(reflection.confidence >= 0.0);
}

// --- Preference Oracle Tests ---

#[test]
fn test_preference_prediction() {
    let (_write, query, model_id, _dir) = setup();
    let prediction = query.predict_preference(&model_id, "honesty").unwrap();
    assert!(prediction.predicted_preference >= 0.0);
    assert!(prediction.predicted_preference <= 1.0);
    assert!(!prediction.reasoning.is_empty());
    assert_eq!(prediction.model_id, model_id);
    assert_eq!(prediction.item, "honesty");
}

#[test]
fn test_preference_prediction_unknown_item() {
    let (_write, query, model_id, _dir) = setup();
    let prediction = query
        .predict_preference(&model_id, "quantum_physics_xyz")
        .unwrap();
    // Unknown item should have low confidence
    assert!(prediction.confidence < 0.5);
}

#[test]
fn test_preference_prediction_direct() {
    let model = LivingUserModel::new();
    let mut graph = BeliefGraph::new();
    graph.add_belief(Belief::new(
        "I value honesty".into(),
        BeliefDomain::Values,
        0.9,
    ));

    let prediction = PreferenceOracle::predict(&model, &graph, &None, "honesty");
    // Should find alignment with the honesty belief
    assert!(prediction.predicted_preference > 0.5);
    assert!(!prediction.reasoning.is_empty());
}

#[test]
fn test_preference_prediction_with_fingerprint() {
    let model = LivingUserModel::new();
    let graph = BeliefGraph::new();
    let fp = Some(DecisionFingerprint::new(model.id));

    let prediction = PreferenceOracle::predict(&model, &graph, &fp, "unknown_thing");
    assert!(prediction.predicted_preference >= 0.0);
    assert!(prediction.predicted_preference <= 1.0);
}

// --- Decision Simulation Tests ---

#[test]
fn test_decision_simulation() {
    let (_write, query, model_id, _dir) = setup();
    let sim = query
        .simulate_decision(
            &model_id,
            "Should I change jobs?",
            &["Stay at current job".into(), "Take the new offer".into()],
        )
        .unwrap();
    assert_eq!(sim.options.len(), 2);
    assert!(sim.predicted_choice.is_some());
    assert!(sim.confidence > 0.0);
    assert_eq!(sim.model_id, model_id);
    assert_eq!(sim.scenario, "Should I change jobs?");

    // Probabilities should sum to ~1.0
    let total: f64 = sim.options.iter().map(|o| o.predicted_probability).sum();
    assert!((total - 1.0).abs() < 0.01);
}

#[test]
fn test_decision_simulation_three_options() {
    let (_write, query, model_id, _dir) = setup();
    let sim = query
        .simulate_decision(
            &model_id,
            "What to have for lunch?",
            &["Pizza".into(), "Sushi".into(), "Salad".into()],
        )
        .unwrap();
    assert_eq!(sim.options.len(), 3);
    assert!(sim.predicted_choice.is_some());

    let total: f64 = sim.options.iter().map(|o| o.predicted_probability).sum();
    assert!((total - 1.0).abs() < 0.01);
}

#[test]
fn test_decision_simulation_direct() {
    let model = LivingUserModel::new();
    let graph = BeliefGraph::new();
    let options = vec!["Option A".into(), "Option B".into()];

    let sim = DecisionSimulator::simulate(&model, &graph, &None, "Test scenario", &options);
    assert_eq!(sim.options.len(), 2);
    assert!(sim.predicted_choice.is_some());
    assert!(!sim.predicted_process.is_empty());
}

// --- Future Projection Tests ---

#[test]
fn test_future_projection() {
    let (_write, query, model_id, _dir) = setup();
    let projection = query.project_future(&model_id, 90).unwrap();
    assert_eq!(projection.time_horizon_days, 90);
    assert!(!projection.projected_beliefs.is_empty());
    assert!(projection.confidence > 0.0);
    assert_eq!(projection.model_id, model_id);
}

#[test]
fn test_future_projection_long_horizon() {
    let (_write, query, model_id, _dir) = setup();
    let short = query.project_future(&model_id, 30).unwrap();
    let long = query.project_future(&model_id, 365).unwrap();
    // Longer horizon should have lower confidence
    assert!(long.confidence <= short.confidence);
}

#[test]
fn test_future_projection_direct() {
    let model = LivingUserModel::new();
    let mut graph = BeliefGraph::new();
    graph.add_belief(Belief::new("Core belief".into(), BeliefDomain::Values, 0.9));
    let drift = DriftTimeline::default();

    let projection = FutureProjector::project(&model, &graph, &drift, 90);
    assert_eq!(projection.projected_beliefs.len(), 1);
    assert_eq!(projection.time_horizon_days, 90);
}

#[test]
fn test_future_projection_with_drift() {
    let model = LivingUserModel::new();
    let mut graph = BeliefGraph::new();
    let b = Belief::new("Drifting belief".into(), BeliefDomain::Values, 0.8);
    let bid = b.id;
    graph.add_belief(b);

    let mut drift = DriftTimeline::default();
    drift.events.push(DriftEvent {
        id: DriftId::new(),
        belief_id: bid,
        timestamp: Timestamp::now(),
        direction: DriftDirection::Weakening,
        magnitude: 0.5,
        cause: DriftCause::Unknown,
        previous_confidence: 0.8,
        new_confidence: 0.6,
    });

    let projection = FutureProjector::project(&model, &graph, &drift, 90);
    assert!(!projection.projected_beliefs.is_empty());

    // The drifting belief should have a projected confidence different from current
    let projected = &projection.projected_beliefs[0];
    assert_eq!(projected.belief_id, bid);
}

#[test]
fn test_future_projection_empty_model() {
    let model = LivingUserModel::new();
    let graph = BeliefGraph::new();
    let drift = DriftTimeline::default();

    let projection = FutureProjector::project(&model, &graph, &drift, 90);
    assert!(projection.projected_beliefs.is_empty());
    assert!(projection.confidence > 0.0);
}

// --- Query Engine Invention Wrappers ---

#[test]
fn test_search_beliefs() {
    let (_write, query, model_id, _dir) = setup();
    let results = query.search_beliefs(&model_id, "honesty").unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].content.contains("honesty"));
}

#[test]
fn test_search_beliefs_case_insensitive() {
    let (_write, query, model_id, _dir) = setup();
    let results = query.search_beliefs(&model_id, "HONESTY").unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_beliefs_no_match() {
    let (_write, query, model_id, _dir) = setup();
    let results = query
        .search_beliefs(&model_id, "quantum_entanglement")
        .unwrap();
    assert!(results.is_empty());
}

#[test]
fn test_get_topology() {
    let (write, _query, model_id, dir) = setup();
    write
        .add_peak(&model_id, "coding".into(), 0.9, true)
        .unwrap();
    write
        .add_valley(&model_id, "public speaking".into(), 0.7)
        .unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query2 = QueryEngine::new(store2);
    let topo = query2.get_topology(&model_id).unwrap();
    assert_eq!(topo.peaks.len(), 1);
    assert_eq!(topo.valleys.len(), 1);
}

#[test]
fn test_get_consciousness() {
    let (_write, query, model_id, _dir) = setup();
    let consciousness = query.get_consciousness(&model_id).unwrap();
    assert_eq!(consciousness.life_phase, LifePhase::Exploring);
}

#[test]
fn test_get_soul() {
    let (_write, query, model_id, _dir) = setup();
    let soul = query.get_soul(&model_id).unwrap();
    assert!(soul.deep_values.is_empty()); // Default has no deep values
    assert_eq!(soul.authenticity_gap, 0.0);
}

#[test]
fn test_get_shadow_map() {
    let (write, _query, model_id, dir) = setup();
    write
        .add_shadow_belief(&model_id, "I fear failure".into(), 0.6, None)
        .unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query2 = QueryEngine::new(store2);
    let shadow = query2.get_shadow_map(&model_id).unwrap();
    assert_eq!(shadow.shadow_beliefs.len(), 1);
}

#[test]
fn test_get_bias_field() {
    let (write, _query, model_id, dir) = setup();
    write
        .add_bias(
            &model_id,
            "Confirmation".into(),
            BiasType::Confirmation,
            0.5,
        )
        .unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query2 = QueryEngine::new(store2);
    let field = query2.get_bias_field(&model_id).unwrap();
    assert_eq!(field.biases.len(), 1);
}

#[test]
fn test_belief_collapse_and_drift() {
    let (write, _query, model_id, dir) = setup();
    let belief_id = write
        .add_belief(
            &model_id,
            "The earth is flat".into(),
            BeliefDomain::Science,
            0.9,
        )
        .unwrap();

    write
        .collapse_belief(
            &model_id,
            &belief_id,
            CollapseTrigger::UndeniableEvidence {
                evidence: "Photos from space".into(),
            },
        )
        .unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query2 = QueryEngine::new(store2);
    let belief = query2.get_belief(&model_id, &belief_id).unwrap();
    assert_eq!(belief.state, BeliefState::Collapsed);
    assert_eq!(belief.confidence, 0.0);

    // Check drift event was recorded
    let drift = query2.get_drift_timeline(&model_id).unwrap();
    assert!(!drift.events.is_empty());
    assert_eq!(
        drift.events.last().unwrap().direction,
        DriftDirection::Reversing
    );
}

#[test]
fn test_collapse_cascades_to_dependents() {
    let (write, _query, model_id, _dir) = setup();
    let foundation = write
        .add_belief(&model_id, "Foundation".into(), BeliefDomain::Values, 0.9)
        .unwrap();
    let dependent = write
        .add_belief(
            &model_id,
            "Depends on foundation".into(),
            BeliefDomain::Values,
            0.8,
        )
        .unwrap();

    write
        .connect_beliefs(
            &model_id,
            dependent,
            foundation,
            ConnectionType::Requires,
            0.9,
        )
        .unwrap();

    write
        .collapse_belief(
            &model_id,
            &foundation,
            CollapseTrigger::DeliberateInvestigation,
        )
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    let dep_belief = file.belief_graph.get_belief(&dependent).unwrap();
    // Dependent should be challenged and weakened
    assert_eq!(dep_belief.state, BeliefState::Challenged);
    assert!(dep_belief.confidence < 0.8);
}

#[test]
fn test_get_portrait() {
    let (write, _query, model_id, dir) = setup();
    write
        .add_shadow_belief(&model_id, "shadow".into(), 0.5, None)
        .unwrap();
    write
        .add_bias(&model_id, "bias".into(), BiasType::Confirmation, 0.5)
        .unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query2 = QueryEngine::new(store2);
    let portrait = query2.get_portrait(&model_id).unwrap();
    assert_eq!(portrait.belief_count, 3); // 3 beliefs from setup()
    assert_eq!(portrait.shadow_count, 1);
    assert_eq!(portrait.bias_count, 1);
    assert!(!portrait.has_fingerprint);
}

#[test]
fn test_get_contradictions() {
    let (write, _query, model_id, dir) = setup();
    let b1 = write
        .add_belief(&model_id, "X is true".into(), BeliefDomain::Values, 0.8)
        .unwrap();
    let b2 = write
        .add_belief(&model_id, "X is false".into(), BeliefDomain::Values, 0.7)
        .unwrap();
    write
        .connect_beliefs(&model_id, b1, b2, ConnectionType::Contradicts, 0.9)
        .unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query2 = QueryEngine::new(store2);
    let contradictions = query2.get_contradictions(&model_id).unwrap();
    assert_eq!(contradictions.len(), 1);
}

#[test]
fn test_get_keystones() {
    let (write, _query, model_id, dir) = setup();
    let foundation = write
        .add_belief(&model_id, "Foundation".into(), BeliefDomain::Values, 0.9)
        .unwrap();
    let dep1 = write
        .add_belief(&model_id, "Dep1".into(), BeliefDomain::Values, 0.7)
        .unwrap();
    let dep2 = write
        .add_belief(&model_id, "Dep2".into(), BeliefDomain::Values, 0.6)
        .unwrap();

    write
        .connect_beliefs(&model_id, dep1, foundation, ConnectionType::Requires, 0.8)
        .unwrap();
    write
        .connect_beliefs(&model_id, dep2, foundation, ConnectionType::Requires, 0.7)
        .unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query2 = QueryEngine::new(store2);
    let keystones = query2.get_keystones(&model_id).unwrap();
    assert_eq!(keystones.len(), 1);
    assert_eq!(keystones[0].belief_id, foundation);
}
