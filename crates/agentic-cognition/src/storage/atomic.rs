//! Atomic file operations — write to temp, then rename
//! Prevents corruption from crashes during write.

use crate::types::{CognitionError, CognitionResult};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Atomic writer — ensures file integrity via temp+rename pattern
pub struct AtomicWriter {
    temp_path: PathBuf,
    final_path: PathBuf,
    file: File,
    lock_path: PathBuf,
    committed: bool,
}

impl AtomicWriter {
    /// Create a new atomic writer for the given path
    pub fn new(path: &Path) -> CognitionResult<Self> {
        let temp_path = path.with_extension("acog.tmp");
        let lock_path = path.with_extension("acog.lock");

        // Check for stale lock
        Self::check_stale_lock(&lock_path)?;

        // Create lock file
        let _lock = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock_path)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    CognitionError::LockError(format!(
                        "Lock file exists: {}. Another process may be writing.",
                        lock_path.display()
                    ))
                } else {
                    CognitionError::IoError(e)
                }
            })?;

        let file = File::create(&temp_path)?;

        Ok(Self {
            temp_path,
            final_path: path.to_path_buf(),
            file,
            lock_path,
            committed: false,
        })
    }

    /// Check if a lock file is stale (older than 60 seconds)
    fn check_stale_lock(lock_path: &Path) -> CognitionResult<()> {
        if !lock_path.exists() {
            return Ok(());
        }

        // Check lock age — if older than 60 seconds, consider stale
        if let Ok(metadata) = fs::metadata(lock_path) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(elapsed) = modified.elapsed() {
                    if elapsed.as_secs() > 60 {
                        let _ = fs::remove_file(lock_path);
                        return Ok(());
                    }
                }
            }
        }

        Err(CognitionError::LockError(format!(
            "Lock file exists: {}",
            lock_path.display()
        )))
    }

    /// Write data to the temp file
    pub fn write_all(&mut self, data: &[u8]) -> CognitionResult<()> {
        self.file.write_all(data).map_err(CognitionError::IoError)
    }

    /// Commit the write — sync and atomic rename
    pub fn commit(mut self) -> CognitionResult<()> {
        // Sync to disk
        self.file.sync_all()?;

        // Atomic rename
        fs::rename(&self.temp_path, &self.final_path)?;

        // Sync parent directory (Unix)
        #[cfg(unix)]
        {
            if let Some(parent) = self.final_path.parent() {
                if let Ok(dir) = File::open(parent) {
                    let _ = dir.sync_all();
                }
            }
        }

        // Remove lock
        let _ = fs::remove_file(&self.lock_path);
        self.committed = true;

        Ok(())
    }

    /// Abort the write — clean up temp and lock
    pub fn abort(mut self) {
        let _ = fs::remove_file(&self.temp_path);
        let _ = fs::remove_file(&self.lock_path);
        self.committed = true; // prevent double cleanup in Drop
    }
}

impl Drop for AtomicWriter {
    fn drop(&mut self) {
        if !self.committed {
            let _ = fs::remove_file(&self.temp_path);
            let _ = fs::remove_file(&self.lock_path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_atomic_write_commit() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.acog");

        let mut writer = AtomicWriter::new(&path).unwrap();
        writer.write_all(b"hello world").unwrap();
        writer.commit().unwrap();

        assert!(path.exists());
        assert_eq!(fs::read_to_string(&path).unwrap(), "hello world");
        assert!(!path.with_extension("acog.tmp").exists());
        assert!(!path.with_extension("acog.lock").exists());
    }

    #[test]
    fn test_atomic_write_abort() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.acog");

        let mut writer = AtomicWriter::new(&path).unwrap();
        writer.write_all(b"should not persist").unwrap();
        writer.abort();

        assert!(!path.exists());
        assert!(!path.with_extension("acog.tmp").exists());
        assert!(!path.with_extension("acog.lock").exists());
    }

    #[test]
    fn test_atomic_write_drop_cleanup() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.acog");

        {
            let mut writer = AtomicWriter::new(&path).unwrap();
            writer.write_all(b"uncommitted").unwrap();
            // Drop without commit
        }

        assert!(!path.exists());
        assert!(!path.with_extension("acog.tmp").exists());
    }

    #[test]
    fn test_concurrent_lock_rejection() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.acog");

        let _writer = AtomicWriter::new(&path).unwrap();
        // Second writer should fail
        let result = AtomicWriter::new(&path);
        assert!(result.is_err());
    }
}
