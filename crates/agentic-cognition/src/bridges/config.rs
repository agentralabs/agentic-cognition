//! Bridge configuration and runtime registry.
//!
//! `BridgeConfig` specifies which sisters are available for bridging.
//! `BridgeRegistry` provides runtime discovery and connection management
//! for multi-sister deployments.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================
// BRIDGE CONFIG
// ============================================================

/// Configuration for bridge connectivity.
///
/// Specifies which sister bridges are enabled and how to reach them.
/// Used during initialization to wire up the BridgeSet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    /// Whether to enable bridges at all (false = fully standalone)
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Per-sister bridge configuration
    #[serde(default)]
    pub sisters: HashMap<String, SisterBridgeConfig>,

    /// Discovery mode: "static" (configured), "mcp" (via MCP), "hydra" (via orchestrator)
    #[serde(default = "default_discovery")]
    pub discovery: String,

    /// Maximum time to wait for a bridge response (ms)
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
}

fn default_true() -> bool {
    true
}

fn default_discovery() -> String {
    "static".to_string()
}

fn default_timeout() -> u64 {
    5000
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sisters: HashMap::new(),
            discovery: "static".to_string(),
            timeout_ms: 5000,
        }
    }
}

impl BridgeConfig {
    /// Create a config with all bridges disabled (standalone mode).
    pub fn standalone() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }

    /// Create a config that discovers bridges via MCP.
    pub fn mcp_discovery() -> Self {
        Self {
            discovery: "mcp".to_string(),
            ..Default::default()
        }
    }

    /// Add a sister bridge configuration.
    pub fn with_sister(mut self, name: impl Into<String>, config: SisterBridgeConfig) -> Self {
        self.sisters.insert(name.into(), config);
        self
    }

    /// Check if a specific sister bridge is configured.
    pub fn has_sister(&self, name: &str) -> bool {
        self.enabled && self.sisters.contains_key(name)
    }
}

/// Configuration for a single sister bridge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SisterBridgeConfig {
    /// Whether this specific bridge is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Connection endpoint (MCP socket, HTTP URL, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,

    /// Transport type: "in-process", "mcp-stdio", "mcp-sse", "http"
    #[serde(default = "default_transport")]
    pub transport: String,

    /// Sister-specific options
    #[serde(default)]
    pub options: HashMap<String, serde_json::Value>,
}

fn default_transport() -> String {
    "in-process".to_string()
}

impl Default for SisterBridgeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: None,
            transport: "in-process".to_string(),
            options: HashMap::new(),
        }
    }
}

impl SisterBridgeConfig {
    /// Create an in-process bridge config (shared memory, same process).
    pub fn in_process() -> Self {
        Self::default()
    }

    /// Create an MCP stdio bridge config.
    pub fn mcp_stdio(command: impl Into<String>) -> Self {
        Self {
            transport: "mcp-stdio".to_string(),
            endpoint: Some(command.into()),
            ..Default::default()
        }
    }

    /// Create an MCP SSE bridge config.
    pub fn mcp_sse(url: impl Into<String>) -> Self {
        Self {
            transport: "mcp-sse".to_string(),
            endpoint: Some(url.into()),
            ..Default::default()
        }
    }
}

// ============================================================
// BRIDGE REGISTRY
// ============================================================

/// Runtime registry of available sister bridges.
///
/// Tracks which bridges are connected, their health status,
/// and provides lookup for bridge operations.
#[derive(Debug, Clone)]
pub struct BridgeRegistry {
    /// Registered bridges
    entries: HashMap<String, BridgeEntry>,
}

/// A registered bridge entry.
#[derive(Debug, Clone)]
pub struct BridgeEntry {
    /// Sister name
    pub name: String,
    /// Whether the bridge is currently connected
    pub connected: bool,
    /// Last successful ping timestamp (nanos since epoch)
    pub last_ping: Option<i64>,
    /// Number of successful calls
    pub call_count: u64,
    /// Number of failed calls
    pub error_count: u64,
    /// Configuration
    pub config: SisterBridgeConfig,
}

impl BridgeRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Build a registry from config.
    pub fn from_config(config: &BridgeConfig) -> Self {
        let mut registry = Self::new();
        if !config.enabled {
            return registry;
        }

        for (name, sister_config) in &config.sisters {
            if sister_config.enabled {
                registry.register(name.clone(), sister_config.clone());
            }
        }

        registry
    }

    /// Register a bridge.
    pub fn register(&mut self, name: String, config: SisterBridgeConfig) {
        self.entries.insert(
            name.clone(),
            BridgeEntry {
                name,
                connected: false,
                last_ping: None,
                call_count: 0,
                error_count: 0,
                config,
            },
        );
    }

    /// Mark a bridge as connected.
    pub fn mark_connected(&mut self, name: &str) {
        if let Some(entry) = self.entries.get_mut(name) {
            entry.connected = true;
            entry.last_ping = Some(chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));
        }
    }

    /// Mark a bridge as disconnected.
    pub fn mark_disconnected(&mut self, name: &str) {
        if let Some(entry) = self.entries.get_mut(name) {
            entry.connected = false;
        }
    }

    /// Record a successful call.
    pub fn record_success(&mut self, name: &str) {
        if let Some(entry) = self.entries.get_mut(name) {
            entry.call_count += 1;
            entry.last_ping = Some(chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));
        }
    }

    /// Record a failed call.
    pub fn record_error(&mut self, name: &str) {
        if let Some(entry) = self.entries.get_mut(name) {
            entry.error_count += 1;
        }
    }

    /// Check if a bridge is registered and connected.
    pub fn is_available(&self, name: &str) -> bool {
        self.entries.get(name).map(|e| e.connected).unwrap_or(false)
    }

    /// Get all registered bridge names.
    pub fn registered(&self) -> Vec<&str> {
        self.entries.keys().map(|s| s.as_str()).collect()
    }

    /// Get all connected bridge names.
    pub fn connected(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|(_, e)| e.connected)
            .map(|(k, _)| k.as_str())
            .collect()
    }

    /// Get a bridge entry by name.
    pub fn get(&self, name: &str) -> Option<&BridgeEntry> {
        self.entries.get(name)
    }

    /// Get total call count across all bridges.
    pub fn total_calls(&self) -> u64 {
        self.entries.values().map(|e| e.call_count).sum()
    }

    /// Get total error count across all bridges.
    pub fn total_errors(&self) -> u64 {
        self.entries.values().map(|e| e.error_count).sum()
    }
}

impl Default for BridgeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_config_default() {
        let config = BridgeConfig::default();
        assert!(config.enabled);
        assert_eq!(config.discovery, "static");
        assert_eq!(config.timeout_ms, 5000);
    }

    #[test]
    fn test_bridge_config_standalone() {
        let config = BridgeConfig::standalone();
        assert!(!config.enabled);
    }

    #[test]
    fn test_bridge_config_with_sisters() {
        let config = BridgeConfig::default()
            .with_sister("memory", SisterBridgeConfig::in_process())
            .with_sister(
                "vision",
                SisterBridgeConfig::mcp_stdio("agentic-vision-mcp"),
            );

        assert!(config.has_sister("memory"));
        assert!(config.has_sister("vision"));
        assert!(!config.has_sister("planning"));
    }

    #[test]
    fn test_bridge_registry_lifecycle() {
        let mut registry = BridgeRegistry::new();

        registry.register("memory".to_string(), SisterBridgeConfig::in_process());
        assert!(!registry.is_available("memory"));
        assert_eq!(registry.registered().len(), 1);

        registry.mark_connected("memory");
        assert!(registry.is_available("memory"));
        assert_eq!(registry.connected().len(), 1);

        registry.record_success("memory");
        registry.record_success("memory");
        registry.record_error("memory");

        let entry = registry.get("memory").unwrap();
        assert_eq!(entry.call_count, 2);
        assert_eq!(entry.error_count, 1);

        registry.mark_disconnected("memory");
        assert!(!registry.is_available("memory"));
    }

    #[test]
    fn test_registry_from_config() {
        let config = BridgeConfig::default()
            .with_sister("memory", SisterBridgeConfig::in_process())
            .with_sister("vision", SisterBridgeConfig::in_process());

        let registry = BridgeRegistry::from_config(&config);
        assert_eq!(registry.registered().len(), 2);

        // Standalone config should produce empty registry
        let standalone = BridgeConfig::standalone();
        let empty_registry = BridgeRegistry::from_config(&standalone);
        assert_eq!(empty_registry.registered().len(), 0);
    }

    #[test]
    fn test_totals() {
        let mut registry = BridgeRegistry::new();
        registry.register("a".into(), SisterBridgeConfig::in_process());
        registry.register("b".into(), SisterBridgeConfig::in_process());

        registry.record_success("a");
        registry.record_success("a");
        registry.record_success("b");
        registry.record_error("b");

        assert_eq!(registry.total_calls(), 3);
        assert_eq!(registry.total_errors(), 1);
    }
}
