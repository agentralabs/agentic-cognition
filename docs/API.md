# AgenticCognition Rust API Reference

## Core Types

### LivingUserModel
The central data structure representing a user's cognitive model.

```rust
use agentic_cognition::LivingUserModel;

let model = LivingUserModel::new();
println!("Model ID: {}", model.id);
println!("Stage: {:?}", model.lifecycle_stage);
```

### Belief
A single belief with confidence, domain, and physics properties.

```rust
use agentic_cognition::{Belief, BeliefDomain};

let belief = Belief::new(
    "I value honesty".into(),
    BeliefDomain::Values,
    0.9,
);
```

### BeliefGraph
The interconnected web of all beliefs.

```rust
use agentic_cognition::BeliefGraph;

let mut graph = BeliefGraph::new();
graph.add_belief(belief);
let keystones = graph.find_keystones();
let contradictions = graph.find_contradictions();
```

## Engines

### WriteEngine
All mutation operations.

```rust
use agentic_cognition::{CognitionStore, WriteEngine};

let store = CognitionStore::with_storage("./data".into())?;
let engine = WriteEngine::new(store);

let model_id = engine.create_model()?;
engine.add_belief(&model_id, "content".into(), BeliefDomain::Values, 0.8)?;
engine.heartbeat(&model_id, vec!["observation".into()])?;
```

### QueryEngine
All read operations and invention queries.

```rust
use agentic_cognition::{CognitionStore, QueryEngine};

let store = CognitionStore::with_storage("./data".into())?;
let engine = QueryEngine::new(store);

let model = engine.get_model(&model_id)?;
let beliefs = engine.list_beliefs(&model_id)?;
let reflection = engine.soul_reflection(&model_id)?;
let prediction = engine.predict_preference(&model_id, "item")?;
```

## Error Handling

All operations return `CognitionResult<T>` which is `Result<T, CognitionError>`.

```rust
use agentic_cognition::CognitionError;

match engine.get_model(&id) {
    Ok(model) => println!("Found: {:?}", model.lifecycle_stage),
    Err(CognitionError::ModelNotFound(id)) => println!("Not found: {id}"),
    Err(e) => eprintln!("Error: {e}"),
}
```

## File Format

```rust
use agentic_cognition::format::AcogFile;

let file = AcogFile::new(model);
file.save(Path::new("user.acog"))?;
let loaded = AcogFile::load(Path::new("user.acog"))?;
```
