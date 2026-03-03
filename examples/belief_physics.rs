//! Belief physics example — crystallization, collapse, drift

use agentic_cognition::*;
use agentic_cognition::types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = CognitionStore::new();
    let write = WriteEngine::new(store);
    let model_id = write.create_model()?;

    // Add a belief and strengthen it
    let belief_id = write.add_belief(
        &model_id,
        "I am good at my job".into(),
        BeliefDomain::Capability,
        0.6,
    )?;

    // Strengthen multiple times (simulating reinforcement)
    for _ in 0..5 {
        write.strengthen_belief(&model_id, &belief_id, 0.05)?;
    }

    // Crystallize (belief becomes rigid)
    write.crystallize_belief(&model_id, &belief_id)?;
    println!("Belief crystallized!");

    // Now collapse it (simulating disconfirming evidence)
    write.collapse_belief(
        &model_id,
        &belief_id,
        agentic_cognition::types::drift::CollapseTrigger::UndeniableEvidence {
            evidence: "Performance review was poor".into(),
        },
    )?;
    println!("Belief collapsed!");

    // Check the drift timeline
    let file = write.store().get_model(&model_id)?;
    println!("Drift events: {}", file.drift.events.len());

    Ok(())
}
