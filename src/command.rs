//! CLI command generation from PipelineConfig.
//!
//! Generates a `qsmxt run` command string by comparing a config against defaults
//! and emitting only non-default flags.

use crate::config::PipelineConfig;

/// Generate a `qsmxt run` CLI command from a pipeline configuration.
///
/// Compares against `PipelineConfig::default()` and only emits flags that differ.
pub fn generate_command(config: &PipelineConfig) -> String {
    let _defaults = PipelineConfig::default();
    // TODO: implement
    "qsmxt run <bids_dir>".to_string()
}
