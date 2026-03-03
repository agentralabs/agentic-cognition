# AgenticCognition Examples

## Example 1: Building a User Model

```bash
# Create model
acog model create
# { "model_id": "abc123...", "status": "created" }

# Add beliefs across domains
acog belief add abc123 "I thrive under pressure" --domain self --confidence 0.8
acog belief add abc123 "Teamwork produces better results" --domain work --confidence 0.7
acog belief add abc123 "Privacy is fundamental" --domain values --confidence 0.95

# Connect beliefs
acog belief connect abc123 <BELIEF1> <BELIEF2> --kind supports --strength 0.6

# Check vitals
acog model vitals abc123
```

## Example 2: Detecting Patterns

```bash
# After several interactions, check fingerprint
acog pattern fingerprint abc123

# Look for shadow beliefs
acog shadow map abc123

# Check for biases
acog bias field abc123
```

## Example 3: Predictions

```bash
# Predict preference
acog predict preference abc123 "startup vs corporate job"

# Simulate decision
acog predict decision abc123 "Accept promotion?" --options "Accept" --options "Decline" --options "Negotiate"

# Project future
acog predict future abc123 --days 180
```

## Example 4: MCP Tool Usage

```json
{"name": "cognition_model_create", "arguments": {}}

{"name": "cognition_belief_add", "arguments": {
  "model_id": "abc123",
  "content": "I value continuous learning",
  "domain": "values",
  "confidence": 0.85
}}

{"name": "cognition_soul_reflect", "arguments": {"model_id": "abc123"}}
```
