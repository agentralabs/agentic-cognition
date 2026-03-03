---
status: stable
---

# MCP Resources

AgenticCognition exposes MCP resources for reading model data without invoking tools. Resources provide a read-only view of the living user model state.

## Resource URI Scheme

All resources use the `acog://` URI scheme:

```
acog://{model_id}/{resource_type}
```

## Available Resources

### acog://{model_id}/vitals

Returns the model vitals as a JSON object, including lifecycle stage, belief count, last heartbeat, interaction count, and health score.

```json
{
  "uri": "acog://abc-123/vitals",
  "mimeType": "application/json",
  "name": "Model vitals for abc-123"
}
```

### acog://{model_id}/beliefs

Returns the full list of beliefs in the model as a JSON array. Each belief includes its ID, text, domain, confidence, crystallization level, and timestamps.

```json
{
  "uri": "acog://abc-123/beliefs",
  "mimeType": "application/json",
  "name": "Belief list for abc-123"
}
```

### acog://{model_id}/belief-graph

Returns the belief graph as a JSON object with `nodes` and `edges` arrays. Nodes are beliefs; edges are entanglements, tensions, and support relationships.

```json
{
  "uri": "acog://abc-123/belief-graph",
  "mimeType": "application/json",
  "name": "Belief graph for abc-123"
}
```

### acog://{model_id}/portrait

Returns the most recently generated portrait as a text string. If no portrait has been generated, the resource returns an empty body.

```json
{
  "uri": "acog://abc-123/portrait",
  "mimeType": "text/plain",
  "name": "Portrait for abc-123"
}
```

### acog://{model_id}/shadow

Returns the shadow map as a JSON object including shadow beliefs, projections, blindspots, defended regions, and bias fields.

```json
{
  "uri": "acog://abc-123/shadow",
  "mimeType": "application/json",
  "name": "Shadow map for abc-123"
}
```

### acog://{model_id}/drift

Returns the drift timeline as a JSON array of drift events, each containing a timestamp, description, affected domain, and magnitude.

```json
{
  "uri": "acog://abc-123/drift",
  "mimeType": "application/json",
  "name": "Drift timeline for abc-123"
}
```

## Resource Templates

The server also exposes a resource template for dynamic model discovery:

```json
{
  "uriTemplate": "acog://{model_id}/vitals",
  "name": "Model vitals",
  "description": "Retrieve vitals for any model by ID"
}
```

## Usage from MCP Clients

MCP clients can list available resources via the `resources/list` method and read individual resources via `resources/read`:

```json
{
  "jsonrpc": "2.0",
  "method": "resources/read",
  "params": {
    "uri": "acog://abc-123/beliefs"
  },
  "id": 1
}
```

The response contains the resource contents:

```json
{
  "jsonrpc": "2.0",
  "result": {
    "contents": [
      {
        "uri": "acog://abc-123/beliefs",
        "mimeType": "application/json",
        "text": "[{\"id\": \"bel-001\", \"text\": \"I value honesty\", ...}]"
      }
    ]
  },
  "id": 1
}
```
