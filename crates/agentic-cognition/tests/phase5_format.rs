//! Phase 5: File format tests

use agentic_cognition::format::AcogFile;
use agentic_cognition::types::*;
use tempfile::TempDir;

#[test]
fn test_acog_save_and_load() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test.acog");

    let model = LivingUserModel::new();
    let model_id = model.id;
    let file = AcogFile::new(model);

    file.save(&path).unwrap();
    assert!(path.exists());

    let loaded = AcogFile::load(&path).unwrap();
    assert_eq!(loaded.model.id, model_id);
    assert_eq!(loaded.model.lifecycle_stage, ModelLifecycleStage::Birth);
}

#[test]
fn test_acog_with_beliefs() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test_beliefs.acog");

    let model = LivingUserModel::new();
    let mut file = AcogFile::new(model);

    // Add beliefs
    file.belief_graph.add_belief(Belief::new(
        "Test belief 1".into(),
        BeliefDomain::Values,
        0.8,
    ));
    file.belief_graph
        .add_belief(Belief::new("Test belief 2".into(), BeliefDomain::Work, 0.7));

    file.save(&path).unwrap();
    let loaded = AcogFile::load(&path).unwrap();
    assert_eq!(loaded.belief_graph.beliefs.len(), 2);
}

#[test]
fn test_acog_with_shadow() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test_shadow.acog");

    let model = LivingUserModel::new();
    let mut file = AcogFile::new(model);

    file.shadow.shadow_beliefs.push(ShadowBelief {
        id: BeliefId::new(),
        content: "I fear failure".into(),
        evidence: Vec::new(),
        strength: 0.6,
        contradicts_conscious: None,
        behavioral_signs: Vec::new(),
        detected_at: Timestamp::now(),
    });

    file.save(&path).unwrap();
    let loaded = AcogFile::load(&path).unwrap();
    assert_eq!(loaded.shadow.shadow_beliefs.len(), 1);
    assert_eq!(loaded.shadow.shadow_beliefs[0].content, "I fear failure");
}

#[test]
fn test_acog_with_bias_field() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test_bias.acog");

    let model = LivingUserModel::new();
    let mut file = AcogFile::new(model);

    file.bias_field.biases.push(ActiveBias {
        id: BiasId::new(),
        name: "Confirmation".into(),
        bias_type: BiasType::Confirmation,
        strength: 0.5,
        domains_affected: Vec::new(),
        evidence: Vec::new(),
        self_aware: false,
        detected_at: Timestamp::now(),
    });

    file.save(&path).unwrap();
    let loaded = AcogFile::load(&path).unwrap();
    assert_eq!(loaded.bias_field.biases.len(), 1);
}

#[test]
fn test_acog_with_drift() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test_drift.acog");

    let model = LivingUserModel::new();
    let mut file = AcogFile::new(model);

    file.drift.value_tectonics.push(ValueTectonic {
        value: "career ambition".into(),
        direction: "decreasing".into(),
        magnitude: 0.3,
        started_at: Timestamp::now(),
        last_observed: Timestamp::now(),
        evidence: Vec::new(),
    });

    file.save(&path).unwrap();
    let loaded = AcogFile::load(&path).unwrap();
    assert_eq!(loaded.drift.value_tectonics.len(), 1);
    assert_eq!(loaded.drift.value_tectonics[0].value, "career ambition");
}

#[test]
fn test_acog_with_fingerprint() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test_fp.acog");

    let model = LivingUserModel::new();
    let mid = model.id;
    let mut file = AcogFile::new(model);

    file.fingerprint = Some(DecisionFingerprint::new(mid));

    file.save(&path).unwrap();
    let loaded = AcogFile::load(&path).unwrap();
    assert!(loaded.fingerprint.is_some());
    assert_eq!(loaded.fingerprint.unwrap().model_id, mid);
}

#[test]
fn test_acog_with_connections() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test_conn.acog");

    let model = LivingUserModel::new();
    let mut file = AcogFile::new(model);

    let b1 = Belief::new("Belief 1".into(), BeliefDomain::Values, 0.9);
    let b2 = Belief::new("Belief 2".into(), BeliefDomain::Values, 0.7);
    let id1 = b1.id;
    let id2 = b2.id;
    file.belief_graph.add_belief(b1);
    file.belief_graph.add_belief(b2);
    file.belief_graph.add_connection(BeliefConnection {
        from: id1,
        to: id2,
        connection_type: ConnectionType::Supports,
        strength: 0.8,
    });

    file.save(&path).unwrap();
    let loaded = AcogFile::load(&path).unwrap();
    assert_eq!(loaded.belief_graph.connections.len(), 1);
    assert_eq!(loaded.belief_graph.connections[0].from, id1);
}

#[test]
fn test_acog_with_entanglements() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test_ent.acog");

    let model = LivingUserModel::new();
    let mut file = AcogFile::new(model);

    let b1 = Belief::new("B1".into(), BeliefDomain::Values, 0.9);
    let b2 = Belief::new("B2".into(), BeliefDomain::Values, 0.7);
    let id1 = b1.id;
    let id2 = b2.id;
    file.belief_graph.add_belief(b1);
    file.belief_graph.add_belief(b2);
    file.belief_graph.entanglements.push(BeliefEntanglement {
        id: EntanglementId::new(),
        beliefs: vec![id1, id2],
        entanglement_type: EntanglementType::Correlated,
        strength: 0.8,
        conscious: false,
    });

    file.save(&path).unwrap();
    let loaded = AcogFile::load(&path).unwrap();
    assert_eq!(loaded.belief_graph.entanglements.len(), 1);
}

#[test]
fn test_acog_checksum_verification() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test_checksum.acog");

    let file = AcogFile::new(LivingUserModel::new());
    file.save(&path).unwrap();

    // Corrupt the file by flipping bits in the body
    let mut data = std::fs::read(&path).unwrap();
    if data.len() > 50 {
        data[50] ^= 0xFF; // flip bits
        std::fs::write(&path, &data).unwrap();
    }

    let result = AcogFile::load(&path);
    assert!(result.is_err());
}

#[test]
fn test_acog_invalid_magic() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("bad.acog");
    // Write enough bytes to pass the size check, but with wrong magic
    let mut data = vec![0u8; 100];
    data[0..4].copy_from_slice(b"XXXX");
    std::fs::write(&path, &data).unwrap();

    let result = AcogFile::load(&path);
    assert!(result.is_err());
    match result {
        Err(CognitionError::FormatError(msg)) => assert!(msg.contains("magic")),
        _ => panic!("Expected FormatError with magic bytes message"),
    }
}

#[test]
fn test_acog_too_small() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tiny.acog");
    std::fs::write(&path, b"ACO").unwrap();

    let result = AcogFile::load(&path);
    assert!(result.is_err());
    match result {
        Err(CognitionError::FormatError(msg)) => assert!(msg.contains("too small")),
        _ => panic!("Expected FormatError about file being too small"),
    }
}

#[test]
fn test_acog_empty_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("empty.acog");
    std::fs::write(&path, b"").unwrap();

    let result = AcogFile::load(&path);
    assert!(result.is_err());
}

#[test]
fn test_atomic_write() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("atomic.acog");

    let file = AcogFile::new(LivingUserModel::new());
    file.save(&path).unwrap();

    // Temp file should not exist after save
    let temp_path = path.with_extension("acog.tmp");
    assert!(!temp_path.exists());

    // Original file should exist
    assert!(path.exists());
}

#[test]
fn test_overwrite_existing_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("overwrite.acog");

    // Write first version
    let model1 = LivingUserModel::new();
    let id1 = model1.id;
    let file1 = AcogFile::new(model1);
    file1.save(&path).unwrap();

    // Write second version
    let model2 = LivingUserModel::new();
    let id2 = model2.id;
    let file2 = AcogFile::new(model2);
    file2.save(&path).unwrap();

    // Should load the second version
    let loaded = AcogFile::load(&path).unwrap();
    assert_eq!(loaded.model.id, id2);
    assert_ne!(loaded.model.id, id1);
}

#[test]
fn test_acog_roundtrip_preserves_all_fields() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("roundtrip.acog");

    let model = LivingUserModel::new();
    let model_id = model.id;
    let mut file = AcogFile::new(model);

    // Populate all sections
    let b1 = Belief::new("Core belief".into(), BeliefDomain::Values, 0.9);
    let b1_id = b1.id;
    file.belief_graph.add_belief(b1);

    file.shadow.shadow_beliefs.push(ShadowBelief {
        id: BeliefId::new(),
        content: "Shadow".into(),
        evidence: Vec::new(),
        strength: 0.5,
        contradicts_conscious: None,
        behavioral_signs: vec!["sign1".into()],
        detected_at: Timestamp::now(),
    });

    file.bias_field.biases.push(ActiveBias {
        id: BiasId::new(),
        name: "Bias".into(),
        bias_type: BiasType::Anchoring,
        strength: 0.4,
        domains_affected: vec!["work".into()],
        evidence: Vec::new(),
        self_aware: true,
        detected_at: Timestamp::now(),
    });

    file.drift.value_tectonics.push(ValueTectonic {
        value: "freedom".into(),
        direction: "increasing".into(),
        magnitude: 0.2,
        started_at: Timestamp::now(),
        last_observed: Timestamp::now(),
        evidence: vec!["evidence1".into()],
    });

    file.fingerprint = Some(DecisionFingerprint::new(model_id));

    file.save(&path).unwrap();
    let loaded = AcogFile::load(&path).unwrap();

    assert_eq!(loaded.model.id, model_id);
    assert_eq!(loaded.belief_graph.beliefs.len(), 1);
    assert!(loaded.belief_graph.get_belief(&b1_id).is_some());
    assert_eq!(loaded.shadow.shadow_beliefs.len(), 1);
    assert_eq!(loaded.shadow.shadow_beliefs[0].behavioral_signs.len(), 1);
    assert_eq!(loaded.bias_field.biases.len(), 1);
    assert!(loaded.bias_field.biases[0].self_aware);
    assert_eq!(loaded.drift.value_tectonics.len(), 1);
    assert_eq!(loaded.drift.value_tectonics[0].evidence.len(), 1);
    assert!(loaded.fingerprint.is_some());
}

#[test]
fn test_nonexistent_file_returns_error() {
    let result = AcogFile::load(std::path::Path::new(
        "/tmp/does_not_exist_cognition_test.acog",
    ));
    assert!(result.is_err());
}

#[test]
fn test_acog_new_has_empty_sections() {
    let file = AcogFile::new(LivingUserModel::new());
    assert!(file.belief_graph.beliefs.is_empty());
    assert!(file.belief_graph.connections.is_empty());
    assert!(file.belief_graph.entanglements.is_empty());
    assert!(file.shadow.shadow_beliefs.is_empty());
    assert!(file.shadow.projections.is_empty());
    assert!(file.shadow.blindspots.is_empty());
    assert!(file.bias_field.biases.is_empty());
    assert!(file.bias_field.triggers.is_empty());
    assert!(file.drift.events.is_empty());
    assert!(file.drift.value_tectonics.is_empty());
    assert!(file.drift.metamorphoses.is_empty());
    assert!(file.drift.growth_rings.is_empty());
    assert!(file.fingerprint.is_none());
}
