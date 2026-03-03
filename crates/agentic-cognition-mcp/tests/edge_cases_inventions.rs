//! Edge case tests for the 24 inventions via MCP tools

use agentic_cognition::*;
use tempfile::TempDir;

fn setup() -> (WriteEngine, ModelId, TempDir) {
    let dir = TempDir::new().unwrap();
    let store = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();
    (write, model_id, dir)
}

fn query_for(dir: &TempDir) -> QueryEngine {
    let store = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    QueryEngine::new(store)
}

#[test]
fn test_empty_model_soul_reflection() {
    let (_w, mid, d) = setup();
    let q = query_for(&d);
    let reflection = q.soul_reflection(&mid).unwrap();
    assert!(reflection.confidence >= 0.0);
}

#[test]
fn test_empty_model_prediction() {
    let (_w, mid, d) = setup();
    let q = query_for(&d);
    let pred = q.predict_preference(&mid, "anything").unwrap();
    assert!(pred.predicted_preference >= 0.0);
    assert!(pred.predicted_preference <= 1.0);
}

#[test]
fn test_empty_model_simulation() {
    let (_w, mid, d) = setup();
    let q = query_for(&d);
    let sim = q
        .simulate_decision(&mid, "test", &["A".into(), "B".into()])
        .unwrap();
    assert_eq!(sim.options.len(), 2);
}

#[test]
fn test_empty_model_future_projection() {
    let (_w, mid, d) = setup();
    let q = query_for(&d);
    let proj = q.project_future(&mid, 365).unwrap();
    assert!(proj.confidence >= 0.0);
}

#[test]
fn test_belief_at_boundaries() {
    let (w, mid, d) = setup();
    // Confidence at exact boundaries
    let _ = w
        .add_belief(&mid, "Zero confidence".into(), BeliefDomain::Other, 0.0)
        .unwrap();
    let _ = w
        .add_belief(&mid, "Full confidence".into(), BeliefDomain::Other, 1.0)
        .unwrap();
    let q = query_for(&d);
    let beliefs = q.list_beliefs(&mid).unwrap();
    assert_eq!(beliefs.len(), 2);
}

#[test]
fn test_belief_invalid_confidence() {
    let (w, mid, _d) = setup();
    assert!(w
        .add_belief(&mid, "Too high".into(), BeliefDomain::Other, 1.1)
        .is_err());
    assert!(w
        .add_belief(&mid, "Negative".into(), BeliefDomain::Other, -0.1)
        .is_err());
}

#[test]
fn test_strengthen_beyond_one() {
    let (w, mid, _d) = setup();
    let bid = w
        .add_belief(&mid, "test".into(), BeliefDomain::Values, 0.9)
        .unwrap();
    w.strengthen_belief(&mid, &bid, 0.5).unwrap();
    let file = w.store().get_model(&mid).unwrap();
    let b = file.belief_graph.get_belief(&bid).unwrap();
    assert!(b.confidence <= 1.0); // must cap at 1.0
}

#[test]
fn test_weaken_below_zero() {
    let (w, mid, _d) = setup();
    let bid = w
        .add_belief(&mid, "test".into(), BeliefDomain::Values, 0.1)
        .unwrap();
    w.weaken_belief(&mid, &bid, 0.5).unwrap();
    let file = w.store().get_model(&mid).unwrap();
    let b = file.belief_graph.get_belief(&bid).unwrap();
    assert!(b.confidence >= 0.0); // must floor at 0.0
}

#[test]
fn test_crystallize_then_weaken() {
    let (w, mid, _d) = setup();
    let bid = w
        .add_belief(&mid, "test".into(), BeliefDomain::Values, 0.9)
        .unwrap();
    w.crystallize_belief(&mid, &bid).unwrap();
    w.weaken_belief(&mid, &bid, 0.1).unwrap();
    let file = w.store().get_model(&mid).unwrap();
    let b = file.belief_graph.get_belief(&bid).unwrap();
    assert_eq!(b.state, BeliefState::Challenged); // crystallized -> challenged on weaken
}

#[test]
fn test_collapse_cascade() {
    let (w, mid, _d) = setup();
    let keystone = w
        .add_belief(&mid, "Foundation".into(), BeliefDomain::Values, 0.9)
        .unwrap();
    let dependent = w
        .add_belief(
            &mid,
            "Built on foundation".into(),
            BeliefDomain::Values,
            0.8,
        )
        .unwrap();
    w.connect_beliefs(&mid, dependent, keystone, ConnectionType::Requires, 0.9)
        .unwrap();

    w.collapse_belief(
        &mid,
        &keystone,
        agentic_cognition::types::drift::CollapseTrigger::DeliberateInvestigation,
    )
    .unwrap();

    let file = w.store().get_model(&mid).unwrap();
    let dep = file.belief_graph.get_belief(&dependent).unwrap();
    assert_eq!(dep.state, BeliefState::Challenged); // cascade
}

#[test]
fn test_large_model_performance() {
    let (w, mid, d) = setup();
    // Add 100 beliefs
    for i in 0..100 {
        w.add_belief(
            &mid,
            format!("Belief {i}"),
            BeliefDomain::Values,
            0.5 + (i as f64 * 0.004),
        )
        .unwrap();
    }
    let q = query_for(&d);
    let beliefs = q.list_beliefs(&mid).unwrap();
    assert_eq!(beliefs.len(), 100);

    // Soul reflection on large model
    let reflection = q.soul_reflection(&mid).unwrap();
    assert!(reflection.confidence > 0.0);
}

#[test]
fn test_all_domains_accepted() {
    let (w, mid, _d) = setup();
    let domains = [
        BeliefDomain::Self_,
        BeliefDomain::Relationships,
        BeliefDomain::Work,
        BeliefDomain::Politics,
        BeliefDomain::Religion,
        BeliefDomain::Science,
        BeliefDomain::Values,
        BeliefDomain::WorldModel,
        BeliefDomain::Identity,
        BeliefDomain::Capability,
        BeliefDomain::Worth,
        BeliefDomain::Other,
    ];
    for domain in domains {
        w.add_belief(&mid, format!("{domain:?} belief"), domain, 0.5)
            .unwrap();
    }
    let file = w.store().get_model(&mid).unwrap();
    assert_eq!(file.belief_graph.beliefs.len(), 12);
}

#[test]
fn test_self_connection_rejected() {
    let (w, mid, _d) = setup();
    let bid = w
        .add_belief(&mid, "test".into(), BeliefDomain::Values, 0.5)
        .unwrap();
    assert!(w
        .connect_beliefs(&mid, bid, bid, ConnectionType::Supports, 0.5)
        .is_err());
}

#[test]
fn test_nonexistent_model() {
    let dir = TempDir::new().unwrap();
    let store = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query = QueryEngine::new(store);
    let fake_id = ModelId::new();
    assert!(query.get_model(&fake_id).is_err());
}
