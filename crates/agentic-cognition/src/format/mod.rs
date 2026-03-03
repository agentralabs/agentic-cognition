//! .acog file format — binary format with checksums for persisting user models
//!
//! File structure:
//! [MAGIC_BYTES: 4] [VERSION: 2] [FLAGS: 2] [HEADER_LEN: 4]
//! [HEADER_CHECKSUM: 32] [HEADER_DATA: variable]
//! [BODY_CHECKSUM: 32] [BODY_DATA: variable]

use crate::types::{
    BeliefGraph, BiasField, CognitionError, CognitionResult, DecisionFingerprint, DriftTimeline,
    LivingUserModel, ShadowMap,
};
use serde::{Deserialize, Serialize};
use std::io::Write as _;
use std::path::Path;

/// Magic bytes for .acog files
const MAGIC: &[u8; 4] = b"ACOG";
/// Current format version
const VERSION: u16 = 1;

/// Complete .acog file content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcogFile {
    pub model: LivingUserModel,
    pub belief_graph: BeliefGraph,
    pub shadow: ShadowMap,
    pub bias_field: BiasField,
    pub drift: DriftTimeline,
    pub fingerprint: Option<DecisionFingerprint>,
}

impl AcogFile {
    pub fn new(model: LivingUserModel) -> Self {
        Self {
            model,
            belief_graph: BeliefGraph::default(),
            shadow: ShadowMap::default(),
            bias_field: BiasField::default(),
            drift: DriftTimeline::default(),
            fingerprint: None,
        }
    }

    /// Save to a file path atomically (write to temp, then rename)
    pub fn save(&self, path: &Path) -> CognitionResult<()> {
        let json = serde_json::to_vec(self)
            .map_err(|e| CognitionError::SerializationError(e.to_string()))?;

        let checksum = blake3::hash(&json);

        let mut data = Vec::new();
        data.extend_from_slice(MAGIC);
        data.extend_from_slice(&VERSION.to_le_bytes());
        data.extend_from_slice(&[0u8; 2]); // flags
        data.extend_from_slice(&(json.len() as u32).to_le_bytes());
        data.extend_from_slice(checksum.as_bytes());
        data.extend_from_slice(&json);

        // Atomic write: write to temp file, then rename
        let temp_path = path.with_extension("acog.tmp");
        let mut file = std::fs::File::create(&temp_path)?;
        file.write_all(&data)?;
        file.sync_all()?;
        std::fs::rename(&temp_path, path)?;

        Ok(())
    }

    /// Load from a file path
    pub fn load(path: &Path) -> CognitionResult<Self> {
        let data = std::fs::read(path)?;

        if data.len() < 44 {
            return Err(CognitionError::FormatError("File too small".into()));
        }

        // Verify magic bytes
        if &data[0..4] != MAGIC {
            return Err(CognitionError::FormatError("Invalid magic bytes".into()));
        }

        // Read version
        let version = u16::from_le_bytes([data[4], data[5]]);
        if version > VERSION {
            return Err(CognitionError::FormatError(format!(
                "Unsupported version: {version}, max supported: {VERSION}"
            )));
        }

        // Read header length
        let header_len = u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize;

        // Read checksum
        let stored_checksum = &data[12..44];

        // Read body
        let body_start = 44;
        let body_end = body_start + header_len;
        if data.len() < body_end {
            return Err(CognitionError::FormatError("File truncated".into()));
        }

        let body = &data[body_start..body_end];

        // Verify checksum
        let computed_checksum = blake3::hash(body);
        if computed_checksum.as_bytes() != stored_checksum {
            return Err(CognitionError::ChecksumMismatch {
                expected: format!("{:?}", stored_checksum),
                actual: computed_checksum.to_hex().to_string(),
            });
        }

        let file: AcogFile = serde_json::from_slice(body)
            .map_err(|e| CognitionError::SerializationError(e.to_string()))?;

        Ok(file)
    }
}
