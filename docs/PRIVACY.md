# AgenticCognition Privacy & Ethics Guide

## Core Principles

### 1. Consent Is Continuous
User modeling requires explicit consent. Consent can be:
- `Pending` (no modeling until granted)
- `Granted` (full modeling)
- `Limited` (only specified domains)
- `Revoked` (all modeling stops, data retained per retention policy)

### 2. Transparency
Users can view all inferences, beliefs, and predictions made about them via `acog model portrait`.

### 3. Data Ownership
Users own their `.acog` files completely. Files are stored locally, never transmitted without explicit action.

### 4. Prediction Humility
All predictions include confidence scores. The model acknowledges uncertainty.

### 5. No Manipulation
Cognition data is for user empowerment, never persuasion engineering or behavioral manipulation.

## Privacy Settings

```rust
PrivacySettings {
    allow_shadow_detection: bool,  // Can the model detect unconscious patterns?
    allow_prediction: bool,        // Can the model predict behavior?
    allow_drift_tracking: bool,    // Can the model track change over time?
    excluded_domains: Vec<String>, // Domains excluded from modeling
    retention_days: u32,           // Data retention (0 = forever)
}
```

## Security

- `.acog` files use blake3 checksums for integrity verification
- Atomic writes prevent corruption
- Per-project isolation prevents cross-contamination
- No network access in core library

## Ethical Guidelines

1. Never use cognition data to manipulate users
2. Always present uncertainty alongside predictions
3. Respect domain exclusions absolutely
4. Support user's right to delete their model at any time
5. Shadow detection is opt-in and presented sensitively
