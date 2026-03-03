---
status: stable
---

# CLI Reference

The `acog` binary provides 40+ commands organized into 8 groups for interacting with AgenticCognition from the terminal.

## Global Options

All commands support the following global options:

| Option | Short | Description | Default |
|:---|:---|:---|:---|
| `--format` | `-f` | Output format: `json`, `table`, `text` | `text` |
| `--storage` | `-s` | Path to `.acog` storage directory | `$ACOG_STORAGE` or `~/.acog` |
| `--verbose` | `-v` | Enable verbose output | off |
| `--help` | `-h` | Show help for any command | -- |
| `--version` | `-V` | Print version information | -- |

## model (9 commands)

Manage living user models and their lifecycle.

| Command | Description |
|:---|:---|
| `model create` | Create a new living user model |
| `model show <id>` | Display model details and current lifecycle stage |
| `model vitals <id>` | Show model health metrics and activity summary |
| `model heartbeat <id>` | Record an interaction heartbeat to advance the model |
| `model portrait <id>` | Generate a natural-language portrait of the user |
| `model soul <id>` | Produce a deep soul reflection across all dimensions |
| `model consciousness <id>` | Display the consciousness map with active regions |
| `model list` | List all models in the storage directory |
| `model delete <id>` | Delete a model and its associated data |

Example usage:

```bash
# Create a model and capture the ID
MODEL_ID=$(acog model create --format json | jq -r '.id')

# Record a heartbeat with context
acog model heartbeat $MODEL_ID --context "discussing career change"

# Generate a soul reflection
acog model soul $MODEL_ID --format json
```

## belief (12 commands)

Manage beliefs within a living user model. Beliefs have physical properties: confidence, crystallization, entanglement, and conviction gravity.

| Command | Description |
|:---|:---|
| `belief add <model> <text>` | Add a new belief to the model |
| `belief show <model> <id>` | Display a single belief with all properties |
| `belief list <model>` | List all beliefs, optionally filtered by domain |
| `belief strengthen <model> <id>` | Increase belief confidence with evidence |
| `belief weaken <model> <id>` | Decrease belief confidence with counter-evidence |
| `belief connect <model> <a> <b>` | Create an entanglement link between two beliefs |
| `belief graph <model>` | Visualize the full belief graph |
| `belief keystones <model>` | Identify keystone beliefs that anchor the system |
| `belief contradictions <model>` | Detect contradictory belief pairs |
| `belief crystallize <model> <id>` | Force-crystallize a belief (make it resistant to change) |
| `belief collapse <model> <id>` | Trigger belief collapse when contradictions are irreconcilable |
| `belief search <model> <query>` | Search beliefs by text content or domain |

Example usage:

```bash
# Add beliefs with domain and confidence
acog belief add $MODEL_ID "I value deep work over meetings" --domain work --confidence 0.85
acog belief add $MODEL_ID "Collaboration is essential for innovation" --domain work --confidence 0.7

# Connect related beliefs
acog belief connect $MODEL_ID $BELIEF_A $BELIEF_B --type tension

# Find keystones
acog belief keystones $MODEL_ID --format table

# Detect contradictions
acog belief contradictions $MODEL_ID
```

## self (6 commands)

Explore the self-concept topology -- the landscape of identity.

| Command | Description |
|:---|:---|
| `self topology <model>` | Generate the full self-concept topology map |
| `self peaks <model>` | Identify identity peaks (strongest self-beliefs) |
| `self valleys <model>` | Identify identity valleys (weakest self-beliefs) |
| `self blindspots <model>` | Detect gaps between self-perception and evidence |
| `self defended <model>` | Find defended territories resistant to change |
| `self edges <model>` | Map identity edges and boundaries |

Example usage:

```bash
# Generate full topology
acog self topology $MODEL_ID --format json

# Find defended territories that resist change
acog self defended $MODEL_ID
```

## pattern (3 commands)

Analyze behavioral patterns and decision fingerprints.

| Command | Description |
|:---|:---|
| `pattern fingerprint <model>` | Generate a decision-making fingerprint |
| `pattern fossils <model>` | Excavate behavioral fossils from past interactions |
| `pattern strata <model>` | Display archaeological strata showing identity layers |

Example usage:

```bash
# Generate decision fingerprint
acog pattern fingerprint $MODEL_ID

# View archaeological layers of identity
acog pattern strata $MODEL_ID --depth 5
```

## shadow (3 commands)

Map the shadow -- unconscious patterns operating below awareness.

| Command | Description |
|:---|:---|
| `shadow map <model>` | Generate the complete shadow map |
| `shadow projections <model>` | Detect projection patterns |
| `shadow blindspots <model>` | Identify shadow blindspots |

Example usage:

```bash
# Generate shadow map
acog shadow map $MODEL_ID --format json

# Detect projections (attributes ascribed to others that belong to self)
acog shadow projections $MODEL_ID
```

## bias (2 commands)

Detect and map cognitive bias fields.

| Command | Description |
|:---|:---|
| `bias field <model>` | Map the active bias field |
| `bias triggers <model>` | Identify emotional triggers that activate biases |

Example usage:

```bash
acog bias field $MODEL_ID --format table
acog bias triggers $MODEL_ID
```

## drift (2 commands)

Track longitudinal changes in beliefs, values, and identity.

| Command | Description |
|:---|:---|
| `drift timeline <model>` | Show drift timeline with change events |
| `drift tectonics <model>` | Display value tectonic movements over time |

Example usage:

```bash
# Show how values have shifted
acog drift timeline $MODEL_ID --range 90d

# Track slow tectonic movements of core values
acog drift tectonics $MODEL_ID
```

## predict (3 commands)

Use the prediction engine to anticipate preferences, simulate decisions, and project identity trajectories.

| Command | Description |
|:---|:---|
| `predict preference <model> <query>` | Predict user preference for a given option |
| `predict decision <model> <scenario>` | Simulate a decision with options |
| `predict future <model>` | Project identity trajectory into the future |

Example usage:

```bash
# Predict preference
acog predict preference $MODEL_ID "Should I take the remote job or the office role?"

# Simulate a decision
acog predict decision $MODEL_ID "career change" --options "stay,switch,freelance"

# Project future trajectory
acog predict future $MODEL_ID --horizon 6m
```
