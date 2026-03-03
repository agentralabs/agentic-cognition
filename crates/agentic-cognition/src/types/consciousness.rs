//! Consciousness state types

use crate::types::ids::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessState {
    pub emotional_weather: EmotionalWeather,
    pub cognitive_load: f64,
    pub attention_focus: Vec<AttentionFocus>,
    pub life_phase: LifePhase,
    pub active_tensions: Vec<Tension>,
    pub energy_level: f64,
    pub updated_at: Timestamp,
}

impl Default for ConsciousnessState {
    fn default() -> Self {
        Self {
            emotional_weather: EmotionalWeather::default(),
            cognitive_load: 0.0,
            attention_focus: Vec::new(),
            life_phase: LifePhase::Exploring,
            active_tensions: Vec::new(),
            energy_level: 0.5,
            updated_at: Timestamp::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalWeather {
    pub baseline_mood: Mood,
    pub current_mood: Mood,
    pub volatility: f64,
    pub dominant_emotion: Option<Emotion>,
    pub pattern: WeatherPattern,
}

impl Default for EmotionalWeather {
    fn default() -> Self {
        Self {
            baseline_mood: Mood::Neutral,
            current_mood: Mood::Neutral,
            volatility: 0.0,
            dominant_emotion: None,
            pattern: WeatherPattern::Stable,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mood {
    Elevated,
    Positive,
    Neutral,
    Subdued,
    Negative,
    Depressed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Emotion {
    Joy,
    Sadness,
    Anger,
    Fear,
    Surprise,
    Disgust,
    Trust,
    Anticipation,
    Love,
    Guilt,
    Shame,
    Pride,
    Envy,
    Gratitude,
    Hope,
    Anxiety,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeatherPattern {
    Stable,
    Stormy,
    Shifting,
    Clearing,
    Darkening,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifePhase {
    Exploring,
    Building,
    Maintaining,
    Transitioning,
    Crisis,
    Recovery,
    Thriving,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionFocus {
    pub domain: String,
    pub intensity: f64,
    pub since: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tension {
    pub description: String,
    pub between: (String, String),
    pub severity: f64,
    pub since: Timestamp,
}
