pub mod simple_analyzer;

pub use simple_analyzer::*;

use ownsight_core::*;
use anyhow::Result;

/// Backend selection for analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalyzerBackend {
    /// Simple syntax-based analyzer (Layer 1)
    Simple,
    /// MIR-based analyzer using rustc internals (Layer 2)
    Mir,
}

impl Default for AnalyzerBackend {
    fn default() -> Self {
        Self::Simple
    }
}

pub trait OwnershipAnalyzer {
    fn analyze(&mut self, source: &str, filename: &str) -> Result<ProgramAnalysis>;
}

/// Create an analyzer with the specified backend
pub fn create_analyzer(backend: AnalyzerBackend, mode: AnalysisMode) -> Box<dyn OwnershipAnalyzer> {
    match backend {
        AnalyzerBackend::Simple => Box::new(SimpleAnalyzer::new(mode)),
        AnalyzerBackend::Mir => {
            #[cfg(feature = "mir")]
            {
                Box::new(ownsight_mir::MirAnalyzer::new(mode))
            }
            #[cfg(not(feature = "mir"))]
            {
                eprintln!("Warning: MIR backend requested but not available. Falling back to Simple analyzer.");
                eprintln!("To use MIR backend, compile with --features mir");
                Box::new(SimpleAnalyzer::new(mode))
            }
        }
    }
}
