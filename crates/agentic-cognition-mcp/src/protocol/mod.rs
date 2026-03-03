//! MCP protocol handler

use crate::tools::ToolRegistry;
use crate::types::*;
use agentic_cognition::engine::validation::Validator;
use agentic_cognition::{CognitionStore, ModelId, QueryEngine, WriteEngine};
use serde_json::{json, Value};

/// MCP protocol handler
pub struct ProtocolHandler {
    write_engine: WriteEngine,
    query_engine: QueryEngine,
}

impl ProtocolHandler {
    pub fn new(storage_dir: std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let store = CognitionStore::with_storage(storage_dir.clone())?;
        let write_engine = WriteEngine::new(store);

        let store2 = CognitionStore::with_storage(storage_dir)?;
        let query_engine = QueryEngine::new(store2);

        Ok(Self {
            write_engine,
            query_engine,
        })
    }

    /// Access the query engine for ghost bridge context sync
    pub fn query_engine(&self) -> &QueryEngine {
        &self.query_engine
    }

    pub fn handle_request(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        match request.method.as_str() {
            "initialize" => self.handle_initialize(request),
            "tools/list" => self.handle_tools_list(request),
            "tools/call" => self.handle_tools_call(request),
            "resources/list" => self.handle_resources_list(request),
            "resources/read" => self.handle_resources_read(request),
            "prompts/list" => self.handle_prompts_list(request),
            "prompts/get" => self.handle_prompts_get(request),
            _ => JsonRpcResponse::error(
                request.id.clone(),
                METHOD_NOT_FOUND,
                format!("Unknown method: {}", request.method),
            ),
        }
    }

    fn handle_initialize(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse::success(
            request.id.clone(),
            json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {},
                    "resources": {},
                    "prompts": {}
                },
                "serverInfo": {
                    "name": "agentic-cognition",
                    "version": "0.1.0"
                }
            }),
        )
    }

    fn handle_tools_list(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        let tools: Vec<Value> = ToolRegistry::all_tools()
            .into_iter()
            .map(|t| {
                json!({
                    "name": t.name,
                    "description": t.description,
                    "inputSchema": t.input_schema,
                })
            })
            .collect();

        JsonRpcResponse::success(request.id.clone(), json!({ "tools": tools }))
    }

    fn handle_tools_call(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        let params = match &request.params {
            Some(p) => p,
            None => {
                return JsonRpcResponse::error(
                    request.id.clone(),
                    INVALID_PARAMS,
                    "Missing params".into(),
                )
            }
        };

        let tool_name = match params.get("name").and_then(|n| n.as_str()) {
            Some(n) => n,
            None => {
                return JsonRpcResponse::error(
                    request.id.clone(),
                    INVALID_PARAMS,
                    "Missing tool name".into(),
                )
            }
        };

        let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

        match self.dispatch_tool(tool_name, &arguments) {
            Ok(result) => JsonRpcResponse::success(
                request.id.clone(),
                json!({
                    "content": [{ "type": "text", "text": result }]
                }),
            ),
            Err(ToolError::NotFound(name)) => JsonRpcResponse::error(
                request.id.clone(),
                TOOL_NOT_FOUND,
                format!("Tool not found: {name}"),
            ),
            Err(ToolError::Execution(msg)) => JsonRpcResponse::tool_error(request.id.clone(), msg),
        }
    }

    // --- Resources ---

    fn handle_resources_list(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse::success(
            request.id.clone(),
            json!({
                "resources": [
                    {
                        "uri": "cognition://models",
                        "name": "All Models",
                        "description": "List of all living user models",
                        "mimeType": "application/json"
                    },
                    {
                        "uri": "cognition://models/{id}",
                        "name": "Model Detail",
                        "description": "Full detail of a specific user model",
                        "mimeType": "application/json"
                    },
                    {
                        "uri": "cognition://models/{id}/beliefs",
                        "name": "Model Beliefs",
                        "description": "All beliefs belonging to a user model",
                        "mimeType": "application/json"
                    },
                    {
                        "uri": "cognition://models/{id}/portrait",
                        "name": "Model Portrait",
                        "description": "Full portrait of a user model with all components",
                        "mimeType": "application/json"
                    },
                    {
                        "uri": "cognition://status",
                        "name": "System Status",
                        "description": "Overall system health and statistics",
                        "mimeType": "application/json"
                    }
                ]
            }),
        )
    }

    fn handle_resources_read(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        let params = match &request.params {
            Some(p) => p,
            None => {
                return JsonRpcResponse::error(
                    request.id.clone(),
                    INVALID_PARAMS,
                    "Missing params".into(),
                )
            }
        };

        let uri = match params.get("uri").and_then(|u| u.as_str()) {
            Some(u) => u,
            None => {
                return JsonRpcResponse::error(
                    request.id.clone(),
                    INVALID_PARAMS,
                    "Missing uri parameter".into(),
                )
            }
        };

        match self.read_resource(uri) {
            Ok(content) => JsonRpcResponse::success(
                request.id.clone(),
                json!({
                    "contents": [{
                        "uri": uri,
                        "mimeType": "application/json",
                        "text": content
                    }]
                }),
            ),
            Err(msg) => JsonRpcResponse::error(request.id.clone(), INVALID_PARAMS, msg),
        }
    }

    fn read_resource(&self, uri: &str) -> Result<String, String> {
        if uri == "cognition://status" {
            let models = self.query_engine.list_models().map_err(|e| e.to_string())?;
            return Ok(serde_json::to_string_pretty(&json!({
                "version": "0.1.0",
                "model_count": models.len(),
                "status": if models.is_empty() { "empty" } else { "active" }
            }))
            .unwrap_or_default());
        }

        if uri == "cognition://models" {
            let models = self.query_engine.list_models().map_err(|e| e.to_string())?;
            let ids: Vec<String> = models.iter().map(|id| id.to_string()).collect();
            return Ok(serde_json::to_string_pretty(&json!({
                "models": ids,
                "count": ids.len()
            }))
            .unwrap_or_default());
        }

        // Parse model-specific URIs: cognition://models/{id}[/suffix]
        if let Some(rest) = uri.strip_prefix("cognition://models/") {
            let (id_str, suffix) = if let Some(pos) = rest.find('/') {
                (&rest[..pos], Some(&rest[pos + 1..]))
            } else {
                (rest, None)
            };

            let uuid = Validator::validate_uuid(id_str).map_err(|e| e.to_string())?;
            let model_id = ModelId::from_uuid(uuid);

            return match suffix {
                None => {
                    // cognition://models/{id}
                    let model = self
                        .query_engine
                        .get_model(&model_id)
                        .map_err(|e| e.to_string())?;
                    Ok(serde_json::to_string_pretty(&json!({
                        "id": model.id.to_string(),
                        "lifecycle_stage": format!("{:?}", model.lifecycle_stage),
                        "evidence_count": model.evidence_count,
                        "consent": format!("{:?}", model.consent)
                    }))
                    .unwrap_or_default())
                }
                Some("beliefs") => {
                    let beliefs = self
                        .query_engine
                        .list_beliefs(&model_id)
                        .map_err(|e| e.to_string())?;
                    let result: Vec<Value> = beliefs
                        .iter()
                        .map(|b| {
                            json!({
                                "id": b.id.to_string(),
                                "content": b.content,
                                "domain": format!("{}", b.domain),
                                "confidence": b.confidence,
                                "state": format!("{:?}", b.state)
                            })
                        })
                        .collect();
                    Ok(serde_json::to_string_pretty(&json!({
                        "model_id": model_id.to_string(),
                        "beliefs": result,
                        "count": result.len()
                    }))
                    .unwrap_or_default())
                }
                Some("portrait") => {
                    let portrait = self
                        .query_engine
                        .get_portrait(&model_id)
                        .map_err(|e| e.to_string())?;
                    Ok(serde_json::to_string_pretty(&json!({
                        "model_id": portrait.model.id.to_string(),
                        "lifecycle_stage": format!("{:?}", portrait.model.lifecycle_stage),
                        "belief_count": portrait.belief_count,
                        "shadow_count": portrait.shadow_count,
                        "bias_count": portrait.bias_count,
                        "drift_event_count": portrait.drift_event_count,
                        "has_fingerprint": portrait.has_fingerprint,
                        "evidence_count": portrait.model.evidence_count
                    }))
                    .unwrap_or_default())
                }
                Some(other) => Err(format!("Unknown resource suffix: {}", other)),
            };
        }

        Err(format!("Unknown resource URI: {}", uri))
    }

    // --- Prompts ---

    fn handle_prompts_list(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse::success(
            request.id.clone(),
            json!({
                "prompts": [
                    {
                        "name": "soul_reflection",
                        "description": "Generate a deep soul reflection for a user model, revealing core traits, drives, and optimization targets",
                        "arguments": [
                            {
                                "name": "model_id",
                                "description": "The UUID of the user model",
                                "required": true
                            }
                        ]
                    },
                    {
                        "name": "decision_support",
                        "description": "Generate decision support analysis based on the user model's beliefs, biases, and patterns",
                        "arguments": [
                            {
                                "name": "model_id",
                                "description": "The UUID of the user model",
                                "required": true
                            },
                            {
                                "name": "decision",
                                "description": "The decision or scenario to analyze",
                                "required": true
                            }
                        ]
                    },
                    {
                        "name": "growth_guidance",
                        "description": "Generate personalized growth guidance based on the user model's self-concept topology and drift patterns",
                        "arguments": [
                            {
                                "name": "model_id",
                                "description": "The UUID of the user model",
                                "required": true
                            }
                        ]
                    },
                    {
                        "name": "trigger_navigation",
                        "description": "Generate guidance for navigating emotional triggers and shadow patterns around a specific topic",
                        "arguments": [
                            {
                                "name": "model_id",
                                "description": "The UUID of the user model",
                                "required": true
                            },
                            {
                                "name": "topic",
                                "description": "The topic or situation triggering the emotional response",
                                "required": true
                            }
                        ]
                    }
                ]
            }),
        )
    }

    fn handle_prompts_get(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        let params = match &request.params {
            Some(p) => p,
            None => {
                return JsonRpcResponse::error(
                    request.id.clone(),
                    INVALID_PARAMS,
                    "Missing params".into(),
                )
            }
        };

        let prompt_name = match params.get("name").and_then(|n| n.as_str()) {
            Some(n) => n,
            None => {
                return JsonRpcResponse::error(
                    request.id.clone(),
                    INVALID_PARAMS,
                    "Missing prompt name".into(),
                )
            }
        };

        let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

        match self.build_prompt(prompt_name, &arguments) {
            Ok(messages) => JsonRpcResponse::success(
                request.id.clone(),
                json!({
                    "description": self.prompt_description(prompt_name),
                    "messages": messages
                }),
            ),
            Err(msg) => JsonRpcResponse::error(request.id.clone(), INVALID_PARAMS, msg),
        }
    }

    fn prompt_description(&self, name: &str) -> &str {
        match name {
            "soul_reflection" => {
                "Deep soul reflection revealing core traits, drives, and optimization targets"
            }
            "decision_support" => {
                "Decision support analysis based on beliefs, biases, and patterns"
            }
            "growth_guidance" => {
                "Personalized growth guidance based on self-concept topology and drift"
            }
            "trigger_navigation" => {
                "Guidance for navigating emotional triggers and shadow patterns"
            }
            _ => "Unknown prompt",
        }
    }

    fn build_prompt(&self, name: &str, args: &Value) -> Result<Vec<Value>, String> {
        match name {
            "soul_reflection" => {
                let model_id = self.parse_model_id_from_args(args)?;
                let reflection = self
                    .query_engine
                    .soul_reflection(&model_id)
                    .map_err(|e| e.to_string())?;

                let traits_desc: Vec<String> = reflection
                    .essence
                    .core_traits
                    .iter()
                    .map(|t| {
                        format!(
                            "{} (strength: {:.2}, consistency: {:.2})",
                            t.trait_name, t.strength, t.consistency
                        )
                    })
                    .collect();
                let drives_desc: Vec<String> = reflection
                    .essence
                    .drives
                    .iter()
                    .map(|d| format!("{:?}", d))
                    .collect();

                Ok(vec![json!({
                    "role": "user",
                    "content": {
                        "type": "text",
                        "text": format!(
                            "Perform a deep soul reflection for this user model.\n\n\
                            Core traits: {}\n\
                            Drives: {}\n\
                            True optimization target: {}\n\
                            Deep fears: {}\n\
                            Confidence: {:.2}\n\n\
                            Reflect on what these patterns reveal about who this person truly is, \
                            what drives them at the deepest level, and what they might not see about themselves.",
                            traits_desc.join(", "),
                            drives_desc.join(", "),
                            reflection.essence.true_optimization_target,
                            reflection.essence.deep_fears.join(", "),
                            reflection.confidence
                        )
                    }
                })])
            }
            "decision_support" => {
                let model_id = self.parse_model_id_from_args(args)?;
                let decision = args
                    .get("decision")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'decision' argument".to_string())?;

                let portrait = self
                    .query_engine
                    .get_portrait(&model_id)
                    .map_err(|e| e.to_string())?;
                let bias = self
                    .query_engine
                    .get_bias_field(&model_id)
                    .map_err(|e| e.to_string())?;

                let bias_names: Vec<String> = bias
                    .biases
                    .iter()
                    .map(|b| format!("{} ({:.2})", b.name, b.strength))
                    .collect();

                Ok(vec![json!({
                    "role": "user",
                    "content": {
                        "type": "text",
                        "text": format!(
                            "Provide decision support for this user facing the following decision:\n\n\
                            Decision: {}\n\n\
                            User model context:\n\
                            - Lifecycle stage: {:?}\n\
                            - Belief count: {}\n\
                            - Known biases: {}\n\
                            - Active triggers: {}\n\n\
                            Analyze how their cognitive biases and belief system might influence this decision. \
                            Identify potential blind spots and suggest how to make this decision more aligned \
                            with their true values.",
                            decision,
                            portrait.model.lifecycle_stage,
                            portrait.belief_count,
                            if bias_names.is_empty() { "none detected".to_string() } else { bias_names.join(", ") },
                            bias.triggers.len()
                        )
                    }
                })])
            }
            "growth_guidance" => {
                let model_id = self.parse_model_id_from_args(args)?;
                let topology = self
                    .query_engine
                    .get_topology(&model_id)
                    .map_err(|e| e.to_string())?;
                let drift = self
                    .query_engine
                    .get_drift_timeline(&model_id)
                    .map_err(|e| e.to_string())?;

                let edges: Vec<String> = topology
                    .growing_edges
                    .iter()
                    .map(|e| format!("{} (rate: {:.2})", e.area, e.growth_rate))
                    .collect();
                let valleys: Vec<String> = topology
                    .valleys
                    .iter()
                    .map(|v| format!("{} (depth: {:.2})", v.domain, v.depth))
                    .collect();

                Ok(vec![json!({
                    "role": "user",
                    "content": {
                        "type": "text",
                        "text": format!(
                            "Generate personalized growth guidance for this user model.\n\n\
                            Self-concept topology:\n\
                            - Confidence peaks: {}\n\
                            - Insecurity valleys: {}\n\
                            - Blind canyons: {}\n\
                            - Defended territories: {}\n\
                            - Growing edges: {}\n\n\
                            Drift patterns:\n\
                            - Growth rings: {}\n\
                            - Value shifts: {}\n\
                            - Metamorphoses: {}\n\n\
                            Based on where this person is growing and where they are stuck, \
                            provide specific, compassionate guidance for their next growth steps.",
                            topology.peaks.len(),
                            if valleys.is_empty() { "none".to_string() } else { valleys.join(", ") },
                            topology.blind_canyons.len(),
                            topology.defended_territories.len(),
                            if edges.is_empty() { "none".to_string() } else { edges.join(", ") },
                            drift.growth_rings.len(),
                            drift.value_tectonics.len(),
                            drift.metamorphoses.len()
                        )
                    }
                })])
            }
            "trigger_navigation" => {
                let model_id = self.parse_model_id_from_args(args)?;
                let topic = args
                    .get("topic")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "Missing 'topic' argument".to_string())?;

                let shadow = self
                    .query_engine
                    .get_shadow_map(&model_id)
                    .map_err(|e| e.to_string())?;
                let bias = self
                    .query_engine
                    .get_bias_field(&model_id)
                    .map_err(|e| e.to_string())?;

                let projections: Vec<String> = shadow
                    .projections
                    .iter()
                    .map(|p| {
                        format!(
                            "{} -> {} ({:.2})",
                            p.disowned_trait, p.projected_onto, p.strength
                        )
                    })
                    .collect();
                let triggers: Vec<String> = bias
                    .triggers
                    .iter()
                    .map(|t| {
                        format!(
                            "{} -> {} ({:.2})",
                            t.trigger, t.response_pattern, t.intensity
                        )
                    })
                    .collect();

                Ok(vec![json!({
                    "role": "user",
                    "content": {
                        "type": "text",
                        "text": format!(
                            "Help this user navigate emotional triggers around the following topic:\n\n\
                            Topic: {}\n\n\
                            Shadow patterns:\n\
                            - Shadow beliefs: {}\n\
                            - Projections: {}\n\
                            - Blindspots: {}\n\n\
                            Emotional triggers:\n\
                            - Known triggers: {}\n\n\
                            Provide compassionate guidance for understanding and navigating \
                            the emotional responses that arise around this topic. Help them see \
                            what their reactions reveal about their deeper patterns.",
                            topic,
                            shadow.shadow_beliefs.len(),
                            if projections.is_empty() { "none detected".to_string() } else { projections.join(", ") },
                            shadow.blindspots.len(),
                            if triggers.is_empty() { "none detected".to_string() } else { triggers.join(", ") }
                        )
                    }
                })])
            }
            _ => Err(format!("Unknown prompt: {}", name)),
        }
    }

    fn parse_model_id_from_args(&self, args: &Value) -> Result<ModelId, String> {
        let id_str = args
            .get("model_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Missing 'model_id' argument".to_string())?;
        let uuid = Validator::validate_uuid(id_str).map_err(|e| e.to_string())?;
        Ok(ModelId::from_uuid(uuid))
    }

    fn dispatch_tool(&self, name: &str, args: &Value) -> Result<String, ToolError> {
        match name {
            "cognition_model_create" => self.tool_model_create(args),
            "cognition_model_heartbeat" => self.tool_model_heartbeat(args),
            "cognition_model_vitals" => self.tool_model_vitals(args),
            "cognition_model_portrait" => self.tool_model_portrait(args),
            "cognition_belief_add" => self.tool_belief_add(args),
            "cognition_belief_query" => self.tool_belief_query(args),
            "cognition_belief_graph" => self.tool_belief_graph(args),
            "cognition_soul_reflect" => self.tool_soul_reflect(args),
            "cognition_self_topology" => self.tool_self_topology(args),
            "cognition_pattern_fingerprint" => self.tool_pattern_fingerprint(args),
            "cognition_shadow_map" => self.tool_shadow_map(args),
            "cognition_drift_track" => self.tool_drift_track(args),
            "cognition_predict" => self.tool_predict(args),
            "cognition_simulate" => self.tool_simulate(args),
            _ => Err(ToolError::NotFound(name.to_string())),
        }
    }

    // --- Tool implementations ---

    fn tool_model_create(&self, _args: &Value) -> Result<String, ToolError> {
        let model_id = self
            .write_engine
            .create_model()
            .map_err(|e| ToolError::Execution(e.to_string()))?;
        Ok(serde_json::to_string_pretty(&json!({
            "model_id": model_id.to_string(),
            "status": "created",
            "lifecycle_stage": "Birth"
        }))
        .unwrap_or_default())
    }

    fn tool_model_heartbeat(&self, args: &Value) -> Result<String, ToolError> {
        let model_id = self.parse_model_id(args)?;
        let observations: Vec<String> = args
            .get("observations")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        self.write_engine
            .heartbeat(&model_id, observations.clone())
            .map_err(|e| ToolError::Execution(e.to_string()))?;

        Ok(serde_json::to_string_pretty(&json!({
            "status": "heartbeat_recorded",
            "observations_count": observations.len()
        }))
        .unwrap_or_default())
    }

    fn tool_model_vitals(&self, args: &Value) -> Result<String, ToolError> {
        let model_id = self.parse_model_id(args)?;
        let vitals = self
            .query_engine
            .get_vitals(&model_id)
            .map_err(|e| ToolError::Execution(e.to_string()))?;

        Ok(serde_json::to_string_pretty(&json!({
            "health": vitals.health,
            "confidence": vitals.confidence,
            "evidence_count": vitals.evidence_count,
            "in_crisis": vitals.in_crisis,
            "prediction_accuracy": vitals.prediction_accuracy,
            "staleness_secs": vitals.staleness_secs
        }))
        .unwrap_or_default())
    }

    fn tool_model_portrait(&self, args: &Value) -> Result<String, ToolError> {
        let model_id = self.parse_model_id(args)?;
        let portrait = self
            .query_engine
            .get_portrait(&model_id)
            .map_err(|e| ToolError::Execution(e.to_string()))?;

        Ok(serde_json::to_string_pretty(&json!({
            "model_id": portrait.model.id.to_string(),
            "lifecycle_stage": format!("{:?}", portrait.model.lifecycle_stage),
            "belief_count": portrait.belief_count,
            "shadow_count": portrait.shadow_count,
            "bias_count": portrait.bias_count,
            "drift_event_count": portrait.drift_event_count,
            "has_fingerprint": portrait.has_fingerprint,
            "evidence_count": portrait.model.evidence_count
        }))
        .unwrap_or_default())
    }

    fn tool_belief_add(&self, args: &Value) -> Result<String, ToolError> {
        let model_id = self.parse_model_id(args)?;
        let content = args
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::Execution("Missing content".into()))?;
        let domain_str = args
            .get("domain")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::Execution("Missing domain".into()))?;
        let confidence = args
            .get("confidence")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| ToolError::Execution("Missing confidence".into()))?;

        let domain =
            Validator::parse_domain(domain_str).map_err(|e| ToolError::Execution(e.to_string()))?;

        let belief_id = self
            .write_engine
            .add_belief(&model_id, content.to_string(), domain, confidence)
            .map_err(|e| ToolError::Execution(e.to_string()))?;

        Ok(serde_json::to_string_pretty(&json!({
            "belief_id": belief_id.to_string(),
            "status": "added"
        }))
        .unwrap_or_default())
    }

    fn tool_belief_query(&self, args: &Value) -> Result<String, ToolError> {
        let model_id = self.parse_model_id(args)?;

        let beliefs = if let Some(search) = args.get("search").and_then(|v| v.as_str()) {
            self.query_engine.search_beliefs(&model_id, search)
        } else if let Some(domain_str) = args.get("domain").and_then(|v| v.as_str()) {
            let domain = Validator::parse_domain(domain_str)
                .map_err(|e| ToolError::Execution(e.to_string()))?;
            self.query_engine.beliefs_by_domain(&model_id, &domain)
        } else {
            self.query_engine.list_beliefs(&model_id)
        }
        .map_err(|e| ToolError::Execution(e.to_string()))?;

        let result: Vec<Value> = beliefs
            .iter()
            .map(|b| {
                json!({
                    "id": b.id.to_string(),
                    "content": b.content,
                    "domain": format!("{}", b.domain),
                    "confidence": b.confidence,
                    "state": format!("{:?}", b.state),
                    "crystallization": b.crystallization
                })
            })
            .collect();

        Ok(serde_json::to_string_pretty(&json!({
            "beliefs": result,
            "count": result.len()
        }))
        .unwrap_or_default())
    }

    fn tool_belief_graph(&self, args: &Value) -> Result<String, ToolError> {
        let model_id = self.parse_model_id(args)?;
        let graph = self
            .query_engine
            .get_belief_graph(&model_id)
            .map_err(|e| ToolError::Execution(e.to_string()))?;
        let keystones = self
            .query_engine
            .get_keystones(&model_id)
            .map_err(|e| ToolError::Execution(e.to_string()))?;
        let contradictions = self
            .query_engine
            .get_contradictions(&model_id)
            .map_err(|e| ToolError::Execution(e.to_string()))?;

        Ok(serde_json::to_string_pretty(&json!({
            "belief_count": graph.beliefs.len(),
            "connection_count": graph.connections.len(),
            "entanglement_count": graph.entanglements.len(),
            "keystone_count": keystones.len(),
            "contradiction_count": contradictions.len()
        }))
        .unwrap_or_default())
    }

    fn tool_soul_reflect(&self, args: &Value) -> Result<String, ToolError> {
        let model_id = self.parse_model_id(args)?;
        let reflection = self
            .query_engine
            .soul_reflection(&model_id)
            .map_err(|e| ToolError::Execution(e.to_string()))?;

        let traits: Vec<Value> = reflection
            .essence
            .core_traits
            .iter()
            .map(|t| {
                json!({
                    "name": t.trait_name,
                    "strength": t.strength,
                    "consistency": t.consistency,
                    "self_aware": t.self_aware
                })
            })
            .collect();

        Ok(serde_json::to_string_pretty(&json!({
            "reflection_id": reflection.reflection_id.to_string(),
            "confidence": reflection.confidence,
            "core_traits": traits,
            "optimization_target": reflection.essence.true_optimization_target,
            "deep_fears": reflection.essence.deep_fears
        }))
        .unwrap_or_default())
    }

    fn tool_self_topology(&self, args: &Value) -> Result<String, ToolError> {
        let model_id = self.parse_model_id(args)?;
        let topology = self
            .query_engine
            .get_topology(&model_id)
            .map_err(|e| ToolError::Execution(e.to_string()))?;

        Ok(serde_json::to_string_pretty(&json!({
            "peaks": topology.peaks.len(),
            "valleys": topology.valleys.len(),
            "blind_canyons": topology.blind_canyons.len(),
            "defended_territories": topology.defended_territories.len(),
            "growing_edges": topology.growing_edges.len()
        }))
        .unwrap_or_default())
    }

    fn tool_pattern_fingerprint(&self, args: &Value) -> Result<String, ToolError> {
        let model_id = self.parse_model_id(args)?;
        let fp = self
            .query_engine
            .get_fingerprint(&model_id)
            .map_err(|e| ToolError::Execution(e.to_string()))?;

        match fp {
            Some(fp) => Ok(serde_json::to_string_pretty(&json!({
                "fingerprint_id": fp.id.to_string(),
                "confidence": fp.confidence,
                "traits": {
                    "information_appetite": fp.traits.information_appetite,
                    "risk_tolerance": fp.traits.risk_tolerance,
                    "speed_accuracy_tradeoff": fp.traits.speed_accuracy_tradeoff,
                    "intuition_analysis_balance": fp.traits.intuition_analysis_balance,
                    "social_influence": fp.traits.social_influence,
                    "time_horizon": fp.traits.time_horizon,
                    "emotional_regulation": fp.traits.emotional_regulation,
                    "reversibility_seeking": fp.traits.reversibility_seeking
                }
            }))
            .unwrap_or_default()),
            None => Ok(r#"{"fingerprint": null, "message": "No fingerprint yet"}"#.into()),
        }
    }

    fn tool_shadow_map(&self, args: &Value) -> Result<String, ToolError> {
        let model_id = self.parse_model_id(args)?;
        let shadow = self
            .query_engine
            .get_shadow_map(&model_id)
            .map_err(|e| ToolError::Execution(e.to_string()))?;

        Ok(serde_json::to_string_pretty(&json!({
            "shadow_beliefs": shadow.shadow_beliefs.len(),
            "projections": shadow.projections.len(),
            "blindspots": shadow.blindspots.len()
        }))
        .unwrap_or_default())
    }

    fn tool_drift_track(&self, args: &Value) -> Result<String, ToolError> {
        let model_id = self.parse_model_id(args)?;
        let drift = self
            .query_engine
            .get_drift_timeline(&model_id)
            .map_err(|e| ToolError::Execution(e.to_string()))?;

        Ok(serde_json::to_string_pretty(&json!({
            "drift_events": drift.events.len(),
            "value_tectonics": drift.value_tectonics.len(),
            "metamorphoses": drift.metamorphoses.len(),
            "growth_rings": drift.growth_rings.len()
        }))
        .unwrap_or_default())
    }

    fn tool_predict(&self, args: &Value) -> Result<String, ToolError> {
        let model_id = self.parse_model_id(args)?;
        let item = args
            .get("item")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::Execution("Missing item".into()))?;

        let prediction = self
            .query_engine
            .predict_preference(&model_id, item)
            .map_err(|e| ToolError::Execution(e.to_string()))?;

        Ok(serde_json::to_string_pretty(&json!({
            "prediction_id": prediction.id.to_string(),
            "item": prediction.item,
            "predicted_preference": prediction.predicted_preference,
            "confidence": prediction.confidence,
            "reasoning": prediction.reasoning
        }))
        .unwrap_or_default())
    }

    fn tool_simulate(&self, args: &Value) -> Result<String, ToolError> {
        let model_id = self.parse_model_id(args)?;
        let scenario = args
            .get("scenario")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::Execution("Missing scenario".into()))?;
        let options: Vec<String> = args
            .get("options")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .ok_or_else(|| ToolError::Execution("Missing options".into()))?;

        let sim = self
            .query_engine
            .simulate_decision(&model_id, scenario, &options)
            .map_err(|e| ToolError::Execution(e.to_string()))?;

        let option_results: Vec<Value> = sim
            .options
            .iter()
            .map(|o| {
                json!({
                    "description": o.description,
                    "predicted_probability": o.predicted_probability,
                    "alignment_factors": o.alignment_factors,
                    "resistance_factors": o.resistance_factors
                })
            })
            .collect();

        Ok(serde_json::to_string_pretty(&json!({
            "simulation_id": sim.id.to_string(),
            "scenario": sim.scenario,
            "predicted_choice": sim.predicted_choice,
            "options": option_results,
            "confidence": sim.confidence
        }))
        .unwrap_or_default())
    }

    // --- Helpers ---

    fn parse_model_id(&self, args: &Value) -> Result<ModelId, ToolError> {
        let id_str = args
            .get("model_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::Execution("Missing model_id".into()))?;
        let uuid =
            Validator::validate_uuid(id_str).map_err(|e| ToolError::Execution(e.to_string()))?;
        Ok(ModelId::from_uuid(uuid))
    }
}

enum ToolError {
    NotFound(String),
    Execution(String),
}
