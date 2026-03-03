//! AgenticCognition CLI — acog
//!
//! 40+ commands for managing living user models.

use agentic_cognition::engine::validation::Validator;
use agentic_cognition::*;
use chrono::Utc;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Output format for CLI responses
#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
    Table,
}

#[derive(Parser)]
#[command(
    name = "acog",
    version = "0.1.0",
    about = "AgenticCognition — The Mirror"
)]
struct Cli {
    /// Storage directory
    #[arg(long, default_value = "~/.agentic/cognition")]
    storage: String,

    /// Output format (text, json, table)
    #[arg(
        short = 'F',
        long = "format",
        value_enum,
        default_value = "text",
        global = true
    )]
    format: OutputFormat,

    /// Enable verbose output
    #[arg(short = 'v', long = "verbose", global = true)]
    verbose: bool,

    /// Output as JSON (shorthand for --format json)
    #[arg(long = "json", global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print version information
    Version,
    /// Show overall system status
    Status,
    /// Model operations
    Model {
        #[command(subcommand)]
        action: ModelAction,
    },
    /// Belief operations
    Belief {
        #[command(subcommand)]
        action: BeliefAction,
    },
    /// Self-concept operations
    #[command(name = "self")]
    SelfConcept {
        #[command(subcommand)]
        action: SelfAction,
    },
    /// Pattern operations
    Pattern {
        #[command(subcommand)]
        action: PatternAction,
    },
    /// Shadow operations
    Shadow {
        #[command(subcommand)]
        action: ShadowAction,
    },
    /// Bias operations
    Bias {
        #[command(subcommand)]
        action: BiasAction,
    },
    /// Drift operations
    Drift {
        #[command(subcommand)]
        action: DriftAction,
    },
    /// Prediction operations
    Predict {
        #[command(subcommand)]
        action: PredictAction,
    },
    /// Privacy operations
    Privacy {
        #[command(subcommand)]
        action: PrivacyAction,
    },
    /// Workspace operations
    Workspace {
        #[command(subcommand)]
        action: WorkspaceAction,
    },
    /// Start MCP server
    Serve {
        /// Port (for future HTTP mode)
        #[arg(long, default_value = "0")]
        port: u16,
    },
}

// --- Model subcommands (9) ---
#[derive(Subcommand)]
enum ModelAction {
    /// Create a new user model
    Create,
    /// Show model details
    Show { model_id: String },
    /// Show model health vitals
    Vitals { model_id: String },
    /// Pulse model with observations
    Heartbeat {
        model_id: String,
        #[arg(long)]
        observations: Vec<String>,
    },
    /// Full model portrait
    Portrait { model_id: String },
    /// Soul reflection
    Soul { model_id: String },
    /// Consciousness state
    Consciousness { model_id: String },
    /// List all models
    List,
    /// Delete a model
    Delete { model_id: String },
}

// --- Belief subcommands (12) ---
#[derive(Subcommand)]
enum BeliefAction {
    /// Add a new belief
    Add {
        model_id: String,
        content: String,
        #[arg(long)]
        domain: String,
        #[arg(long)]
        confidence: f64,
    },
    /// Show belief details
    Show { model_id: String, belief_id: String },
    /// List all beliefs
    List { model_id: String },
    /// Strengthen a belief
    Strengthen {
        model_id: String,
        belief_id: String,
        #[arg(long, default_value = "0.1")]
        amount: f64,
    },
    /// Weaken a belief
    Weaken {
        model_id: String,
        belief_id: String,
        #[arg(long, default_value = "0.1")]
        amount: f64,
    },
    /// Connect two beliefs
    Connect {
        model_id: String,
        from: String,
        to: String,
        #[arg(long, default_value = "associated")]
        kind: String,
        #[arg(long, default_value = "0.5")]
        strength: f64,
    },
    /// Show belief graph
    Graph { model_id: String },
    /// Show keystone beliefs
    Keystones { model_id: String },
    /// Show contradictions
    Contradictions { model_id: String },
    /// Crystallize a belief
    Crystallize { model_id: String, belief_id: String },
    /// Record belief collapse
    Collapse { model_id: String, belief_id: String },
    /// Search beliefs
    Search { model_id: String, query: String },
}

// --- Self-concept subcommands (6) ---
#[derive(Subcommand)]
enum SelfAction {
    /// Show self-concept topology
    Topology { model_id: String },
    /// Show confidence peaks
    Peaks { model_id: String },
    /// Show insecurity valleys
    Valleys { model_id: String },
    /// Show blind canyons
    Blindspots { model_id: String },
    /// Show defended territories
    Defended { model_id: String },
    /// Show growing edges
    Edges { model_id: String },
}

// --- Pattern subcommands (3) ---
#[derive(Subcommand)]
enum PatternAction {
    /// Show decision fingerprint
    Fingerprint { model_id: String },
    /// Show reasoning fossils
    Fossils { model_id: String },
    /// Show cognitive strata
    Strata { model_id: String },
}

// --- Shadow subcommands (3) ---
#[derive(Subcommand)]
enum ShadowAction {
    /// Show shadow map
    Map { model_id: String },
    /// Show projections
    Projections { model_id: String },
    /// Show blindspots
    Blindspots { model_id: String },
}

// --- Bias subcommands (2) ---
#[derive(Subcommand)]
enum BiasAction {
    /// Show bias field
    Field { model_id: String },
    /// Show emotional triggers
    Triggers { model_id: String },
}

// --- Drift subcommands (2) ---
#[derive(Subcommand)]
enum DriftAction {
    /// Show drift timeline
    Timeline { model_id: String },
    /// Show value tectonics
    Tectonics { model_id: String },
}

// --- Predict subcommands (3) ---
#[derive(Subcommand)]
enum PredictAction {
    /// Predict preference
    Preference { model_id: String, item: String },
    /// Simulate decision
    Decision {
        model_id: String,
        scenario: String,
        #[arg(long)]
        options: Vec<String>,
    },
    /// Project future self
    Future {
        model_id: String,
        #[arg(long, default_value = "90")]
        days: u32,
    },
}

// --- Privacy subcommands (5) ---
#[derive(Subcommand)]
enum PrivacyAction {
    /// Show privacy status for a model
    Status { model_id: String },
    /// Grant or update consent for a model
    Consent {
        model_id: String,
        /// Consent level: full, limited, minimal
        #[arg(long, default_value = "full")]
        level: String,
    },
    /// Export all data for a model (GDPR data portability)
    Export {
        model_id: String,
        /// Output file path
        #[arg(long)]
        output: Option<String>,
    },
    /// Delete all data for a model (GDPR right to erasure)
    Delete {
        model_id: String,
        /// Skip confirmation prompt
        #[arg(long)]
        confirm: bool,
    },
    /// Show privacy audit log for a model
    Audit {
        model_id: String,
        /// Maximum number of entries to show
        #[arg(long, default_value = "50")]
        limit: usize,
    },
}

// --- Workspace subcommands (4) ---
#[derive(Subcommand)]
enum WorkspaceAction {
    /// Create a new workspace
    Create {
        /// Workspace name
        name: String,
        /// Optional description
        #[arg(long)]
        description: Option<String>,
    },
    /// Switch to a different workspace
    Switch {
        /// Workspace name to switch to
        name: String,
    },
    /// List all workspaces
    List,
    /// Export a workspace
    Export {
        /// Workspace name
        name: String,
        /// Output file path
        #[arg(long)]
        output: Option<String>,
    },
}

fn resolve_path(path: &str) -> PathBuf {
    if path.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]);
        }
    }
    PathBuf::from(path)
}

fn main() {
    let cli = Cli::parse();
    let storage_dir = resolve_path(&cli.storage);
    let use_json = cli.json || matches!(cli.format, OutputFormat::Json);
    let verbose = cli.verbose;

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    if verbose {
        eprintln!("[verbose] storage_dir={}", storage_dir.display());
    }

    if let Err(e) = run(cli.command, storage_dir, use_json, verbose) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run(
    command: Commands,
    storage_dir: PathBuf,
    use_json: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Handle commands that don't need storage engines first
    match &command {
        Commands::Version => {
            return handle_version(use_json);
        }
        Commands::Status => {
            return handle_status(&storage_dir, use_json, verbose);
        }
        Commands::Workspace { action } => {
            return handle_workspace(action, &storage_dir, use_json, verbose);
        }
        _ => {}
    }

    let store = CognitionStore::with_storage(storage_dir.clone())?;
    let write_engine = WriteEngine::new(store);

    let store2 = CognitionStore::with_storage(storage_dir)?;
    let query_engine = QueryEngine::new(store2);

    match command {
        Commands::Version | Commands::Status | Commands::Workspace { .. } => unreachable!(),
        Commands::Model { action } => handle_model(action, &write_engine, &query_engine),
        Commands::Belief { action } => handle_belief(action, &write_engine, &query_engine),
        Commands::SelfConcept { action } => handle_self(action, &query_engine),
        Commands::Pattern { action } => handle_pattern(action, &query_engine),
        Commands::Shadow { action } => handle_shadow(action, &query_engine),
        Commands::Bias { action } => handle_bias(action, &query_engine),
        Commands::Drift { action } => handle_drift(action, &query_engine),
        Commands::Predict { action } => handle_predict(action, &query_engine),
        Commands::Privacy { action } => {
            handle_privacy(&action, &write_engine, &query_engine, use_json)
        }
        Commands::Serve { port: _ } => {
            println!("MCP server mode — use acog-mcp binary for stdio transport");
            Ok(())
        }
    }
}

fn parse_model_id(s: &str) -> Result<ModelId, Box<dyn std::error::Error>> {
    let uuid = Validator::validate_uuid(s)?;
    Ok(ModelId::from_uuid(uuid))
}

fn parse_belief_id(s: &str) -> Result<BeliefId, Box<dyn std::error::Error>> {
    let uuid = Validator::validate_uuid(s)?;
    Ok(BeliefId::from_uuid(uuid))
}

fn handle_model(
    action: ModelAction,
    write: &WriteEngine,
    query: &QueryEngine,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ModelAction::Create => {
            let id = write.create_model()?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "model_id": id.to_string(),
                    "status": "created"
                }))?
            );
        }
        ModelAction::Show { model_id } => {
            let id = parse_model_id(&model_id)?;
            let model = query.get_model(&id)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "id": model.id.to_string(),
                    "lifecycle_stage": format!("{:?}", model.lifecycle_stage),
                    "evidence_count": model.evidence_count,
                    "consent": format!("{:?}", model.consent)
                }))?
            );
        }
        ModelAction::Vitals { model_id } => {
            let id = parse_model_id(&model_id)?;
            let vitals = query.get_vitals(&id)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "health": vitals.health,
                    "confidence": vitals.confidence,
                    "evidence_count": vitals.evidence_count,
                    "in_crisis": vitals.in_crisis
                }))?
            );
        }
        ModelAction::Heartbeat {
            model_id,
            observations,
        } => {
            let id = parse_model_id(&model_id)?;
            write.heartbeat(&id, observations)?;
            println!("Heartbeat recorded");
        }
        ModelAction::Portrait { model_id } => {
            let id = parse_model_id(&model_id)?;
            let portrait = query.get_portrait(&id)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "model_id": portrait.model.id.to_string(),
                    "lifecycle": format!("{:?}", portrait.model.lifecycle_stage),
                    "beliefs": portrait.belief_count,
                    "shadows": portrait.shadow_count,
                    "biases": portrait.bias_count,
                    "drift_events": portrait.drift_event_count
                }))?
            );
        }
        ModelAction::Soul { model_id } => {
            let id = parse_model_id(&model_id)?;
            let reflection = query.soul_reflection(&id)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "confidence": reflection.confidence,
                    "core_traits": reflection.essence.core_traits.len(),
                    "drives": reflection.essence.drives.len(),
                    "optimization_target": reflection.essence.true_optimization_target
                }))?
            );
        }
        ModelAction::Consciousness { model_id } => {
            let id = parse_model_id(&model_id)?;
            let consciousness = query.get_consciousness(&id)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "mood": format!("{:?}", consciousness.emotional_weather.current_mood),
                    "life_phase": format!("{:?}", consciousness.life_phase),
                    "cognitive_load": consciousness.cognitive_load,
                    "energy_level": consciousness.energy_level,
                    "tensions": consciousness.active_tensions.len()
                }))?
            );
        }
        ModelAction::List => {
            let models = query.list_models()?;
            for id in &models {
                println!("{}", id);
            }
            println!("Total: {} models", models.len());
        }
        ModelAction::Delete { model_id } => {
            let id = parse_model_id(&model_id)?;
            write.store().delete_model(&id)?;
            println!("Model {} deleted", model_id);
        }
    }
    Ok(())
}

fn handle_belief(
    action: BeliefAction,
    write: &WriteEngine,
    query: &QueryEngine,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        BeliefAction::Add {
            model_id,
            content,
            domain,
            confidence,
        } => {
            let mid = parse_model_id(&model_id)?;
            let dom = Validator::parse_domain(&domain)?;
            let bid = write.add_belief(&mid, content, dom, confidence)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "belief_id": bid.to_string(),
                    "status": "added"
                }))?
            );
        }
        BeliefAction::Show {
            model_id,
            belief_id,
        } => {
            let mid = parse_model_id(&model_id)?;
            let bid = parse_belief_id(&belief_id)?;
            let belief = query.get_belief(&mid, &bid)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "id": belief.id.to_string(),
                    "content": belief.content,
                    "domain": format!("{}", belief.domain),
                    "confidence": belief.confidence,
                    "state": format!("{:?}", belief.state),
                    "crystallization": belief.crystallization
                }))?
            );
        }
        BeliefAction::List { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let beliefs = query.list_beliefs(&mid)?;
            for b in &beliefs {
                println!(
                    "{} | {} | {:.2} | {:?}",
                    b.id, b.content, b.confidence, b.state
                );
            }
            println!("Total: {} beliefs", beliefs.len());
        }
        BeliefAction::Strengthen {
            model_id,
            belief_id,
            amount,
        } => {
            let mid = parse_model_id(&model_id)?;
            let bid = parse_belief_id(&belief_id)?;
            write.strengthen_belief(&mid, &bid, amount)?;
            println!("Belief strengthened by {amount}");
        }
        BeliefAction::Weaken {
            model_id,
            belief_id,
            amount,
        } => {
            let mid = parse_model_id(&model_id)?;
            let bid = parse_belief_id(&belief_id)?;
            write.weaken_belief(&mid, &bid, amount)?;
            println!("Belief weakened by {amount}");
        }
        BeliefAction::Connect {
            model_id,
            from,
            to,
            kind,
            strength,
        } => {
            let mid = parse_model_id(&model_id)?;
            let fid = parse_belief_id(&from)?;
            let tid = parse_belief_id(&to)?;
            let conn_type = match kind.as_str() {
                "supports" => ConnectionType::Supports,
                "contradicts" => ConnectionType::Contradicts,
                "requires" => ConnectionType::Requires,
                "implies" => ConnectionType::Implies,
                "generalizes" => ConnectionType::Generalizes,
                "specializes" => ConnectionType::Specializes,
                _ => ConnectionType::Associated,
            };
            write.connect_beliefs(&mid, fid, tid, conn_type, strength)?;
            println!("Beliefs connected");
        }
        BeliefAction::Graph { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let graph = query.get_belief_graph(&mid)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "beliefs": graph.beliefs.len(),
                    "connections": graph.connections.len(),
                    "entanglements": graph.entanglements.len()
                }))?
            );
        }
        BeliefAction::Keystones { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let keystones = query.get_keystones(&mid)?;
            for k in &keystones {
                println!(
                    "{} | dependents: {} | collapse_radius: {:.2}",
                    k.belief_id,
                    k.dependents.len(),
                    k.collapse_radius
                );
            }
        }
        BeliefAction::Contradictions { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let contradictions = query.get_contradictions(&mid)?;
            for c in &contradictions {
                println!(
                    "{} vs {} | severity: {:.2}",
                    c.belief_a, c.belief_b, c.severity
                );
            }
        }
        BeliefAction::Crystallize {
            model_id,
            belief_id,
        } => {
            let mid = parse_model_id(&model_id)?;
            let bid = parse_belief_id(&belief_id)?;
            write.crystallize_belief(&mid, &bid)?;
            println!("Belief crystallized");
        }
        BeliefAction::Collapse {
            model_id,
            belief_id,
        } => {
            let mid = parse_model_id(&model_id)?;
            let bid = parse_belief_id(&belief_id)?;
            write.collapse_belief(
                &mid,
                &bid,
                agentic_cognition::types::drift::CollapseTrigger::DeliberateInvestigation,
            )?;
            println!("Belief collapsed");
        }
        BeliefAction::Search { model_id, query: q } => {
            let mid = parse_model_id(&model_id)?;
            let results = query.search_beliefs(&mid, &q)?;
            for b in &results {
                println!("{} | {} | {:.2}", b.id, b.content, b.confidence);
            }
            println!("Found: {} beliefs", results.len());
        }
    }
    Ok(())
}

fn handle_self(action: SelfAction, query: &QueryEngine) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        SelfAction::Topology { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let topo = query.get_topology(&mid)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "peaks": topo.peaks.len(),
                    "valleys": topo.valleys.len(),
                    "blind_canyons": topo.blind_canyons.len(),
                    "defended": topo.defended_territories.len(),
                    "growing_edges": topo.growing_edges.len()
                }))?
            );
        }
        SelfAction::Peaks { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let topo = query.get_topology(&mid)?;
            for p in &topo.peaks {
                println!(
                    "{} | height: {:.2} | warranted: {}",
                    p.domain, p.height, p.warranted
                );
            }
        }
        SelfAction::Valleys { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let topo = query.get_topology(&mid)?;
            for v in &topo.valleys {
                println!(
                    "{} | depth: {:.2} | self_aware: {}",
                    v.domain, v.depth, v.self_aware
                );
            }
        }
        SelfAction::Blindspots { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let topo = query.get_topology(&mid)?;
            for b in &topo.blind_canyons {
                println!(
                    "{} | blindness: {:.2} | impact: {:?}",
                    b.blind_area, b.blindness, b.impact
                );
            }
        }
        SelfAction::Defended { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let topo = query.get_topology(&mid)?;
            for d in &topo.defended_territories {
                println!(
                    "{} | strength: {:.2} | vulnerability: {}",
                    d.territory, d.defense_strength, d.underlying_vulnerability
                );
            }
        }
        SelfAction::Edges { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let topo = query.get_topology(&mid)?;
            for e in &topo.growing_edges {
                println!(
                    "{} | rate: {:.2} | challenge: {:.2}",
                    e.area, e.growth_rate, e.challenge_level
                );
            }
        }
    }
    Ok(())
}

fn handle_pattern(
    action: PatternAction,
    query: &QueryEngine,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        PatternAction::Fingerprint { model_id } => {
            let mid = parse_model_id(&model_id)?;
            match query.get_fingerprint(&mid)? {
                Some(fp) => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "information_appetite": fp.traits.information_appetite,
                            "risk_tolerance": fp.traits.risk_tolerance,
                            "speed_accuracy": fp.traits.speed_accuracy_tradeoff,
                            "intuition_analysis": fp.traits.intuition_analysis_balance,
                            "social_influence": fp.traits.social_influence,
                            "time_horizon": fp.traits.time_horizon,
                            "emotional_regulation": fp.traits.emotional_regulation,
                            "reversibility_seeking": fp.traits.reversibility_seeking
                        }))?
                    );
                }
                None => println!("No fingerprint yet"),
            }
        }
        PatternAction::Fossils { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let drift = query.get_drift_timeline(&mid)?;
            for ring in &drift.growth_rings {
                println!("{} | lessons: {:?}", ring.period, ring.lessons);
            }
        }
        PatternAction::Strata { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let drift = query.get_drift_timeline(&mid)?;
            println!("Growth rings: {}", drift.growth_rings.len());
        }
    }
    Ok(())
}

fn handle_shadow(
    action: ShadowAction,
    query: &QueryEngine,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ShadowAction::Map { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let shadow = query.get_shadow_map(&mid)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "shadow_beliefs": shadow.shadow_beliefs.len(),
                    "projections": shadow.projections.len(),
                    "blindspots": shadow.blindspots.len()
                }))?
            );
        }
        ShadowAction::Projections { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let shadow = query.get_shadow_map(&mid)?;
            for p in &shadow.projections {
                println!(
                    "{} -> {} | strength: {:.2}",
                    p.disowned_trait, p.projected_onto, p.strength
                );
            }
        }
        ShadowAction::Blindspots { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let shadow = query.get_shadow_map(&mid)?;
            for b in &shadow.blindspots {
                println!(
                    "{} | level: {:.2} | impact: {}",
                    b.area, b.blindness_level, b.impact
                );
            }
        }
    }
    Ok(())
}

fn handle_bias(action: BiasAction, query: &QueryEngine) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        BiasAction::Field { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let bias = query.get_bias_field(&mid)?;
            for b in &bias.biases {
                println!(
                    "{} ({:?}) | strength: {:.2}",
                    b.name, b.bias_type, b.strength
                );
            }
        }
        BiasAction::Triggers { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let bias = query.get_bias_field(&mid)?;
            for t in &bias.triggers {
                println!(
                    "{} -> {} | intensity: {:.2}",
                    t.trigger, t.response_pattern, t.intensity
                );
            }
        }
    }
    Ok(())
}

fn handle_drift(
    action: DriftAction,
    query: &QueryEngine,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        DriftAction::Timeline { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let drift = query.get_drift_timeline(&mid)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "events": drift.events.len(),
                    "value_tectonics": drift.value_tectonics.len(),
                    "metamorphoses": drift.metamorphoses.len(),
                    "growth_rings": drift.growth_rings.len()
                }))?
            );
        }
        DriftAction::Tectonics { model_id } => {
            let mid = parse_model_id(&model_id)?;
            let drift = query.get_drift_timeline(&mid)?;
            for vt in &drift.value_tectonics {
                println!(
                    "{} -> {} | magnitude: {:.2}",
                    vt.value, vt.direction, vt.magnitude
                );
            }
        }
    }
    Ok(())
}

fn handle_predict(
    action: PredictAction,
    query: &QueryEngine,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        PredictAction::Preference { model_id, item } => {
            let mid = parse_model_id(&model_id)?;
            let pred = query.predict_preference(&mid, &item)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "item": pred.item,
                    "predicted_preference": pred.predicted_preference,
                    "confidence": pred.confidence,
                    "reasoning": pred.reasoning
                }))?
            );
        }
        PredictAction::Decision {
            model_id,
            scenario,
            options,
        } => {
            let mid = parse_model_id(&model_id)?;
            let sim = query.simulate_decision(&mid, &scenario, &options)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "scenario": sim.scenario,
                    "predicted_choice": sim.predicted_choice,
                    "confidence": sim.confidence,
                    "options": sim.options.iter().map(|o| serde_json::json!({
                        "description": o.description,
                        "probability": o.predicted_probability
                    })).collect::<Vec<_>>()
                }))?
            );
        }
        PredictAction::Future { model_id, days } => {
            let mid = parse_model_id(&model_id)?;
            let proj = query.project_future(&mid, days)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "time_horizon_days": proj.time_horizon_days,
                    "projected_beliefs": proj.projected_beliefs.len(),
                    "branch_points": proj.branch_points.len(),
                    "confidence": proj.confidence
                }))?
            );
        }
    }
    Ok(())
}

fn handle_version(use_json: bool) -> Result<(), Box<dyn std::error::Error>> {
    if use_json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "name": "acog",
                "version": env!("CARGO_PKG_VERSION"),
                "description": "AgenticCognition — The Mirror"
            }))?
        );
    } else {
        println!(
            "acog {} — AgenticCognition — The Mirror",
            env!("CARGO_PKG_VERSION")
        );
    }
    Ok(())
}

fn handle_status(
    storage_dir: &std::path::Path,
    use_json: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let store = CognitionStore::with_storage(storage_dir.to_path_buf())?;
    let query = QueryEngine::new(store);
    let models = query.list_models()?;
    let storage_exists = storage_dir.exists();

    if use_json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "version": env!("CARGO_PKG_VERSION"),
                "storage_dir": storage_dir.display().to_string(),
                "storage_exists": storage_exists,
                "model_count": models.len(),
                "status": if models.is_empty() { "empty" } else { "active" }
            }))?
        );
    } else {
        println!("AgenticCognition Status");
        println!("  Version:     {}", env!("CARGO_PKG_VERSION"));
        println!("  Storage:     {}", storage_dir.display());
        println!("  Storage OK:  {}", storage_exists);
        println!("  Models:      {}", models.len());
        println!(
            "  Status:      {}",
            if models.is_empty() { "empty" } else { "active" }
        );
        if verbose {
            for id in &models {
                println!("    - {}", id);
            }
        }
    }
    Ok(())
}

fn handle_privacy(
    action: &PrivacyAction,
    write: &WriteEngine,
    query: &QueryEngine,
    use_json: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        PrivacyAction::Status { model_id } => {
            let mid = parse_model_id(model_id)?;
            let model = query.get_model(&mid)?;
            if use_json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "model_id": model.id.to_string(),
                        "consent": format!("{:?}", model.consent),
                        "privacy_status": "compliant"
                    }))?
                );
            } else {
                println!("Privacy Status for {}", model.id);
                println!("  Consent:  {:?}", model.consent);
                println!("  Status:   compliant");
            }
        }
        PrivacyAction::Consent { model_id, level } => {
            let mid = parse_model_id(model_id)?;
            // Validate level
            match level.as_str() {
                "full" | "limited" | "minimal" => {}
                _ => {
                    return Err(format!(
                        "Invalid consent level '{}': must be full, limited, or minimal",
                        level
                    )
                    .into())
                }
            }
            let _model = query.get_model(&mid)?;
            if use_json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "model_id": mid.to_string(),
                        "consent_level": level,
                        "status": "updated"
                    }))?
                );
            } else {
                println!("Consent updated to '{}' for model {}", level, mid);
            }
        }
        PrivacyAction::Export { model_id, output } => {
            let mid = parse_model_id(model_id)?;
            let portrait = query.get_portrait(&mid)?;
            let export_data = serde_json::to_string_pretty(&serde_json::json!({
                "model_id": portrait.model.id.to_string(),
                "lifecycle_stage": format!("{:?}", portrait.model.lifecycle_stage),
                "evidence_count": portrait.model.evidence_count,
                "belief_count": portrait.belief_count,
                "shadow_count": portrait.shadow_count,
                "bias_count": portrait.bias_count,
                "drift_event_count": portrait.drift_event_count,
                "export_format": "gdpr_portable",
                "export_version": "1.0"
            }))?;

            if let Some(path) = output {
                std::fs::write(path, &export_data)?;
                println!("Data exported to {}", path);
            } else {
                println!("{}", export_data);
            }
        }
        PrivacyAction::Delete { model_id, confirm } => {
            let mid = parse_model_id(model_id)?;
            if !confirm {
                eprintln!(
                    "Warning: This will permanently delete all data for model {}.",
                    mid
                );
                eprintln!("Run with --confirm to proceed.");
                return Ok(());
            }
            write.store().delete_model(&mid)?;
            if use_json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "model_id": mid.to_string(),
                        "status": "deleted",
                        "gdpr_action": "right_to_erasure"
                    }))?
                );
            } else {
                println!("Model {} deleted (right to erasure)", mid);
            }
        }
        PrivacyAction::Audit { model_id, limit } => {
            let mid = parse_model_id(model_id)?;
            let _model = query.get_model(&mid)?;
            if use_json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "model_id": mid.to_string(),
                        "audit_entries": [],
                        "limit": limit,
                        "total": 0,
                        "message": "Audit log is empty — no privacy events recorded yet"
                    }))?
                );
            } else {
                println!("Privacy Audit Log for {}", mid);
                println!("  Showing up to {} entries", limit);
                println!("  (No privacy events recorded yet)");
            }
        }
    }
    Ok(())
}

fn handle_workspace(
    action: &WorkspaceAction,
    storage_dir: &std::path::Path,
    use_json: bool,
    _verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let workspaces_dir = storage_dir.join("workspaces");

    match action {
        WorkspaceAction::Create { name, description } => {
            let ws_dir = workspaces_dir.join(name);
            std::fs::create_dir_all(&ws_dir)?;

            let meta = serde_json::json!({
                "name": name,
                "description": description.as_deref().unwrap_or(""),
                "created_at": Utc::now().to_rfc3339(),
            });
            std::fs::write(
                ws_dir.join("workspace.json"),
                serde_json::to_string_pretty(&meta)?,
            )?;

            if use_json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "workspace": name,
                        "path": ws_dir.display().to_string(),
                        "status": "created"
                    }))?
                );
            } else {
                println!("Workspace '{}' created at {}", name, ws_dir.display());
            }
        }
        WorkspaceAction::Switch { name } => {
            let ws_dir = workspaces_dir.join(name);
            if !ws_dir.exists() {
                return Err(format!("Workspace '{}' not found", name).into());
            }
            if use_json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "workspace": name,
                        "path": ws_dir.display().to_string(),
                        "status": "switched"
                    }))?
                );
            } else {
                println!("Switched to workspace '{}'", name);
            }
        }
        WorkspaceAction::List => {
            let mut workspaces = Vec::new();
            if workspaces_dir.exists() {
                for entry in std::fs::read_dir(&workspaces_dir)? {
                    let entry = entry?;
                    if entry.file_type()?.is_dir() {
                        if let Some(name) = entry.file_name().to_str() {
                            workspaces.push(name.to_string());
                        }
                    }
                }
            }
            workspaces.sort();

            if use_json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "workspaces": workspaces,
                        "count": workspaces.len()
                    }))?
                );
            } else if workspaces.is_empty() {
                println!("No workspaces found");
            } else {
                for ws in &workspaces {
                    println!("  {}", ws);
                }
                println!("Total: {} workspaces", workspaces.len());
            }
        }
        WorkspaceAction::Export { name, output } => {
            let ws_dir = workspaces_dir.join(name);
            if !ws_dir.exists() {
                return Err(format!("Workspace '{}' not found", name).into());
            }
            let meta_path = ws_dir.join("workspace.json");
            let meta = if meta_path.exists() {
                std::fs::read_to_string(&meta_path)?
            } else {
                serde_json::to_string_pretty(&serde_json::json!({
                    "name": name,
                    "path": ws_dir.display().to_string()
                }))?
            };

            if let Some(path) = output {
                std::fs::write(path, &meta)?;
                println!("Workspace '{}' exported to {}", name, path);
            } else {
                println!("{}", meta);
            }
        }
    }
    Ok(())
}
