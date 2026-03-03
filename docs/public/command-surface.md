---
status: stable
---

# Command Surface

AgenticCognition exposes its functionality through 14 MCP tools and 40+ CLI commands.

## Compact Facade Tools

AgenticCognition uses focused MCP tools (not operation-based routing). Each tool has a single purpose with a clear parameter schema.

### Model Tools

- `cognition_model_create` -- Create a new living user model. Accepts optional `name` and `context` parameters. Returns the model ID and initial lifecycle stage.
- `cognition_model_heartbeat` -- Record an interaction heartbeat to advance the model. Requires `model_id`, accepts optional `context` and `observations`.
- `cognition_model_vitals` -- Retrieve health metrics including lifecycle stage, belief count, last heartbeat, and health score. Requires `model_id`.
- `cognition_model_portrait` -- Generate a natural-language portrait of the user. Requires `model_id`, accepts optional `depth` (brief, standard, deep).

### Belief Tools

- `cognition_belief_add` -- Add a new belief with text, domain, confidence, and evidence. Requires `model_id` and `text`.
- `cognition_belief_query` -- Query beliefs by text, domain, or minimum confidence. Requires `model_id`.
- `cognition_belief_graph` -- Retrieve the full belief graph with entanglements. Requires `model_id`, accepts optional `depth` and `center`.

### Reflection Tools

- `cognition_soul_reflect` -- Produce a deep soul reflection across all model dimensions. Requires `model_id`, accepts optional `focus`.
- `cognition_self_topology` -- Generate the self-concept topology showing peaks, valleys, edges, and defended territories. Requires `model_id`.

### Analysis Tools

- `cognition_pattern_fingerprint` -- Generate a decision-making fingerprint from behavioral patterns. Requires `model_id`, accepts optional `domain`.
- `cognition_shadow_map` -- Generate the shadow map with projections, blindspots, and defended regions. Requires `model_id`.
- `cognition_drift_track` -- Track longitudinal drift in beliefs and values. Requires `model_id`, accepts optional `range` and `domain`.

### Prediction Tools

- `cognition_predict` -- Predict user preferences. Requires `model_id` and `query`.
- `cognition_simulate` -- Simulate a decision scenario with multiple options. Requires `model_id` and `scenario`, accepts optional `options`.

## CLI Commands

AgenticCognition provides 40+ CLI commands through the `acog` binary, organized into eight groups.

### model (9 commands)

`create`, `show`, `vitals`, `heartbeat`, `portrait`, `soul`, `consciousness`, `list`, `delete`

Manage living user models and their lifecycle. The `create` command generates a new model. The `heartbeat` command records interaction context. The `soul` command produces a deep multi-dimensional reflection.

### belief (12 commands)

`add`, `show`, `list`, `strengthen`, `weaken`, `connect`, `graph`, `keystones`, `contradictions`, `crystallize`, `collapse`, `search`

Manage the belief graph. Beliefs can be added, strengthened, weakened, and connected. The `keystones` command identifies beliefs that anchor the system. The `contradictions` command detects conflicting belief pairs.

### self (6 commands)

`topology`, `peaks`, `valleys`, `blindspots`, `defended`, `edges`

Explore the self-concept landscape. Identity peaks represent the strongest self-beliefs. Valleys represent areas of low confidence. Defended regions resist change.

### pattern (3 commands)

`fingerprint`, `fossils`, `strata`

Analyze behavioral patterns. The fingerprint captures decision-making style. Fossils are preserved behavioral artifacts. Strata show archaeological layers of identity.

### shadow (3 commands)

`map`, `projections`, `blindspots`

Map unconscious patterns. The shadow map shows projections (attributes ascribed to others that belong to self), blindspots (gaps in self-awareness), and defended territories.

### bias (2 commands)

`field`, `triggers`

Detect cognitive bias fields and emotional triggers that distort perception and decision-making.

### drift (2 commands)

`timeline`, `tectonics`

Track longitudinal changes. The timeline shows drift events over time. Tectonics reveals slow, deep movement of core value systems.

### predict (3 commands)

`preference`, `decision`, `future`

Use the prediction engine. Preference prediction answers specific questions. Decision simulation evaluates scenarios with options. Future projection estimates identity trajectory.

## Global Options

All commands support `--format json|table|text`, `--storage <path>`, and `--verbose` flags. See [CLI Reference](cli-reference.md) for complete details.
