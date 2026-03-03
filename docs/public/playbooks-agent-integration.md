---
status: stable
---

# Playbooks: Agent Integration

This document provides step-by-step playbooks for integrating AgenticCognition into agent workflows. Each playbook includes setup instructions and usage patterns.

## Playbook 1: Personal Assistant Agent

Connect cognition to a personal assistant to provide responses calibrated to the user's belief system, decision patterns, and growth trajectory.

**Setup:**

1. Install AgenticCognition and configure the MCP server for your client
2. Create a living user model: `acog model create`
3. Add initial beliefs based on onboarding conversation

**Usage pattern:**

- On each interaction, call `cognition_model_heartbeat` with the conversation topic
- Before giving advice, call `cognition_predict` to check alignment with user values
- Periodically call `cognition_model_portrait` to refresh the assistant's understanding
- When the user faces a choice, call `cognition_simulate` to evaluate options

## Playbook 2: Coaching Agent

Use shadow psychology and drift tracking to build a coaching agent that surfaces blindspots, tracks growth over time, and provides personalized guidance based on self-concept topology.

**Setup:**

1. Create a dedicated coaching model with `acog model create --name coaching`
2. Add beliefs across multiple domains: values, relationships, work, growth
3. Build initial belief graph with entanglement links

**Usage pattern:**

- Call `cognition_shadow_map` monthly to check for new projections or blindspots
- Use `cognition_drift_track` to show the user how their values have shifted
- Call `cognition_self_topology` to identify defended regions that may limit growth
- Present shadow insights tentatively, giving the user agency over engagement
- Track growth rings to celebrate progress over time

## Playbook 3: Decision Support Agent

Combine decision simulation with belief graph analysis to help users make better decisions.

**Setup:**

1. Ensure the model has beliefs in the relevant decision domain
2. Build entanglement links between related beliefs (support, tension, opposition)
3. Establish keystone beliefs that anchor the domain

**Usage pattern:**

- When the user presents a decision, call `cognition_simulate` with the scenario and options
- Display per-option alignment scores showing how each choice relates to core values
- Call `cognition_belief_graph` to visualize which beliefs support or oppose each option
- Use `cognition_pattern_fingerprint` to show the user their decision-making tendencies
- After the decision is made, record the outcome to refine future predictions

## Playbook 4: Multi-Agent Ecosystem

In a multi-agent setup, AgenticCognition serves as the shared understanding layer.

**Setup:**

1. Create a single model that all agents share via the same `.acog` file
2. Configure each agent's MCP server to point to the same storage directory
3. Use file locking to prevent concurrent write conflicts

**Usage pattern:**

- Each agent calls `cognition_model_heartbeat` to record its interactions
- Agents call `cognition_predict` before making decisions on behalf of the user
- The coordination agent calls `cognition_model_vitals` to monitor model health
- All agents benefit from the accumulated understanding of every other agent
- Use `cognition_soul_reflect` as a shared context briefing for new agents

## Playbook 5: Team Context Agent

Use per-project `.acog` files to maintain separate user models for different work contexts.

**Setup:**

1. Set `ACOG_STORAGE` to a project-local directory: `export ACOG_STORAGE=./.acog`
2. Create a project-specific model: `acog model create --name "project-x"`
3. Add beliefs relevant to the project context

**Usage pattern:**

- The agent automatically loads the correct model based on the active project directory
- Work-domain beliefs are kept separate from personal beliefs
- When switching projects, the agent's understanding switches with it
- Use `cognition_drift_track` to monitor how the user's relationship with a project evolves
- Archive completed project models for future reference
