---
status: stable
---

# Frequently Asked Questions

## What is a .acog file?

A `.acog` file is a custom binary format with BLAKE3 integrity protection that holds the entire living user model. One file contains beliefs, shadows, drift history, decision fingerprints, and archaeological layers. See [File Format](file-format.md) for the binary layout and integrity guarantees.

## Does AgenticCognition require a cloud connection?

No. Everything runs locally. No telemetry, no cloud sync by default. The user owns their data completely. The architecture enforces this -- there is no networking code in the core library.

## Which MCP clients are supported?

Any MCP-compatible client works: Claude Desktop, Cursor, Windsurf, VS Code, Cody, Claude Code, and any future client supporting the Model Context Protocol.

## Can I use AgenticCognition without the MCP server?

Yes. The CLI (`acog`) and the Rust library provide full access to all 24 capabilities. The MCP server is one of several access surfaces.

## How does shadow detection work?

Shadow beliefs are inferred from behavioral patterns that contradict stated beliefs. The system detects projections (attributes ascribed to others that belong to self), blindspots (gaps between self-assessment and observed behavior), and defended regions. Shadow detection improves in accuracy as the model accumulates more heartbeats and observations.

## Is my data private?

Absolutely. See the Privacy and Ethics section in the README. The five privacy principles guarantee consent, transparency, humility, growth orientation, and data sacredness.

## How large does a .acog file get?

A year of intensive daily modeling produces approximately 200 KB (around 1,000 beliefs). A decade produces approximately 2 MB. See [Benchmarks](benchmarks.md) for detailed capacity metrics.

## Can I use multiple models for different contexts?

Yes. Each `.acog` file is an independent model. You can maintain separate models for different contexts (work, personal, project-specific) by using the `--storage` flag or the `ACOG_STORAGE` environment variable to point to different directories.

## What happens if my .acog file gets corrupted?

The BLAKE3 integrity check detects corruption on every read. If corruption is detected, the system reports an integrity error. Atomic writes (temp-file-plus-rename) prevent partial write corruption. See [Troubleshooting](troubleshooting.md) for recovery steps.

## How does AgenticCognition differ from user profiles or preference databases?

User profiles store static key-value pairs. AgenticCognition models a living, evolving belief system with physics (crystallization, entanglement, gravity, collapse), shadow psychology, drift tracking, and predictive capabilities. It understands not just what you prefer, but why, and how those preferences are changing.

## Can other Agentra sisters access my cognitive model?

Only through typed bridge traits with NoOp defaults. No sister can access the model without explicit integration. When connected, sisters enhance capability (e.g., Memory provides historical context, Identity provides signed verification) but are never required.

## What Rust version is required to build from source?

Rust 1.80.0 or later. The project uses stable Rust with no nightly features.

## How do I update AgenticCognition?

Run the installer again. It is safe to re-run at any time. The update process replaces the binary, merges MCP client configurations (never destructive), and preserves all existing `.acog` files and user data.
