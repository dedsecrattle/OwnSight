//! MIR-based ownership analysis using rustc internals
//! 
//! This crate provides accurate ownership and borrowing analysis by leveraging
//! the Rust compiler's MIR (Mid-level Intermediate Representation).

#[cfg(feature = "rustc")]
pub mod driver;
#[cfg(feature = "rustc")]
pub mod mir_visitor;
#[cfg(feature = "rustc")]
pub mod lifetime;
#[cfg(feature = "rustc")]
pub mod closure;
#[cfg(feature = "rustc")]
pub mod async_await;
#[cfg(feature = "rustc")]
pub mod partial_move;
#[cfg(feature = "rustc")]
pub mod function;

pub mod analyzer;

pub use analyzer::MirAnalyzer;

use ownsight_core::*;
use anyhow::Result;

/// Trait for MIR-based ownership analysis
pub trait MirOwnershipAnalyzer {
    fn analyze(&mut self, source: &str, filename: &str) -> Result<ProgramAnalysis>;
    fn analyze_crate(&mut self, crate_path: &str) -> Result<ProgramAnalysis>;
}
