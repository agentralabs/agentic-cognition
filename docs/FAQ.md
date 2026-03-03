# Frequently Asked Questions

## What is AgenticCognition?

AgenticCognition is a longitudinal user modeling system that creates living models of human consciousness for AI agents.

## How is this different from a user profile?

Profiles are static snapshots. AgenticCognition models are alive -- they breathe (heartbeat), evolve (lifecycle), and can predict behavior.

## Does it require other Agentra sisters?

No. All sister bridges default to NoOp. AgenticCognition works standalone.

## Where is data stored?

In `.acog` files in `~/.agentic/cognition/` by default. Override with `--storage`.

## Is my data secure?

Yes. Files use blake3 checksums, atomic writes prevent corruption, and data stays local.

## What are the 14 MCP tools?

See [MCP-TOOLS.md](MCP-TOOLS.md) for the complete reference.

## How accurate are predictions?

Predictions include confidence scores. Accuracy improves with more observations.
