//! Tool registry — defines all 14 MCP tools

use serde_json::{json, Value};

/// Tool definition
pub struct ToolDef {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

/// Registry of all 14 MCP tools
pub struct ToolRegistry;

impl ToolRegistry {
    pub fn all_tools() -> Vec<ToolDef> {
        vec![
            ToolDef {
                name: "cognition_model_create".into(),
                description: "Create a new living user model".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            ToolDef {
                name: "cognition_model_heartbeat".into(),
                description: "Pulse model with new observations to keep it alive".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "model_id": { "type": "string", "description": "Model UUID" },
                        "observations": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "New observations to integrate"
                        }
                    },
                    "required": ["model_id", "observations"]
                }),
            },
            ToolDef {
                name: "cognition_model_vitals".into(),
                description: "Get model health, confidence, and vital signs".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "model_id": { "type": "string", "description": "Model UUID" }
                    },
                    "required": ["model_id"]
                }),
            },
            ToolDef {
                name: "cognition_model_portrait".into(),
                description: "Get full model portrait with all components summarized".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "model_id": { "type": "string", "description": "Model UUID" }
                    },
                    "required": ["model_id"]
                }),
            },
            ToolDef {
                name: "cognition_belief_add".into(),
                description: "Add a new belief to the user model".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "model_id": { "type": "string", "description": "Model UUID" },
                        "content": { "type": "string", "description": "Belief content" },
                        "domain": { "type": "string", "description": "Belief domain (self, relationships, work, politics, religion, science, values, world_model, identity, capability, worth, other)" },
                        "confidence": { "type": "number", "description": "Confidence 0.0-1.0" }
                    },
                    "required": ["model_id", "content", "domain", "confidence"]
                }),
            },
            ToolDef {
                name: "cognition_belief_query".into(),
                description: "Query beliefs by domain, search term, or list all".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "model_id": { "type": "string", "description": "Model UUID" },
                        "domain": { "type": "string", "description": "Filter by domain (optional)" },
                        "search": { "type": "string", "description": "Search term (optional)" }
                    },
                    "required": ["model_id"]
                }),
            },
            ToolDef {
                name: "cognition_belief_graph".into(),
                description: "Get the full belief graph with connections and keystones".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "model_id": { "type": "string", "description": "Model UUID" }
                    },
                    "required": ["model_id"]
                }),
            },
            ToolDef {
                name: "cognition_soul_reflect".into(),
                description: "Perform deep soul reflection to discover user essence".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "model_id": { "type": "string", "description": "Model UUID" }
                    },
                    "required": ["model_id"]
                }),
            },
            ToolDef {
                name: "cognition_self_topology".into(),
                description: "Get self-concept topology with peaks, valleys, and blind canyons"
                    .into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "model_id": { "type": "string", "description": "Model UUID" }
                    },
                    "required": ["model_id"]
                }),
            },
            ToolDef {
                name: "cognition_pattern_fingerprint".into(),
                description: "Get decision fingerprint showing unique decision-making signature"
                    .into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "model_id": { "type": "string", "description": "Model UUID" }
                    },
                    "required": ["model_id"]
                }),
            },
            ToolDef {
                name: "cognition_shadow_map".into(),
                description: "Get shadow map of unconscious beliefs, projections, and blindspots"
                    .into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "model_id": { "type": "string", "description": "Model UUID" }
                    },
                    "required": ["model_id"]
                }),
            },
            ToolDef {
                name: "cognition_drift_track".into(),
                description: "Track belief drift, value tectonics, and metamorphoses over time"
                    .into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "model_id": { "type": "string", "description": "Model UUID" }
                    },
                    "required": ["model_id"]
                }),
            },
            ToolDef {
                name: "cognition_predict".into(),
                description: "Predict user preference for an item based on their model".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "model_id": { "type": "string", "description": "Model UUID" },
                        "item": { "type": "string", "description": "Item to predict preference for" }
                    },
                    "required": ["model_id", "item"]
                }),
            },
            ToolDef {
                name: "cognition_simulate".into(),
                description: "Simulate how user would decide in a given scenario".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "model_id": { "type": "string", "description": "Model UUID" },
                        "scenario": { "type": "string", "description": "Decision scenario" },
                        "options": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Available options"
                        }
                    },
                    "required": ["model_id", "scenario", "options"]
                }),
            },
        ]
    }

    #[allow(dead_code)]
    pub fn tool_count() -> usize {
        Self::all_tools().len()
    }
}
