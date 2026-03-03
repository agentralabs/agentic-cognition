//! MCP integration example — how to configure for Claude Desktop

fn main() {
    let config = r#"
Add this to your Claude Desktop config (~/.claude/claude_desktop_config.json):

{
  "mcpServers": {
    "cognition": {
      "command": "acog-mcp",
      "args": ["--storage", "~/.agentic/cognition"]
    }
  }
}

Then ask Claude:
- "Create a user model for me"
- "Add a belief: I value continuous learning"
- "What are my keystone beliefs?"
- "Predict how I'd decide on accepting a new job"
- "Perform a soul reflection"
"#;
    println!("{config}");
}
