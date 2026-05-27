//! Pipeline configuration types and defaults.
//!
//! This module contains the PipelineConfig struct, all algorithm enums,
//! mask operation types, and their serde serialization/deserialization.
//!
//! Default values are sourced from qsm-core parameter structs.

// TODO: Extract from qsmxt.rs/src/pipeline/config.rs
// This is a placeholder — the full extraction involves ~900 lines of types,
// defaults, validation, and TOML serialization.

use serde::{Deserialize, Serialize};

/// Placeholder PipelineConfig — will be replaced with full extraction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    #[serde(default)]
    pub description: String,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            description: String::new(),
        }
    }
}
