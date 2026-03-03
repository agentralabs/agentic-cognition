//! Stress and edge-case tests for AgenticCognition
//!
//! These tests cover boundary conditions, heavy loads, and error paths
//! to validate robustness claims in the research paper.

use agentic_cognition::types::*;
use agentic_cognition::*;
use tempfile::TempDir;

fn setup() -> (WriteEngine, TempDir) {
    let dir = TempDir::new().unwrap();
    let store = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let write = WriteEngine::new(store);
    (write, dir)
}

fn query_for(dir: &TempDir) -> QueryEngine {
    let store = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    QueryEngine::new(store)
}

// ── Stress tests: heavy load scenarios ─────────────────────────────────────

#[test]
fn stress_create_many_models() {
    let (write, _dir) = setup();
    for _ in 0..100 {
        let id = write.create_model().unwrap();
        assert!(!id.to_string().is_empty());
    }
}

#[test]
fn stress_add_1000_beliefs_single_model() {
    let (write, dir) = setup();
    let model_id = write.create_model().unwrap();
    for i in 0..1000 {
        write
            .add_belief(
                &model_id,
                format!("Belief number {} about heavy load testing", i),
                BeliefDomain::Values,
                (i as f64 % 100.0) / 100.0,
            )
            .unwrap();
    }
    let query = query_for(&dir);
    let beliefs = query.list_beliefs(&model_id).unwrap();
    assert_eq!(beliefs.len(), 1000);
}

#[test]
fn stress_rapid_heartbeats() {
    let (write, _dir) = setup();
    let model_id = write.create_model().unwrap();
    for i in 0..500 {
        write
            .heartbeat(
                &model_id,
                vec![format!("Observation {} under heavy heartbeat stress", i)],
            )
            .unwrap();
    }
}

#[test]
fn stress_belief_graph_dense_connections() {
    let (write, dir) = setup();
    let model_id = write.create_model().unwrap();
    let mut ids = Vec::new();
    for i in 0..50 {
        let id = write
            .add_belief(
                &model_id,
                format!("Dense graph belief {}", i),
                BeliefDomain::Values,
                0.7,
            )
            .unwrap();
        ids.push(id);
    }
    // Connect pairs to create a dense graph
    for i in 0..ids.len() {
        for j in (i + 1)..ids.len().min(i + 5) {
            let _ = write.connect_beliefs(&model_id, ids[i], ids[j], ConnectionType::Supports, 0.5);
        }
    }
    let query = query_for(&dir);
    let graph = query.get_belief_graph(&model_id).unwrap();
    assert!(graph.beliefs.len() >= 50);
}

// ── Edge case tests: boundary conditions ───────────────────────────────────

#[test]
fn edge_empty_model_soul_reflection() {
    let (write, dir) = setup();
    let model_id = write.create_model().unwrap();
    // Soul reflection on empty model should not panic
    let query = query_for(&dir);
    let result = query.soul_reflection(&model_id);
    assert!(result.is_ok());
}

#[test]
fn edge_empty_model_shadow_map() {
    let (write, dir) = setup();
    let model_id = write.create_model().unwrap();
    let query = query_for(&dir);
    let shadow = query.get_shadow_map(&model_id).unwrap();
    assert_eq!(shadow.shadow_beliefs.len(), 0);
}

#[test]
fn edge_zero_confidence_belief() {
    let (write, dir) = setup();
    let model_id = write.create_model().unwrap();
    let id = write
        .add_belief(
            &model_id,
            "Zero confidence boundary belief".into(),
            BeliefDomain::Values,
            0.0,
        )
        .unwrap();
    let query = query_for(&dir);
    let beliefs = query.list_beliefs(&model_id).unwrap();
    let found = beliefs.iter().find(|b| b.id == id).unwrap();
    assert_eq!(found.confidence, 0.0);
}

#[test]
fn edge_max_confidence_belief() {
    let (write, dir) = setup();
    let model_id = write.create_model().unwrap();
    let id = write
        .add_belief(
            &model_id,
            "Maximum confidence boundary belief".into(),
            BeliefDomain::Values,
            1.0,
        )
        .unwrap();
    let query = query_for(&dir);
    let beliefs = query.list_beliefs(&model_id).unwrap();
    let found = beliefs.iter().find(|b| b.id == id).unwrap();
    assert_eq!(found.confidence, 1.0);
}

#[test]
fn edge_empty_belief_content() {
    let (write, _dir) = setup();
    let model_id = write.create_model().unwrap();
    // Empty string belief should be handled gracefully
    let result = write.add_belief(&model_id, "".into(), BeliefDomain::Values, 0.5);
    // Either succeeds or returns a validation error, must not panic
    let _ = result;
}

#[test]
fn edge_very_long_belief_content() {
    let (write, dir) = setup();
    let model_id = write.create_model().unwrap();
    let long_content = "A".repeat(10_000);
    let result = write.add_belief(&model_id, long_content.clone(), BeliefDomain::Values, 0.5);
    // Should handle long content without panicking
    if let Ok(id) = result {
        let query = query_for(&dir);
        let beliefs = query.list_beliefs(&model_id).unwrap();
        let found = beliefs.iter().find(|b| b.id == id);
        assert!(found.is_some());
    }
}

// ── Boundary tests: persistence edge cases ─────────────────────────────────

#[test]
fn boundary_save_and_reload_empty_model() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("boundary_empty.acog");
    let model = LivingUserModel::new();
    let file = agentic_cognition::format::AcogFile::new(model);
    file.save(&path).unwrap();
    let loaded = agentic_cognition::format::AcogFile::load(&path).unwrap();
    assert_eq!(loaded.belief_graph.beliefs.len(), 0);
}

#[test]
fn boundary_save_and_reload_large_model() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("boundary_large.acog");
    let model = LivingUserModel::new();
    let mut file = agentic_cognition::format::AcogFile::new(model);
    for i in 0..500 {
        file.belief_graph.add_belief(Belief::new(
            format!("Large boundary test belief {}", i),
            BeliefDomain::Values,
            0.7,
        ));
    }
    file.save(&path).unwrap();
    let loaded = agentic_cognition::format::AcogFile::load(&path).unwrap();
    assert_eq!(loaded.belief_graph.beliefs.len(), 500);
}

#[test]
fn boundary_predict_with_no_beliefs() {
    let (write, dir) = setup();
    let model_id = write.create_model().unwrap();
    let query = query_for(&dir);
    let result = query.predict_preference(&model_id, "test item");
    // Must not panic on empty model
    assert!(result.is_ok());
}

#[test]
fn boundary_simulate_with_empty_options() {
    let (write, dir) = setup();
    let model_id = write.create_model().unwrap();
    write
        .add_belief(&model_id, "Test belief".into(), BeliefDomain::Values, 0.5)
        .unwrap();
    let query = query_for(&dir);
    let options: Vec<String> = vec![];
    let result = query.simulate_decision(&model_id, "scenario", &options);
    // Should handle gracefully
    let _ = result;
}
