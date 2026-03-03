---
status: stable
---

# MCP Prompts

AgenticCognition provides MCP prompt definitions that guide AI agents through common cognition workflows. Prompts are exposed via the `prompts/list` and `prompts/get` MCP methods.

## Available Prompts

### create-model

Guides the agent through creating a new living user model with initial context.

**Arguments:**

| Argument | Type | Required | Description |
|:---|:---|:---|:---|
| `name` | string | no | Optional name for the model |

**Prompt messages:**

```json
{
  "name": "create-model",
  "description": "Guide the creation of a new living user model",
  "arguments": [
    {"name": "name", "description": "Optional name for the model", "required": false}
  ]
}
```

When invoked, the prompt returns a series of messages that instruct the agent to create a model, ask the user introductory questions, and add initial beliefs based on responses.

### belief-analysis

Guides the agent through analyzing a user's belief system by querying the belief graph, identifying keystones, and surfacing contradictions.

**Arguments:**

| Argument | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model to analyze |
| `domain` | string | no | Focus on a specific domain |

**Prompt messages:**

The prompt instructs the agent to retrieve the belief graph, identify the five strongest beliefs, check for contradictions, and present findings to the user with questions for deeper exploration.

### soul-reflection

Guides the agent through a deep soul reflection session that combines belief analysis, shadow mapping, and drift tracking.

**Arguments:**

| Argument | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model to reflect on |
| `focus` | string | no | Optional focus area |

**Prompt messages:**

The prompt instructs the agent to run soul reflection, shadow mapping, and drift tracking in sequence, then synthesize the results into a cohesive narrative about the user's current psychological landscape.

### shadow-exploration

Guides the agent through exploring the user's shadow with sensitivity and care.

**Arguments:**

| Argument | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model to explore |

**Prompt messages:**

The prompt provides instructions for gentle shadow exploration, emphasizing ethical guidelines: always present shadow insights as possibilities, never as diagnoses; use tentative language; and always give the user agency over whether to engage with shadow material.

### decision-support

Guides the agent through using the prediction engine to help a user make a decision.

**Arguments:**

| Argument | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model to use |
| `scenario` | string | yes | Decision scenario |

**Prompt messages:**

The prompt instructs the agent to simulate the decision, present alignment analysis for each option, and help the user understand how each choice relates to their core values and belief system.

## Using Prompts

MCP clients list available prompts:

```json
{
  "jsonrpc": "2.0",
  "method": "prompts/list",
  "id": 1
}
```

And retrieve a specific prompt with arguments:

```json
{
  "jsonrpc": "2.0",
  "method": "prompts/get",
  "params": {
    "name": "soul-reflection",
    "arguments": {
      "model_id": "abc-123",
      "focus": "growth"
    }
  },
  "id": 2
}
```

The response contains the prompt messages that the client should use to guide the interaction.
