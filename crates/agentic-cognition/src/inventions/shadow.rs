//! Inventions 9-13: Shadow Psychology — detection of unconscious patterns

use crate::types::*;

/// Shadow pattern detector
pub struct ShadowDetector;

impl ShadowDetector {
    /// Detect potential shadow beliefs by finding behavioral contradictions
    pub fn detect_shadow_beliefs(graph: &BeliefGraph, shadow_map: &ShadowMap) -> Vec<ShadowBelief> {
        let mut detected = Vec::new();
        let now = Timestamp::now();

        // Look for pairs of beliefs that contradict in practice
        for conn in &graph.connections {
            if conn.connection_type == ConnectionType::Contradicts && conn.strength > 0.5 {
                if let (Some(a), Some(b)) =
                    (graph.beliefs.get(&conn.from), graph.beliefs.get(&conn.to))
                {
                    // If one belief is explicit and the other implicit, the implicit one is shadow
                    if a.explicit
                        && !b.explicit
                        && !shadow_map.shadow_beliefs.iter().any(|s| s.id == b.id)
                    {
                        detected.push(ShadowBelief {
                            id: b.id,
                            content: format!("Shadow of: {}", b.content),
                            evidence: vec![ShadowEvidence {
                                observation: format!("Contradicts explicit belief: {}", a.content),
                                timestamp: now,
                                weight: conn.strength,
                            }],
                            strength: conn.strength * 0.7,
                            contradicts_conscious: Some(a.id),
                            behavioral_signs: vec![format!(
                                "Behavior contradicts stated belief: {}",
                                a.content
                            )],
                            detected_at: now,
                        });
                    }
                }
            }
        }

        detected
    }

    /// Detect projections — strong reactions to traits in others
    pub fn detect_projections(_soul: &ModelSoul, bias_field: &BiasField) -> Vec<Projection> {
        let mut projections = Vec::new();
        let now = Timestamp::now();

        // Look for triggers that match disowned traits
        for trigger in &bias_field.triggers {
            if trigger.intensity > 0.7 {
                // Strong reaction suggests projection
                projections.push(Projection {
                    id: ProjectionId::new(),
                    disowned_trait: trigger.trigger.clone(),
                    projected_onto: "others (pattern detected)".to_string(),
                    strength: trigger.intensity * 0.6,
                    evidence: vec![format!(
                        "Strong emotional reaction: {}",
                        trigger.response_pattern
                    )],
                    original_self_trait: trigger.trigger.clone(),
                    detected_at: now,
                });
            }
        }

        projections
    }

    /// Map blindspot topology — areas of systematic self-blindness
    pub fn map_blindspot_topology(
        self_concept: &SelfConceptTopology,
        soul: &ModelSoul,
    ) -> Vec<Blindspot> {
        let mut blindspots = Vec::new();
        let now = Timestamp::now();

        // Unwarranted confidence peaks indicate blindspots
        for peak in &self_concept.peaks {
            if !peak.warranted {
                if let Some(gap) = peak.reality_gap {
                    if gap > 0.3 {
                        blindspots.push(Blindspot {
                            id: BlindspotId::new(),
                            area: format!("Overconfidence in: {}", peak.domain),
                            blindness_level: gap,
                            evidence: vec![format!(
                                "Reality gap of {:.0}% in {}",
                                gap * 100.0,
                                peak.domain
                            )],
                            impact: format!("May lead to poor decisions in {}", peak.domain),
                            penetrability: 1.0 - gap,
                            detected_at: now,
                        });
                    }
                }
            }
        }

        // Authenticity gap indicates self-blindness
        if soul.authenticity_gap > 0.3 {
            blindspots.push(Blindspot {
                id: BlindspotId::new(),
                area: "Gap between presented and authentic self".to_string(),
                blindness_level: soul.authenticity_gap,
                evidence: vec![format!(
                    "Authenticity gap: {:.0}%",
                    soul.authenticity_gap * 100.0
                )],
                impact: "Relationships may be affected by inauthenticity".to_string(),
                penetrability: 0.5,
                detected_at: now,
            });
        }

        blindspots
    }
}
