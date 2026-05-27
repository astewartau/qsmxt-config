//! QSMxT Pipeline Configuration Library
//!
//! Shared configuration, command generation, and methods text for QSMxT tools.
//! Used by both qsmxt.rs (CLI/TUI) and qsmbly (browser WASM).

pub mod config;
pub mod command;
pub mod methods;
pub mod error;

pub use config::*;
pub use error::{ConfigError, Result};
