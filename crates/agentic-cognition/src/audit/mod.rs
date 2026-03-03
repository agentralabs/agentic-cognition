//! Audit logging for sensitive operations

use crate::types::{CognitionResult, Timestamp};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

/// Audit log for recording sensitive operations
pub struct AuditLog {
    file: Option<File>,
    enabled: bool,
}

impl AuditLog {
    /// Create an enabled audit log writing to the given path
    pub fn new(path: &Path) -> CognitionResult<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self {
            file: Some(file),
            enabled: true,
        })
    }

    /// Create a disabled (no-op) audit log
    pub fn disabled() -> Self {
        Self {
            file: None,
            enabled: false,
        }
    }

    /// Log an audit entry
    pub fn log(&mut self, entry: &AuditEntry) -> CognitionResult<()> {
        if !self.enabled {
            return Ok(());
        }
        if let Some(ref mut file) = self.file {
            let json = serde_json::to_string(entry)
                .map_err(|e| crate::types::CognitionError::SerializationError(e.to_string()))?;
            writeln!(file, "{}", json)?;
            file.flush()?;
        }
        Ok(())
    }
}

/// A single audit entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: Timestamp,
    pub session_id: String,
    pub operation: String,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub action: AuditAction,
    pub success: bool,
    pub error: Option<String>,
    pub details: Option<serde_json::Value>,
}

impl AuditEntry {
    pub fn new(
        operation: &str,
        entity_type: EntityType,
        entity_id: &str,
        action: AuditAction,
    ) -> Self {
        Self {
            timestamp: Timestamp::now(),
            session_id: String::new(),
            operation: operation.to_string(),
            entity_type,
            entity_id: entity_id.to_string(),
            action,
            success: true,
            error: None,
            details: None,
        }
    }

    pub fn with_error(mut self, error: &str) -> Self {
        self.success = false;
        self.error = Some(error.to_string());
        self
    }
}

/// Type of audited action
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AuditAction {
    Create,
    Read,
    Update,
    Delete,
    Export,
    Consent,
    Auth,
}

/// Type of entity being audited
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EntityType {
    Model,
    Belief,
    Pattern,
    Shadow,
    Bias,
    Trigger,
    Prediction,
    Privacy,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_audit_log_write() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("audit.log");
        let mut log = AuditLog::new(&path).unwrap();

        let entry = AuditEntry::new(
            "create_model",
            EntityType::Model,
            "abc123",
            AuditAction::Create,
        );
        log.log(&entry).unwrap();

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("create_model"));
    }

    #[test]
    fn test_disabled_log() {
        let mut log = AuditLog::disabled();
        let entry = AuditEntry::new("test", EntityType::Model, "x", AuditAction::Read);
        log.log(&entry).unwrap(); // should not error
    }

    #[test]
    fn test_audit_entry_error() {
        let entry = AuditEntry::new("delete", EntityType::Model, "x", AuditAction::Delete)
            .with_error("Not found");
        assert!(!entry.success);
        assert_eq!(entry.error.as_deref(), Some("Not found"));
    }
}
