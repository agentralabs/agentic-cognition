//! Per-project isolation — prevents cross-project contamination

use crate::types::{CognitionError, CognitionResult};
use std::path::{Path, PathBuf};

/// Unique identity for a project based on canonical path
pub struct ProjectIdentity {
    /// Unique project ID (blake3 hash of canonical path)
    pub project_id: String,
    /// Canonical path of the project
    pub canonical_path: PathBuf,
    /// Data directory for this project
    pub data_dir: PathBuf,
    /// Cache directory for this project
    pub cache_dir: PathBuf,
}

impl ProjectIdentity {
    /// Create from a project directory path
    pub fn from_path(path: &Path) -> CognitionResult<Self> {
        let canonical = path.canonicalize().map_err(CognitionError::IoError)?;
        let hash = blake3::hash(canonical.to_string_lossy().as_bytes());
        let project_id = format!("proj_{}", &hash.to_hex()[..16]);

        let base_dir = Self::base_data_dir();
        let data_dir = base_dir.join(&project_id).join("data");
        let cache_dir = base_dir.join(&project_id).join("cache");

        std::fs::create_dir_all(&data_dir).map_err(CognitionError::IoError)?;
        std::fs::create_dir_all(&cache_dir).map_err(CognitionError::IoError)?;

        Ok(Self {
            project_id,
            canonical_path: canonical,
            data_dir,
            cache_dir,
        })
    }

    /// Get the .acog file path for this project
    pub fn acog_path(&self) -> PathBuf {
        self.data_dir.join("cognition.acog")
    }

    /// Base data directory for all projects
    fn base_data_dir() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("agentic-cognition")
    }
}

/// Guard to prevent cross-project contamination
pub struct IsolationGuard {
    project_id: String,
}

impl IsolationGuard {
    pub fn new(project_id: String) -> Self {
        Self { project_id }
    }

    /// Validate that an operation targets the correct project
    pub fn validate(&self, target_project_id: &str) -> CognitionResult<()> {
        if self.project_id != target_project_id {
            return Err(CognitionError::ValidationError(format!(
                "Cross-project access denied: expected {}, got {}",
                self.project_id, target_project_id
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_project_identity() {
        let dir = TempDir::new().unwrap();
        let identity = ProjectIdentity::from_path(dir.path()).unwrap();
        assert!(identity.project_id.starts_with("proj_"));
        assert!(identity.data_dir.exists());
        assert!(identity.cache_dir.exists());
    }

    #[test]
    fn test_isolation_guard() {
        let guard = IsolationGuard::new("proj_abc".to_string());
        assert!(guard.validate("proj_abc").is_ok());
        assert!(guard.validate("proj_xyz").is_err());
    }

    #[test]
    fn test_deterministic_project_id() {
        let dir = TempDir::new().unwrap();
        let id1 = ProjectIdentity::from_path(dir.path()).unwrap();
        let id2 = ProjectIdentity::from_path(dir.path()).unwrap();
        assert_eq!(id1.project_id, id2.project_id);
    }
}
