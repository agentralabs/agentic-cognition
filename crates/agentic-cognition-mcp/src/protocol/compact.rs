//! Compact MCP tool facade — collapses 14 tools into 3 grouped facades

use serde_json::{json, Value};

/// Compact facade group definitions
const MODEL_OPS: &[&str] = &[
    "model_create",
    "model_heartbeat",
    "model_vitals",
    "model_portrait",
    "belief_add",
    "belief_query",
    "belief_graph",
];

const DEEP_OPS: &[&str] = &[
    "soul_reflect",
    "self_topology",
    "pattern_fingerprint",
    "shadow_map",
    "drift_track",
];

const PREDICT_OPS: &[&str] = &["predict", "simulate"];

/// Check whether the compact tool surface is active
pub fn mcp_tool_surface_is_compact() -> bool {
    std::env::var("COGNITION_MCP_COMPACT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Build an input schema for a compact facade tool with an `operation` enum
pub fn compact_op_schema(ops: &[&str], description: &str) -> Value {
    json!({
        "type": "object",
        "description": description,
        "properties": {
            "operation": {
                "type": "string",
                "enum": ops,
                "description": "Operation to perform"
            },
            "args": {
                "type": "object",
                "description": "Arguments forwarded to the underlying tool",
                "additionalProperties": true
            }
        },
        "required": ["operation"]
    })
}

/// Return the three compact tool definitions
pub fn compact_tool_definitions() -> Vec<Value> {
    vec![
        json!({
            "name": "cognition_model",
            "description": "Manage user models and beliefs",
            "inputSchema": compact_op_schema(
                MODEL_OPS,
                "Model lifecycle and belief management"
            )
        }),
        json!({
            "name": "cognition_deep",
            "description": "Explore deep psychological structure",
            "inputSchema": compact_op_schema(
                DEEP_OPS,
                "Soul reflection, topology, fingerprints, shadows, drift"
            )
        }),
        json!({
            "name": "cognition_predict",
            "description": "Predict preferences and simulate decisions",
            "inputSchema": compact_op_schema(
                PREDICT_OPS,
                "Prediction and decision simulation"
            )
        }),
    ]
}

/// Decode a compact tool call into (operation, args)
pub fn decode_compact_operation(args: Value) -> Result<(String, Value), String> {
    let operation = args
        .get("operation")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing 'operation' field".to_string())?
        .to_string();

    let inner_args = args
        .get("args")
        .cloned()
        .unwrap_or_else(|| json!({}));

    Ok((operation, inner_args))
}

/// Resolve a compact group + operation to the canonical tool name
pub fn resolve_compact_tool(group: &str, operation: &str) -> Option<String> {
    let ops: &[&str] = match group {
        "cognition_model" => MODEL_OPS,
        "cognition_deep" => DEEP_OPS,
        "cognition_predict" => PREDICT_OPS,
        _ => return None,
    };

    if ops.contains(&operation) {
        Some(format!("cognition_{}", operation))
    } else {
        None
    }
}

/// Normalize a tool call: if compact, resolve to canonical name + inner args.
/// If already a canonical tool name, pass through unchanged.
pub fn normalize_compact_tool_call(
    tool_name: &str,
    args: Value,
) -> Result<(String, Value), String> {
    // Check if this is a compact facade group name
    match tool_name {
        "cognition_model" | "cognition_deep" | "cognition_predict" => {
            let (operation, inner_args) = decode_compact_operation(args)?;
            let canonical = resolve_compact_tool(tool_name, &operation)
                .ok_or_else(|| {
                    format!(
                        "Unknown operation '{}' for group '{}'",
                        operation, tool_name
                    )
                })?;
            Ok((canonical, inner_args))
        }
        // Not a compact name — pass through as-is
        _ => Ok((tool_name.to_string(), args)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_definitions_count() {
        assert_eq!(compact_tool_definitions().len(), 3);
    }

    #[test]
    fn resolve_model_ops() {
        assert_eq!(
            resolve_compact_tool("cognition_model", "model_create"),
            Some("cognition_model_create".into())
        );
        assert_eq!(
            resolve_compact_tool("cognition_model", "belief_add"),
            Some("cognition_belief_add".into())
        );
        assert_eq!(
            resolve_compact_tool("cognition_model", "bogus"),
            None
        );
    }

    #[test]
    fn resolve_deep_ops() {
        assert_eq!(
            resolve_compact_tool("cognition_deep", "soul_reflect"),
            Some("cognition_soul_reflect".into())
        );
        assert_eq!(
            resolve_compact_tool("cognition_deep", "drift_track"),
            Some("cognition_drift_track".into())
        );
    }

    #[test]
    fn resolve_predict_ops() {
        assert_eq!(
            resolve_compact_tool("cognition_predict", "predict"),
            Some("cognition_predict".into())
        );
        assert_eq!(
            resolve_compact_tool("cognition_predict", "simulate"),
            Some("cognition_simulate".into())
        );
    }

    #[test]
    fn normalize_compact_call() {
        let args = serde_json::json!({
            "operation": "model_vitals",
            "args": { "model_id": "abc-123" }
        });
        let (name, inner) = normalize_compact_tool_call("cognition_model", args).unwrap();
        assert_eq!(name, "cognition_model_vitals");
        assert_eq!(inner["model_id"], "abc-123");
    }

    #[test]
    fn normalize_passthrough() {
        let args = serde_json::json!({ "model_id": "abc" });
        let (name, inner) =
            normalize_compact_tool_call("cognition_model_vitals", args.clone()).unwrap();
        assert_eq!(name, "cognition_model_vitals");
        assert_eq!(inner, args);
    }

    #[test]
    fn decode_missing_operation() {
        let args = serde_json::json!({ "args": {} });
        assert!(decode_compact_operation(args).is_err());
    }

    #[test]
    fn decode_missing_args_defaults_empty() {
        let args = serde_json::json!({ "operation": "predict" });
        let (op, inner) = decode_compact_operation(args).unwrap();
        assert_eq!(op, "predict");
        assert_eq!(inner, serde_json::json!({}));
    }

    #[test]
    fn compact_mode_off_by_default() {
        // Unless env var is set, compact mode is off
        assert!(!mcp_tool_surface_is_compact());
    }
}
