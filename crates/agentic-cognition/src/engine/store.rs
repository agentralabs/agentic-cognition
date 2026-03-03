//! CognitionStore — persistence layer for user models

use crate::engine::index::IndexManager;
use crate::format::AcogFile;
use crate::types::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// In-memory store for cognition data with optional file persistence
pub struct CognitionStore {
    /// All loaded models
    models: Arc<RwLock<HashMap<ModelId, AcogFile>>>,
    /// Index manager
    indexes: Arc<RwLock<HashMap<ModelId, IndexManager>>>,
    /// Storage directory
    storage_dir: Option<PathBuf>,
}

impl CognitionStore {
    /// Create a new in-memory store
    pub fn new() -> Self {
        Self {
            models: Arc::new(RwLock::new(HashMap::new())),
            indexes: Arc::new(RwLock::new(HashMap::new())),
            storage_dir: None,
        }
    }

    /// Create a store with file persistence
    pub fn with_storage(dir: PathBuf) -> CognitionResult<Self> {
        std::fs::create_dir_all(&dir)?;
        let mut store = Self::new();
        store.storage_dir = Some(dir);
        store.load_all()?;
        Ok(store)
    }

    /// Load all .acog files from the storage directory
    fn load_all(&self) -> CognitionResult<()> {
        let dir = match &self.storage_dir {
            Some(d) => d,
            None => return Ok(()),
        };

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("acog") {
                match AcogFile::load(&path) {
                    Ok(file) => {
                        let model_id = file.model.id;
                        let mut idx = IndexManager::new();
                        idx.rebuild(&file.belief_graph);

                        let mut models = self
                            .models
                            .write()
                            .map_err(|e| CognitionError::LockError(e.to_string()))?;
                        models.insert(model_id, file);

                        let mut indexes = self
                            .indexes
                            .write()
                            .map_err(|e| CognitionError::LockError(e.to_string()))?;
                        indexes.insert(model_id, idx);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to load {:?}: {}", path, e);
                    }
                }
            }
        }

        Ok(())
    }

    /// Save a specific model to disk
    pub fn persist(&self, model_id: &ModelId) -> CognitionResult<()> {
        let dir = match &self.storage_dir {
            Some(d) => d,
            None => return Ok(()),
        };

        let models = self
            .models
            .read()
            .map_err(|e| CognitionError::LockError(e.to_string()))?;

        let file = models
            .get(model_id)
            .ok_or(CognitionError::ModelNotFound(*model_id))?;
        let path = dir.join(format!("{}.acog", model_id));
        file.save(&path)
    }

    /// Insert a new model
    pub fn insert_model(&self, file: AcogFile) -> CognitionResult<ModelId> {
        let model_id = file.model.id;

        let mut models = self
            .models
            .write()
            .map_err(|e| CognitionError::LockError(e.to_string()))?;

        if models.contains_key(&model_id) {
            return Err(CognitionError::ModelAlreadyExists(model_id));
        }

        let mut idx = IndexManager::new();
        idx.rebuild(&file.belief_graph);

        models.insert(model_id, file);

        let mut indexes = self
            .indexes
            .write()
            .map_err(|e| CognitionError::LockError(e.to_string()))?;
        indexes.insert(model_id, idx);

        drop(models);
        drop(indexes);

        // Auto-persist if storage is configured
        self.persist(&model_id)?;

        Ok(model_id)
    }

    /// Get a model reference (cloned to avoid holding lock)
    pub fn get_model(&self, model_id: &ModelId) -> CognitionResult<AcogFile> {
        let models = self
            .models
            .read()
            .map_err(|e| CognitionError::LockError(e.to_string()))?;
        models
            .get(model_id)
            .cloned()
            .ok_or(CognitionError::ModelNotFound(*model_id))
    }

    /// Update a model in the store
    pub fn update_model(
        &self,
        model_id: &ModelId,
        updater: impl FnOnce(&mut AcogFile),
    ) -> CognitionResult<()> {
        let mut models = self
            .models
            .write()
            .map_err(|e| CognitionError::LockError(e.to_string()))?;
        let file = models
            .get_mut(model_id)
            .ok_or(CognitionError::ModelNotFound(*model_id))?;
        updater(file);

        // Rebuild index
        let mut indexes = self
            .indexes
            .write()
            .map_err(|e| CognitionError::LockError(e.to_string()))?;
        let idx = indexes.entry(*model_id).or_insert_with(IndexManager::new);
        idx.rebuild(&file.belief_graph);

        drop(models);
        drop(indexes);

        self.persist(model_id)?;
        Ok(())
    }

    /// Delete a model
    pub fn delete_model(&self, model_id: &ModelId) -> CognitionResult<()> {
        let mut models = self
            .models
            .write()
            .map_err(|e| CognitionError::LockError(e.to_string()))?;
        models
            .remove(model_id)
            .ok_or(CognitionError::ModelNotFound(*model_id))?;

        let mut indexes = self
            .indexes
            .write()
            .map_err(|e| CognitionError::LockError(e.to_string()))?;
        indexes.remove(model_id);

        // Remove file if exists
        if let Some(dir) = &self.storage_dir {
            let path = dir.join(format!("{}.acog", model_id));
            if path.exists() {
                std::fs::remove_file(&path)?;
            }
        }

        Ok(())
    }

    /// List all model IDs
    pub fn list_models(&self) -> CognitionResult<Vec<ModelId>> {
        let models = self
            .models
            .read()
            .map_err(|e| CognitionError::LockError(e.to_string()))?;
        Ok(models.keys().copied().collect())
    }

    /// Get index for a model
    pub fn get_index(&self, model_id: &ModelId) -> CognitionResult<IndexManager> {
        let indexes = self
            .indexes
            .read()
            .map_err(|e| CognitionError::LockError(e.to_string()))?;
        indexes
            .get(model_id)
            .cloned()
            .ok_or(CognitionError::ModelNotFound(*model_id))
    }
}

impl Default for CognitionStore {
    fn default() -> Self {
        Self::new()
    }
}

// Allow IndexManager to be cloned for returning from locked context
impl Clone for IndexManager {
    fn clone(&self) -> Self {
        Self {
            beliefs_by_domain: self.beliefs_by_domain.clone(),
            beliefs_by_state: self.beliefs_by_state.clone(),
            high_confidence_beliefs: self.high_confidence_beliefs.clone(),
            crystallized_beliefs: self.crystallized_beliefs.clone(),
        }
    }
}
