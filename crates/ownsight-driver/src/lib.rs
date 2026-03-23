pub mod simple_analyzer;

pub use simple_analyzer::*;

use ownsight_core::*;
use anyhow::Result;

pub trait OwnershipAnalyzer {
    fn analyze(&mut self, source: &str, filename: &str) -> Result<ProgramAnalysis>;
}
