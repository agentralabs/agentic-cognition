# AgenticCognition CLI Reference

## Installation

```bash
cargo install agentic-cognition-cli
```

## Global Options

| Option | Default | Description |
|--------|---------|-------------|
| `--storage` | `~/.agentic/cognition` | Storage directory |

## Commands

### Model Operations (9 commands)

| Command | Description |
|---------|-------------|
| `acog model create` | Create a new user model |
| `acog model show <ID>` | Show model details |
| `acog model vitals <ID>` | Show model health |
| `acog model heartbeat <ID> --observations <OBS>` | Pulse model |
| `acog model portrait <ID>` | Full model portrait |
| `acog model soul <ID>` | Soul reflection |
| `acog model consciousness <ID>` | Consciousness state |
| `acog model list` | List all models |
| `acog model delete <ID>` | Delete a model |

### Belief Operations (12 commands)

| Command | Description |
|---------|-------------|
| `acog belief add <ID> <CONTENT> --domain <D> --confidence <C>` | Add belief |
| `acog belief show <ID> <BID>` | Show belief |
| `acog belief list <ID>` | List beliefs |
| `acog belief strengthen <ID> <BID> --amount <A>` | Strengthen |
| `acog belief weaken <ID> <BID> --amount <A>` | Weaken |
| `acog belief connect <ID> <FROM> <TO> --kind <K>` | Connect |
| `acog belief graph <ID>` | Show graph |
| `acog belief keystones <ID>` | Show keystones |
| `acog belief contradictions <ID>` | Show contradictions |
| `acog belief crystallize <ID> <BID>` | Crystallize |
| `acog belief collapse <ID> <BID>` | Collapse |
| `acog belief search <ID> <QUERY>` | Search |

### Self-Concept (6), Pattern (3), Shadow (3), Bias (2), Drift (2), Predict (3)

See `acog <subcommand> --help` for full details.

## Domains

Valid belief domains: `self`, `relationships`, `work`, `politics`, `religion`, `science`, `values`, `world_model`, `identity`, `capability`, `worth`, `other`

## Connection Types

`supports`, `contradicts`, `requires`, `implies`, `associated`, `generalizes`, `specializes`
