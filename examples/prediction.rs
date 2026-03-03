//! Prediction example — preference oracle and decision simulation

use agentic_cognition::*;
use agentic_cognition::types::*;
use tempfile::TempDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    let store = CognitionStore::with_storage(dir.path().to_path_buf())?;
    let write = WriteEngine::new(store);
    let model_id = write.create_model()?;

    // Build a model with beliefs
    write.add_belief(&model_id, "I value work-life balance".into(), BeliefDomain::Values, 0.9)?;
    write.add_belief(&model_id, "I enjoy creative work".into(), BeliefDomain::Work, 0.8)?;
    write.add_belief(&model_id, "Money is important but not everything".into(), BeliefDomain::Values, 0.7)?;

    // Query for predictions
    let store2 = CognitionStore::with_storage(dir.path().to_path_buf())?;
    let query = QueryEngine::new(store2);

    // Predict preference
    let pred = query.predict_preference(&model_id, "remote work")?;
    println!("Preference for 'remote work': {:.2} (confidence: {:.2})", pred.predicted_preference, pred.confidence);
    for reason in &pred.reasoning {
        println!("  - {reason}");
    }

    // Simulate a decision
    let sim = query.simulate_decision(
        &model_id,
        "Should I accept a higher-paying but less creative role?",
        &["Accept the role".into(), "Decline and stay".into(), "Negotiate for creative freedom".into()],
    )?;
    println!("\nDecision simulation:");
    for (i, opt) in sim.options.iter().enumerate() {
        let marker = if sim.predicted_choice == Some(i) { " <-- predicted" } else { "" };
        println!("  {}: {:.0}% probability{marker}", opt.description, opt.predicted_probability * 100.0);
    }

    // Project future self
    let future = query.project_future(&model_id, 180)?;
    println!("\n6-month projection:");
    println!("  Confidence: {:.2}", future.confidence);
    println!("  Projected belief changes: {}", future.projected_beliefs.len());
    println!("  Branch points: {}", future.branch_points.len());

    Ok(())
}
