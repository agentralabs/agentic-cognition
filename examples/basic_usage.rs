//! Basic usage example for AgenticCognition

use agentic_cognition::*;
use agentic_cognition::types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an in-memory store
    let store = CognitionStore::new();
    let write = WriteEngine::new(store);

    // Create a new user model
    let model_id = write.create_model()?;
    println!("Created model: {model_id}");

    // Add beliefs
    let b1 = write.add_belief(&model_id, "I value honesty".into(), BeliefDomain::Values, 0.9)?;
    let b2 = write.add_belief(&model_id, "Hard work pays off".into(), BeliefDomain::WorldModel, 0.7)?;
    let b3 = write.add_belief(&model_id, "I am creative".into(), BeliefDomain::Self_, 0.8)?;
    println!("Added 3 beliefs");

    // Connect beliefs
    write.connect_beliefs(&model_id, b1, b2, ConnectionType::Supports, 0.6)?;
    println!("Connected beliefs");

    // Heartbeat
    write.heartbeat(&model_id, vec!["User discussed career goals".into()])?;
    println!("Heartbeat recorded");

    // Query the model
    let store2 = CognitionStore::new();
    let query = QueryEngine::new(store2);

    // Note: In a real app, you'd use persistent storage:
    // let store = CognitionStore::with_storage("./data".into())?;

    println!("\nDone! Model {model_id} has beliefs and connections.");
    Ok(())
}
