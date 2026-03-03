//! MCP tool count verification test

use agentic_cognition_mcp::tools::ToolRegistry;

#[test]
fn mcp_tool_count() {
    let tools = ToolRegistry::all_tools();
    assert_eq!(tools.len(), 14, "MCP tool count must be exactly 14");
    assert_eq!(ToolRegistry::tool_count(), 14);
}

#[test]
fn mcp_tool_names_unique() {
    let tools = ToolRegistry::all_tools();
    let mut names: Vec<String> = tools.iter().map(|t| t.name.clone()).collect();
    let original_len = names.len();
    names.sort();
    names.dedup();
    assert_eq!(names.len(), original_len, "Tool names must be unique");
}

#[test]
fn mcp_tool_descriptions_valid() {
    let tools = ToolRegistry::all_tools();
    for tool in &tools {
        // Descriptions must not be empty
        assert!(
            !tool.description.is_empty(),
            "Tool {} has empty description",
            tool.name
        );
        // Descriptions must not end with period (MCP Quality Standard)
        assert!(
            !tool.description.ends_with('.'),
            "Tool {} description ends with period",
            tool.name
        );
        // Descriptions must start with a capital letter (verb-first imperative)
        let first_char = tool.description.chars().next().unwrap();
        assert!(
            first_char.is_uppercase(),
            "Tool {} description must start with uppercase",
            tool.name
        );
    }
}

#[test]
fn mcp_tool_schemas_valid() {
    let tools = ToolRegistry::all_tools();
    for tool in &tools {
        // All schemas must be objects
        assert!(
            tool.input_schema.is_object(),
            "Tool {} schema must be an object",
            tool.name
        );
        // All schemas must have a "type" field
        assert_eq!(
            tool.input_schema.get("type").and_then(|v| v.as_str()),
            Some("object"),
            "Tool {} schema type must be 'object'",
            tool.name
        );
    }
}
