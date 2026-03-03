//! Performance benchmarks for AgenticCognition (SPEC-13)

use agentic_cognition::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tempfile::TempDir;

fn setup_store() -> (CognitionStore, TempDir) {
    let dir = TempDir::new().unwrap();
    let store = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
    (store, dir)
}

fn benchmark_model_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("model_operations");

    group.bench_function("create_model", |b| {
        let (store, _dir) = setup_store();
        let engine = WriteEngine::new(store);
        b.iter(|| {
            let _ = black_box(engine.create_model().unwrap());
        });
    });

    group.bench_function("heartbeat_10_observations", |b| {
        let (store, _dir) = setup_store();
        let engine = WriteEngine::new(store);
        let model_id = engine.create_model().unwrap();
        let obs: Vec<String> = (0..10).map(|i| format!("observation {i}")).collect();

        b.iter(|| {
            engine.heartbeat(&model_id, black_box(obs.clone())).unwrap();
        });
    });

    group.finish();
}

fn benchmark_belief_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("belief_operations");

    group.bench_function("add_belief", |b| {
        let (store, _dir) = setup_store();
        let engine = WriteEngine::new(store);
        let model_id = engine.create_model().unwrap();

        b.iter(|| {
            let _ = engine
                .add_belief(
                    &model_id,
                    black_box("Test belief content".to_string()),
                    BeliefDomain::Values,
                    0.8,
                )
                .unwrap();
        });
    });

    group.bench_function("get_belief_graph_100", |b| {
        let (store, dir) = setup_store();
        let engine = WriteEngine::new(store);
        let model_id = engine.create_model().unwrap();

        for i in 0..100 {
            engine
                .add_belief(&model_id, format!("Belief {i}"), BeliefDomain::Values, 0.5)
                .unwrap();
        }

        let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
        let query = QueryEngine::new(store2);

        b.iter(|| {
            let _ = black_box(query.get_belief_graph(&model_id).unwrap());
        });
    });

    group.bench_function("search_beliefs", |b| {
        let (store, dir) = setup_store();
        let engine = WriteEngine::new(store);
        let model_id = engine.create_model().unwrap();

        for i in 0..100 {
            engine
                .add_belief(
                    &model_id,
                    format!("Belief about topic {i}"),
                    BeliefDomain::Values,
                    0.5,
                )
                .unwrap();
        }

        let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
        let query = QueryEngine::new(store2);

        b.iter(|| {
            let _ = black_box(query.search_beliefs(&model_id, "topic").unwrap());
        });
    });

    group.finish();
}

fn benchmark_predictions(c: &mut Criterion) {
    let mut group = c.benchmark_group("predictions");

    group.bench_function("predict_preference", |b| {
        let (store, dir) = setup_store();
        let engine = WriteEngine::new(store);
        let model_id = engine.create_model().unwrap();

        for i in 0..20 {
            engine
                .add_belief(&model_id, format!("Value {i}"), BeliefDomain::Values, 0.7)
                .unwrap();
        }

        let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
        let query = QueryEngine::new(store2);

        b.iter(|| {
            let _ = black_box(query.predict_preference(&model_id, "test item").unwrap());
        });
    });

    group.bench_function("simulate_decision", |b| {
        let (store, dir) = setup_store();
        let engine = WriteEngine::new(store);
        let model_id = engine.create_model().unwrap();

        for i in 0..20 {
            engine
                .add_belief(&model_id, format!("Belief {i}"), BeliefDomain::Values, 0.7)
                .unwrap();
        }

        let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
        let query = QueryEngine::new(store2);
        let options = vec!["Option A".into(), "Option B".into(), "Option C".into()];

        b.iter(|| {
            let _ = black_box(
                query
                    .simulate_decision(&model_id, "scenario", &options)
                    .unwrap(),
            );
        });
    });

    group.bench_function("soul_reflection", |b| {
        let (store, dir) = setup_store();
        let engine = WriteEngine::new(store);
        let model_id = engine.create_model().unwrap();

        for i in 0..20 {
            engine
                .add_belief(&model_id, format!("Belief {i}"), BeliefDomain::Values, 0.7)
                .unwrap();
        }

        let store2 = CognitionStore::with_storage(dir.path().to_path_buf()).unwrap();
        let query = QueryEngine::new(store2);

        b.iter(|| {
            let _ = black_box(query.soul_reflection(&model_id).unwrap());
        });
    });

    group.finish();
}

fn benchmark_persistence(c: &mut Criterion) {
    let mut group = c.benchmark_group("persistence");

    group.bench_function("save_100_beliefs", |b| {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("bench.acog");

        let model = LivingUserModel::new();
        let mut file = agentic_cognition::format::AcogFile::new(model);
        for i in 0..100 {
            file.belief_graph.add_belief(Belief::new(
                format!("Belief {i}"),
                BeliefDomain::Values,
                0.7,
            ));
        }

        b.iter(|| {
            file.save(black_box(&path)).unwrap();
        });
    });

    group.bench_function("load_100_beliefs", |b| {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("bench.acog");

        let model = LivingUserModel::new();
        let mut file = agentic_cognition::format::AcogFile::new(model);
        for i in 0..100 {
            file.belief_graph.add_belief(Belief::new(
                format!("Belief {i}"),
                BeliefDomain::Values,
                0.7,
            ));
        }
        file.save(&path).unwrap();

        b.iter(|| {
            let _ = black_box(agentic_cognition::format::AcogFile::load(&path).unwrap());
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_model_operations,
    benchmark_belief_operations,
    benchmark_predictions,
    benchmark_persistence,
);
criterion_main!(benches);
