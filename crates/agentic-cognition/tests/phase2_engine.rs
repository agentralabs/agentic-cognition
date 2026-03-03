//! Phase 2: Engine tests

use agentic_cognition::engine::validation::Validator;
use agentic_cognition::types::*;
use agentic_cognition::*;
use tempfile::TempDir;

fn create_test_store() -> (CognitionStore, TempDir) {
    let dir = TempDir::new().unwrap();
    let store = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    (store, dir)
}

#[test]
fn test_create_model() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();
    assert!(!model_id.to_string().is_empty());
}

#[test]
fn test_create_and_query_model() {
    let (store, dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query = QueryEngine::new(store2);
    let model = query.get_model(&model_id).unwrap();
    assert_eq!(model.id, model_id);
    assert_eq!(model.lifecycle_stage, ModelLifecycleStage::Birth);
}

#[test]
fn test_heartbeat() {
    let (store, dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    write
        .heartbeat(
            &model_id,
            vec!["observation 1".into(), "observation 2".into()],
        )
        .unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query = QueryEngine::new(store2);
    let vitals = query.get_vitals(&model_id).unwrap();
    assert_eq!(vitals.evidence_count, 2);
}

#[test]
fn test_heartbeat_lifecycle_transition() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    // Add 6 observations to trigger Birth -> Infancy
    write
        .heartbeat(
            &model_id,
            vec![
                "obs1".into(),
                "obs2".into(),
                "obs3".into(),
                "obs4".into(),
                "obs5".into(),
                "obs6".into(),
            ],
        )
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.model.lifecycle_stage, ModelLifecycleStage::Infancy);
}

#[test]
fn test_add_belief() {
    let (store, dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    let belief_id = write
        .add_belief(&model_id, "I am capable".into(), BeliefDomain::Self_, 0.8)
        .unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query = QueryEngine::new(store2);
    let belief = query.get_belief(&model_id, &belief_id).unwrap();
    assert_eq!(belief.content, "I am capable");
    assert_eq!(belief.confidence, 0.8);
    assert_eq!(belief.domain, BeliefDomain::Self_);
}

#[test]
fn test_add_belief_empty_content_rejected() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    let result = write.add_belief(&model_id, "".into(), BeliefDomain::Values, 0.5);
    assert!(result.is_err());

    let result2 = write.add_belief(&model_id, "   ".into(), BeliefDomain::Values, 0.5);
    assert!(result2.is_err());
}

#[test]
fn test_add_belief_invalid_confidence_rejected() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    let result = write.add_belief(&model_id, "test".into(), BeliefDomain::Values, 1.5);
    assert!(result.is_err());

    let result2 = write.add_belief(&model_id, "test".into(), BeliefDomain::Values, -0.1);
    assert!(result2.is_err());
}

#[test]
fn test_strengthen_belief() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();
    let belief_id = write
        .add_belief(&model_id, "test".into(), BeliefDomain::Values, 0.5)
        .unwrap();

    write.strengthen_belief(&model_id, &belief_id, 0.2).unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    let belief = file.belief_graph.get_belief(&belief_id).unwrap();
    assert!((belief.confidence - 0.7).abs() < 0.01);
}

#[test]
fn test_strengthen_belief_caps_at_one() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();
    let belief_id = write
        .add_belief(&model_id, "test".into(), BeliefDomain::Values, 0.9)
        .unwrap();

    write.strengthen_belief(&model_id, &belief_id, 0.5).unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    let belief = file.belief_graph.get_belief(&belief_id).unwrap();
    assert_eq!(belief.confidence, 1.0);
}

#[test]
fn test_strengthen_belief_transitions_state() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();
    let belief_id = write
        .add_belief(&model_id, "test".into(), BeliefDomain::Values, 0.75)
        .unwrap();

    write.strengthen_belief(&model_id, &belief_id, 0.1).unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    let belief = file.belief_graph.get_belief(&belief_id).unwrap();
    assert_eq!(belief.state, BeliefState::Strengthening);
}

#[test]
fn test_weaken_belief() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();
    let belief_id = write
        .add_belief(&model_id, "test".into(), BeliefDomain::Values, 0.8)
        .unwrap();

    write.weaken_belief(&model_id, &belief_id, 0.3).unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    let belief = file.belief_graph.get_belief(&belief_id).unwrap();
    assert!((belief.confidence - 0.5).abs() < 0.01);
}

#[test]
fn test_weaken_belief_floors_at_zero() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();
    let belief_id = write
        .add_belief(&model_id, "test".into(), BeliefDomain::Values, 0.2)
        .unwrap();

    write.weaken_belief(&model_id, &belief_id, 0.5).unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    let belief = file.belief_graph.get_belief(&belief_id).unwrap();
    assert_eq!(belief.confidence, 0.0);
}

#[test]
fn test_weaken_belief_records_drift() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();
    let belief_id = write
        .add_belief(&model_id, "test".into(), BeliefDomain::Values, 0.8)
        .unwrap();

    write.weaken_belief(&model_id, &belief_id, 0.3).unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert!(!file.drift.events.is_empty());
    assert_eq!(file.drift.events[0].direction, DriftDirection::Weakening);
}

#[test]
fn test_weaken_belief_to_collapsing() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();
    let belief_id = write
        .add_belief(&model_id, "test".into(), BeliefDomain::Values, 0.3)
        .unwrap();

    write.weaken_belief(&model_id, &belief_id, 0.2).unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    let belief = file.belief_graph.get_belief(&belief_id).unwrap();
    assert_eq!(belief.state, BeliefState::Collapsing);
}

#[test]
fn test_crystallize_belief() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();
    let belief_id = write
        .add_belief(&model_id, "test".into(), BeliefDomain::Values, 0.9)
        .unwrap();

    write.crystallize_belief(&model_id, &belief_id).unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    let belief = file.belief_graph.get_belief(&belief_id).unwrap();
    assert_eq!(belief.state, BeliefState::Crystallized);
    assert_eq!(belief.crystallization, 1.0);
}

#[test]
fn test_connect_beliefs() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();
    let b1 = write
        .add_belief(&model_id, "A".into(), BeliefDomain::Values, 0.8)
        .unwrap();
    let b2 = write
        .add_belief(&model_id, "B".into(), BeliefDomain::Values, 0.7)
        .unwrap();

    write
        .connect_beliefs(&model_id, b1, b2, ConnectionType::Supports, 0.9)
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.belief_graph.connections.len(), 1);
    assert_eq!(file.belief_graph.connections[0].from, b1);
    assert_eq!(file.belief_graph.connections[0].to, b2);
}

#[test]
fn test_self_connection_rejected() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();
    let b1 = write
        .add_belief(&model_id, "A".into(), BeliefDomain::Values, 0.8)
        .unwrap();

    let result = write.connect_beliefs(&model_id, b1, b1, ConnectionType::Supports, 0.5);
    assert!(result.is_err());
}

#[test]
fn test_add_peak() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    write
        .add_peak(&model_id, "coding".into(), 0.9, true)
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.model.self_concept.peaks.len(), 1);
    assert_eq!(file.model.self_concept.peaks[0].domain, "coding");
    assert_eq!(file.model.self_concept.peaks[0].height, 0.9);
    assert!(file.model.self_concept.peaks[0].warranted);
    assert!(file.model.self_concept.peaks[0].reality_gap.is_none());
}

#[test]
fn test_add_peak_unwarranted() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    write
        .add_peak(&model_id, "singing".into(), 0.8, false)
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert!(!file.model.self_concept.peaks[0].warranted);
    assert!(file.model.self_concept.peaks[0].reality_gap.is_some());
}

#[test]
fn test_add_valley() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    write
        .add_valley(&model_id, "public speaking".into(), 0.7)
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.model.self_concept.valleys.len(), 1);
    assert_eq!(file.model.self_concept.valleys[0].domain, "public speaking");
    assert_eq!(file.model.self_concept.valleys[0].depth, 0.7);
}

#[test]
fn test_add_blindspot() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    write
        .add_blindspot(&model_id, "emotional intelligence".into(), 0.7)
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.model.self_concept.blind_canyons.len(), 1);
    assert_eq!(
        file.model.self_concept.blind_canyons[0].blind_area,
        "emotional intelligence"
    );
    assert_eq!(file.model.self_concept.blind_canyons[0].blindness, 0.7);
}

#[test]
fn test_add_shadow_belief() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    let shadow_id = write
        .add_shadow_belief(&model_id, "I fear rejection".into(), 0.7, None)
        .unwrap();
    assert!(!shadow_id.to_string().is_empty());

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.shadow.shadow_beliefs.len(), 1);
    assert_eq!(file.shadow.shadow_beliefs[0].content, "I fear rejection");
    assert_eq!(file.shadow.shadow_beliefs[0].strength, 0.7);
}

#[test]
fn test_add_shadow_belief_with_contradiction() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();
    let b1 = write
        .add_belief(&model_id, "I am confident".into(), BeliefDomain::Self_, 0.8)
        .unwrap();

    write
        .add_shadow_belief(&model_id, "I doubt myself".into(), 0.6, Some(b1))
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(
        file.shadow.shadow_beliefs[0].contradicts_conscious,
        Some(b1)
    );
}

#[test]
fn test_add_projection() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    let proj_id = write
        .add_projection(&model_id, "laziness".into(), "colleague".into())
        .unwrap();
    assert!(!proj_id.to_string().is_empty());

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.shadow.projections.len(), 1);
    assert_eq!(file.shadow.projections[0].disowned_trait, "laziness");
    assert_eq!(file.shadow.projections[0].projected_onto, "colleague");
}

#[test]
fn test_add_bias() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    let bias_id = write
        .add_bias(
            &model_id,
            "Confirmation".into(),
            BiasType::Confirmation,
            0.6,
        )
        .unwrap();
    assert!(!bias_id.to_string().is_empty());

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.bias_field.biases.len(), 1);
    assert_eq!(file.bias_field.biases[0].name, "Confirmation");
    assert_eq!(file.bias_field.biases[0].bias_type, BiasType::Confirmation);
}

#[test]
fn test_add_trigger() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    let trigger_id = write
        .add_trigger(&model_id, "criticism".into(), "defensive".into(), 0.8)
        .unwrap();
    assert!(!trigger_id.to_string().is_empty());

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.bias_field.triggers.len(), 1);
    assert_eq!(file.bias_field.triggers[0].trigger, "criticism");
    assert_eq!(file.bias_field.triggers[0].response_pattern, "defensive");
}

#[test]
fn test_list_models() {
    let (store, dir) = create_test_store();
    let write = WriteEngine::new(store);
    let _id1 = write.create_model().unwrap();
    let _id2 = write.create_model().unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query = QueryEngine::new(store2);
    let models = query.list_models().unwrap();
    assert_eq!(models.len(), 2);
}

#[test]
fn test_list_models_empty() {
    let (store, _dir) = create_test_store();
    let query = QueryEngine::new(store);
    let models = query.list_models().unwrap();
    assert!(models.is_empty());
}

#[test]
fn test_delete_model() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    write.store().delete_model(&model_id).unwrap();
    let result = write.store().get_model(&model_id);
    assert!(result.is_err());
}

#[test]
fn test_delete_nonexistent_model_fails() {
    let (store, _dir) = create_test_store();
    let fake_id = ModelId::new();
    let result = store.delete_model(&fake_id);
    assert!(result.is_err());
}

#[test]
fn test_get_nonexistent_model_fails() {
    let (store, _dir) = create_test_store();
    let query = QueryEngine::new(store);
    let fake_id = ModelId::new();
    let result = query.get_model(&fake_id);
    assert!(result.is_err());
}

#[test]
fn test_invalid_confidence_rejected() {
    assert!(Validator::validate_confidence(0.0).is_ok());
    assert!(Validator::validate_confidence(1.0).is_ok());
    assert!(Validator::validate_confidence(0.5).is_ok());
    assert!(Validator::validate_confidence(-0.1).is_err());
    assert!(Validator::validate_confidence(1.1).is_err());
    assert!(Validator::validate_confidence(2.0).is_err());
    assert!(Validator::validate_confidence(-1.0).is_err());
}

#[test]
fn test_validate_non_empty() {
    assert!(Validator::validate_non_empty("field", "hello").is_ok());
    assert!(Validator::validate_non_empty("field", "").is_err());
    assert!(Validator::validate_non_empty("field", "   ").is_err());
    assert!(Validator::validate_non_empty("field", "\t\n").is_err());
    assert!(Validator::validate_non_empty("field", "a").is_ok());
}

#[test]
fn test_parse_domain() {
    assert!(matches!(
        Validator::parse_domain("self"),
        Ok(BeliefDomain::Self_)
    ));
    assert!(matches!(
        Validator::parse_domain("work"),
        Ok(BeliefDomain::Work)
    ));
    assert!(matches!(
        Validator::parse_domain("values"),
        Ok(BeliefDomain::Values)
    ));
    assert!(matches!(
        Validator::parse_domain("relationships"),
        Ok(BeliefDomain::Relationships)
    ));
    assert!(matches!(
        Validator::parse_domain("politics"),
        Ok(BeliefDomain::Politics)
    ));
    assert!(matches!(
        Validator::parse_domain("religion"),
        Ok(BeliefDomain::Religion)
    ));
    assert!(matches!(
        Validator::parse_domain("science"),
        Ok(BeliefDomain::Science)
    ));
    assert!(matches!(
        Validator::parse_domain("world_model"),
        Ok(BeliefDomain::WorldModel)
    ));
    assert!(matches!(
        Validator::parse_domain("worldmodel"),
        Ok(BeliefDomain::WorldModel)
    ));
    assert!(matches!(
        Validator::parse_domain("identity"),
        Ok(BeliefDomain::Identity)
    ));
    assert!(matches!(
        Validator::parse_domain("capability"),
        Ok(BeliefDomain::Capability)
    ));
    assert!(matches!(
        Validator::parse_domain("worth"),
        Ok(BeliefDomain::Worth)
    ));
    assert!(matches!(
        Validator::parse_domain("other"),
        Ok(BeliefDomain::Other)
    ));
    assert!(Validator::parse_domain("invalid_domain").is_err());
    assert!(Validator::parse_domain("").is_err());
}

#[test]
fn test_parse_domain_case_insensitive() {
    assert!(matches!(
        Validator::parse_domain("SELF"),
        Ok(BeliefDomain::Self_)
    ));
    assert!(matches!(
        Validator::parse_domain("Work"),
        Ok(BeliefDomain::Work)
    ));
    assert!(matches!(
        Validator::parse_domain("VALUES"),
        Ok(BeliefDomain::Values)
    ));
}

#[test]
fn test_validate_lifecycle_transition() {
    use ModelLifecycleStage::*;
    assert!(Validator::validate_lifecycle_transition(&Birth, &Infancy).is_ok());
    assert!(Validator::validate_lifecycle_transition(&Birth, &Maturity).is_err());
}

#[test]
fn test_validate_uuid() {
    assert!(Validator::validate_uuid("550e8400-e29b-41d4-a716-446655440000").is_ok());
    assert!(Validator::validate_uuid("not-a-uuid").is_err());
    assert!(Validator::validate_uuid("").is_err());
}

#[test]
fn test_list_beliefs() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();
    write
        .add_belief(&model_id, "A".into(), BeliefDomain::Values, 0.8)
        .unwrap();
    write
        .add_belief(&model_id, "B".into(), BeliefDomain::Work, 0.7)
        .unwrap();

    // We use store() directly since we have the write engine
    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.belief_graph.beliefs.len(), 2);
}

#[test]
fn test_beliefs_by_domain() {
    let (store, dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();
    write
        .add_belief(&model_id, "A".into(), BeliefDomain::Values, 0.8)
        .unwrap();
    write
        .add_belief(&model_id, "B".into(), BeliefDomain::Work, 0.7)
        .unwrap();
    write
        .add_belief(&model_id, "C".into(), BeliefDomain::Values, 0.6)
        .unwrap();

    let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    let query = QueryEngine::new(store2);
    let values = query
        .beliefs_by_domain(&model_id, &BeliefDomain::Values)
        .unwrap();
    assert_eq!(values.len(), 2);

    let work = query
        .beliefs_by_domain(&model_id, &BeliefDomain::Work)
        .unwrap();
    assert_eq!(work.len(), 1);
}

#[test]
fn test_in_memory_store() {
    let store = CognitionStore::new();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();
    let belief_id = write
        .add_belief(&model_id, "test".into(), BeliefDomain::Values, 0.8)
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    let belief = file.belief_graph.get_belief(&belief_id).unwrap();
    assert_eq!(belief.content, "test");
}

#[test]
fn test_transition_lifecycle() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    write
        .transition_lifecycle(&model_id, ModelLifecycleStage::Infancy)
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.model.lifecycle_stage, ModelLifecycleStage::Infancy);
}

#[test]
fn test_transition_lifecycle_invalid() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    let result = write.transition_lifecycle(&model_id, ModelLifecycleStage::Maturity);
    assert!(result.is_err());
}

#[test]
fn test_add_defended_territory() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    write
        .add_defended_territory(
            &model_id,
            "competence".into(),
            0.8,
            "fear of inadequacy".into(),
        )
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.model.self_concept.defended_territories.len(), 1);
    assert_eq!(
        file.model.self_concept.defended_territories[0].territory,
        "competence"
    );
}

#[test]
fn test_add_growing_edge() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    write
        .add_growing_edge(&model_id, "leadership".into(), 0.6)
        .unwrap();

    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.model.self_concept.growing_edges.len(), 1);
    assert_eq!(file.model.self_concept.growing_edges[0].area, "leadership");
}

#[test]
fn test_add_fossil() {
    let (store, _dir) = create_test_store();
    let write = WriteEngine::new(store);
    let model_id = write.create_model().unwrap();

    let fossil_id = write
        .add_fossil(&model_id, "avoid conflict".into(), "childhood".into(), 0.7)
        .unwrap();

    assert!(!fossil_id.to_string().is_empty());
    let file = write.store().get_model(&model_id).unwrap();
    assert_eq!(file.drift.growth_rings.len(), 1);
}
