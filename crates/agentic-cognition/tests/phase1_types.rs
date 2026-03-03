//! Phase 1: Type system tests

use agentic_cognition::types::*;

#[test]
fn test_model_id_creation() {
    let id1 = ModelId::new();
    let id2 = ModelId::new();
    assert_ne!(id1, id2);
}

#[test]
fn test_model_id_display_and_parse() {
    let id = ModelId::new();
    let s = id.to_string();
    let parsed: ModelId = s.parse().unwrap();
    assert_eq!(id, parsed);
}

#[test]
fn test_timestamp_now() {
    let ts = Timestamp::now();
    assert!(ts.as_nanos() > 0);
}

#[test]
fn test_timestamp_ordering() {
    let t1 = Timestamp::now();
    std::thread::sleep(std::time::Duration::from_millis(1));
    let t2 = Timestamp::now();
    assert!(t2 > t1);
}

#[test]
fn test_timestamp_epoch() {
    let ts = Timestamp::epoch();
    assert_eq!(ts.as_nanos(), 0);
}

#[test]
fn test_timestamp_default_is_now() {
    let ts = Timestamp::default();
    assert!(ts.as_nanos() > 0);
}

#[test]
fn test_living_user_model_creation() {
    let model = LivingUserModel::new();
    assert_eq!(model.lifecycle_stage, ModelLifecycleStage::Birth);
    assert_eq!(model.evidence_count, 0);
    assert_eq!(model.consent, ConsentStatus::Pending);
}

#[test]
fn test_living_user_model_default() {
    let model = LivingUserModel::default();
    assert_eq!(model.lifecycle_stage, ModelLifecycleStage::Birth);
    assert_eq!(model.evidence_count, 0);
}

#[test]
fn test_living_user_model_with_id() {
    let id = ModelId::new();
    let model = LivingUserModel::new().with_id(id);
    assert_eq!(model.id, id);
}

#[test]
fn test_belief_creation() {
    let belief = Belief::new("Test belief".into(), BeliefDomain::Values, 0.8);
    assert_eq!(belief.content, "Test belief");
    assert_eq!(belief.confidence, 0.8);
    assert_eq!(belief.state, BeliefState::Forming);
    assert_eq!(belief.domain, BeliefDomain::Values);
    assert_eq!(belief.crystallization, 0.0);
    assert_eq!(belief.centrality, 0.0);
    assert!(belief.explicit);
}

#[test]
fn test_belief_domain_display() {
    assert_eq!(format!("{}", BeliefDomain::Self_), "self");
    assert_eq!(format!("{}", BeliefDomain::Relationships), "relationships");
    assert_eq!(format!("{}", BeliefDomain::Work), "work");
    assert_eq!(format!("{}", BeliefDomain::Politics), "politics");
    assert_eq!(format!("{}", BeliefDomain::Religion), "religion");
    assert_eq!(format!("{}", BeliefDomain::Science), "science");
    assert_eq!(format!("{}", BeliefDomain::Values), "values");
    assert_eq!(format!("{}", BeliefDomain::WorldModel), "world_model");
    assert_eq!(format!("{}", BeliefDomain::Identity), "identity");
    assert_eq!(format!("{}", BeliefDomain::Capability), "capability");
    assert_eq!(format!("{}", BeliefDomain::Worth), "worth");
    assert_eq!(format!("{}", BeliefDomain::Other), "other");
}

#[test]
fn test_model_lifecycle_transitions() {
    use ModelLifecycleStage::*;
    assert!(Birth.can_transition_to(&Infancy));
    assert!(Infancy.can_transition_to(&Growth));
    assert!(Growth.can_transition_to(&Maturity));
    assert!(Maturity.can_transition_to(&Stale));
    assert!(Maturity.can_transition_to(&Crisis));
    assert!(Stale.can_transition_to(&Crisis));
    assert!(Crisis.can_transition_to(&Rebirth));
    assert!(Rebirth.can_transition_to(&Growth));
    assert!(Growth.can_transition_to(&Crisis));
    assert!(Maturity.can_transition_to(&Growth)); // evolution

    // Invalid transitions
    assert!(!Birth.can_transition_to(&Maturity));
    assert!(!Maturity.can_transition_to(&Birth));
    assert!(!Birth.can_transition_to(&Growth));
    assert!(!Infancy.can_transition_to(&Maturity));
}

#[test]
fn test_belief_graph_basic() {
    let mut graph = BeliefGraph::new();
    let b1 = Belief::new("Belief 1".into(), BeliefDomain::Values, 0.9);
    let b2 = Belief::new("Belief 2".into(), BeliefDomain::Work, 0.7);
    let id1 = b1.id;
    let id2 = b2.id;

    graph.add_belief(b1);
    graph.add_belief(b2);

    assert_eq!(graph.beliefs.len(), 2);
    assert!(graph.get_belief(&id1).is_some());
    assert!(graph.get_belief(&id2).is_some());
}

#[test]
fn test_belief_graph_get_belief_mut() {
    let mut graph = BeliefGraph::new();
    let b1 = Belief::new("Mutable".into(), BeliefDomain::Values, 0.5);
    let id1 = b1.id;
    graph.add_belief(b1);

    let belief = graph.get_belief_mut(&id1).unwrap();
    belief.confidence = 0.9;

    assert_eq!(graph.get_belief(&id1).unwrap().confidence, 0.9);
}

#[test]
fn test_belief_graph_connections() {
    let mut graph = BeliefGraph::new();
    let b1 = Belief::new("Belief 1".into(), BeliefDomain::Values, 0.9);
    let b2 = Belief::new("Belief 2".into(), BeliefDomain::Values, 0.7);
    let id1 = b1.id;
    let id2 = b2.id;

    graph.add_belief(b1);
    graph.add_belief(b2);
    graph.add_connection(BeliefConnection {
        from: id1,
        to: id2,
        connection_type: ConnectionType::Supports,
        strength: 0.8,
    });

    assert_eq!(graph.connections.len(), 1);
    assert_eq!(graph.connections[0].from, id1);
    assert_eq!(graph.connections[0].to, id2);
    assert_eq!(
        graph.connections[0].connection_type,
        ConnectionType::Supports
    );
}

#[test]
fn test_belief_graph_contradictions() {
    let mut graph = BeliefGraph::new();
    let b1 = Belief::new("A is true".into(), BeliefDomain::Values, 0.9);
    let b2 = Belief::new("A is false".into(), BeliefDomain::Values, 0.7);
    let id1 = b1.id;
    let id2 = b2.id;

    graph.add_belief(b1);
    graph.add_belief(b2);
    graph.add_connection(BeliefConnection {
        from: id1,
        to: id2,
        connection_type: ConnectionType::Contradicts,
        strength: 0.9,
    });

    let contradictions = graph.find_contradictions();
    assert_eq!(contradictions.len(), 1);
    assert_eq!(contradictions[0].belief_a, id1);
    assert_eq!(contradictions[0].belief_b, id2);
}

#[test]
fn test_belief_graph_no_contradictions_for_supports() {
    let mut graph = BeliefGraph::new();
    let b1 = Belief::new("X".into(), BeliefDomain::Values, 0.9);
    let b2 = Belief::new("Y".into(), BeliefDomain::Values, 0.7);
    let id1 = b1.id;
    let id2 = b2.id;

    graph.add_belief(b1);
    graph.add_belief(b2);
    graph.add_connection(BeliefConnection {
        from: id1,
        to: id2,
        connection_type: ConnectionType::Supports,
        strength: 0.8,
    });

    let contradictions = graph.find_contradictions();
    assert!(contradictions.is_empty());
}

#[test]
fn test_belief_graph_domain_filter() {
    let mut graph = BeliefGraph::new();
    graph.add_belief(Belief::new("Work belief".into(), BeliefDomain::Work, 0.8));
    graph.add_belief(Belief::new(
        "Value belief".into(),
        BeliefDomain::Values,
        0.7,
    ));
    graph.add_belief(Belief::new("Another work".into(), BeliefDomain::Work, 0.6));

    let work_beliefs = graph.beliefs_in_domain(&BeliefDomain::Work);
    assert_eq!(work_beliefs.len(), 2);

    let value_beliefs = graph.beliefs_in_domain(&BeliefDomain::Values);
    assert_eq!(value_beliefs.len(), 1);

    let science_beliefs = graph.beliefs_in_domain(&BeliefDomain::Science);
    assert!(science_beliefs.is_empty());
}

#[test]
fn test_belief_graph_find_keystones() {
    let mut graph = BeliefGraph::new();
    let b1 = Belief::new("Foundation".into(), BeliefDomain::Values, 0.9);
    let b2 = Belief::new("Depends1".into(), BeliefDomain::Values, 0.7);
    let b3 = Belief::new("Depends2".into(), BeliefDomain::Values, 0.6);
    let id1 = b1.id;
    let id2 = b2.id;
    let id3 = b3.id;

    graph.add_belief(b1);
    graph.add_belief(b2);
    graph.add_belief(b3);

    // b2 and b3 both require b1
    graph.add_connection(BeliefConnection {
        from: id2,
        to: id1,
        connection_type: ConnectionType::Requires,
        strength: 0.9,
    });
    graph.add_connection(BeliefConnection {
        from: id3,
        to: id1,
        connection_type: ConnectionType::Requires,
        strength: 0.8,
    });

    let keystones = graph.find_keystones();
    assert_eq!(keystones.len(), 1);
    assert_eq!(keystones[0].belief_id, id1);
    assert_eq!(keystones[0].dependents.len(), 2);
}

#[test]
fn test_decision_fingerprint_default() {
    let fp = DecisionFingerprint::new(ModelId::new());
    assert_eq!(fp.confidence, 0.0);
    assert_eq!(fp.traits.risk_tolerance, 0.0);
    assert_eq!(fp.traits.information_appetite, 0.0);
    assert_eq!(fp.traits.speed_accuracy_tradeoff, 0.0);
    assert!(fp.biases.is_empty());
}

#[test]
fn test_shadow_map_default() {
    let shadow = ShadowMap::default();
    assert!(shadow.shadow_beliefs.is_empty());
    assert!(shadow.projections.is_empty());
    assert!(shadow.blindspots.is_empty());
}

#[test]
fn test_self_concept_default() {
    let topo = SelfConceptTopology::default();
    assert!(topo.peaks.is_empty());
    assert!(topo.valleys.is_empty());
    assert!(topo.blind_canyons.is_empty());
    assert!(topo.defended_territories.is_empty());
    assert!(topo.growing_edges.is_empty());
}

#[test]
fn test_bias_field_default() {
    let field = BiasField::default();
    assert!(field.biases.is_empty());
    assert!(field.triggers.is_empty());
}

#[test]
fn test_drift_timeline_default() {
    let drift = DriftTimeline::default();
    assert!(drift.events.is_empty());
    assert!(drift.value_tectonics.is_empty());
    assert!(drift.metamorphoses.is_empty());
    assert!(drift.growth_rings.is_empty());
}

#[test]
fn test_consciousness_state_default() {
    let cs = ConsciousnessState::default();
    assert_eq!(cs.life_phase, LifePhase::Exploring);
    assert_eq!(cs.cognitive_load, 0.0);
    assert_eq!(cs.energy_level, 0.5);
    assert!(cs.attention_focus.is_empty());
    assert!(cs.active_tensions.is_empty());
}

#[test]
fn test_emotional_weather_default() {
    let ew = EmotionalWeather::default();
    assert_eq!(ew.baseline_mood, Mood::Neutral);
    assert_eq!(ew.current_mood, Mood::Neutral);
    assert_eq!(ew.volatility, 0.0);
    assert!(ew.dominant_emotion.is_none());
}

#[test]
fn test_model_soul_default() {
    let soul = ModelSoul::default();
    assert!(soul.deep_values.is_empty());
    assert!(soul.drives.is_empty());
    assert!(soul.core_wound.is_none());
    assert!(soul.core_gift.is_none());
    assert_eq!(soul.authenticity_gap, 0.0);
}

#[test]
fn test_model_vitals_default() {
    let vitals = ModelVitals::default();
    assert_eq!(vitals.health, 1.0);
    assert_eq!(vitals.confidence, 0.0);
    assert_eq!(vitals.staleness_secs, 0);
    assert_eq!(vitals.evidence_count, 0);
    assert!(!vitals.in_crisis);
    assert_eq!(vitals.prediction_accuracy, 0.0);
}

#[test]
fn test_all_id_types() {
    // Ensure all ID types can be created, displayed, and parsed
    let m = ModelId::new();
    let b = BeliefId::new();
    let p = PatternId::new();
    let d = DriftId::new();
    let bs = BlindspotId::new();
    let pr = ProjectionId::new();
    let bi = BiasId::new();
    let t = TriggerId::new();
    let pred = PredictionId::new();
    let sim = SimulationId::new();
    let c = CollapseId::new();
    let e = EntanglementId::new();
    let f = FingerprintId::new();
    let th = ThreadId::new();
    let r = ReflectionId::new();
    let fo = FossilId::new();
    let s = StratumId::new();

    // All should have unique UUIDs
    assert_ne!(m.to_string(), b.to_string());

    // All should be displayable and parseable
    let m_str = m.to_string();
    let m_parsed: ModelId = m_str.parse().unwrap();
    assert_eq!(m, m_parsed);

    let b_str = b.to_string();
    let b_parsed: BeliefId = b_str.parse().unwrap();
    assert_eq!(b, b_parsed);

    // Verify Default trait works
    let _m_default = ModelId::default();
    let _b_default = BeliefId::default();

    // Verify they all have non-empty string representations
    for id_str in &[
        m.to_string(),
        b.to_string(),
        p.to_string(),
        d.to_string(),
        bs.to_string(),
        pr.to_string(),
        bi.to_string(),
        t.to_string(),
        pred.to_string(),
        sim.to_string(),
        c.to_string(),
        e.to_string(),
        f.to_string(),
        th.to_string(),
        r.to_string(),
        fo.to_string(),
        s.to_string(),
    ] {
        assert!(!id_str.is_empty());
        assert_eq!(id_str.len(), 36); // UUID format length
    }
}

#[test]
fn test_belief_state_variants() {
    // Ensure all belief states exist
    let states = [
        BeliefState::Forming,
        BeliefState::Strengthening,
        BeliefState::Crystallized,
        BeliefState::Challenged,
        BeliefState::Collapsing,
        BeliefState::Collapsed,
    ];
    assert_eq!(states.len(), 6);
}

#[test]
fn test_connection_type_variants() {
    let types = [
        ConnectionType::Supports,
        ConnectionType::Contradicts,
        ConnectionType::Requires,
        ConnectionType::Implies,
        ConnectionType::Associated,
        ConnectionType::Generalizes,
        ConnectionType::Specializes,
    ];
    assert_eq!(types.len(), 7);
}

#[test]
fn test_consent_status_variants() {
    let pending = ConsentStatus::Pending;
    let granted = ConsentStatus::Granted;
    let revoked = ConsentStatus::Revoked;
    let limited = ConsentStatus::Limited { domains_allowed: 3 };

    assert_eq!(pending, ConsentStatus::Pending);
    assert_eq!(granted, ConsentStatus::Granted);
    assert_eq!(revoked, ConsentStatus::Revoked);
    assert!(matches!(
        limited,
        ConsentStatus::Limited { domains_allowed: 3 }
    ));
}

#[test]
fn test_privacy_settings_default() {
    let ps = PrivacySettings::default();
    assert!(ps.allow_shadow_detection);
    assert!(ps.allow_prediction);
    assert!(ps.allow_drift_tracking);
    assert!(ps.excluded_domains.is_empty());
    assert_eq!(ps.retention_days, 0);
}

#[test]
fn test_belief_entanglement_type() {
    // Ensure entanglement types can be created
    let _corr = EntanglementType::Correlated;
    let _anti = EntanglementType::AntiCorrelated;
    let _prot = EntanglementType::Protective;
    let _mut = EntanglementType::MutuallyDefining;
    let _hidden = EntanglementType::Hidden {
        surface_distance: 0.5,
    };
}

#[test]
fn test_belief_graph_empty() {
    let graph = BeliefGraph::new();
    assert!(graph.beliefs.is_empty());
    assert!(graph.connections.is_empty());
    assert!(graph.entanglements.is_empty());
    assert!(graph.find_contradictions().is_empty());
    assert!(graph.find_keystones().is_empty());
    assert!(graph.beliefs_in_domain(&BeliefDomain::Values).is_empty());
}
