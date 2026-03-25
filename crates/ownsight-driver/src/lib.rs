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

/// Backend availability information
#[derive(Debug, Clone)]
pub struct BackendStatus {
    pub simple_available: bool,
    pub mir_available: bool,
    pub mir_error: Option<String>,
}

/// Check if MIR backend is available and functional
pub fn check_mir_availability() -> BackendStatus {
    let simple_available = true; // Simple backend is always available
    
    #[cfg(feature = "mir")]
    {
        // Try to create a MIR analyzer with a simple test
        let test_code = "fn main() {}";
        let mut analyzer = ownsight_mir::MirAnalyzer::new(AnalysisMode::Teaching);
        
        match analyzer.analyze(test_code, "test.rs") {
            Ok(_) => BackendStatus {
                simple_available,
                mir_available: true,
                mir_error: None,
            },
            Err(e) => BackendStatus {
                simple_available,
                mir_available: false,
                mir_error: Some(format!("MIR backend test failed: {}", e)),
            },
        }
    }
    
    #[cfg(not(feature = "mir"))]
    {
        BackendStatus {
            simple_available,
            mir_available: false,
            mir_error: Some("MIR backend not compiled. Binary was built without --features mir".to_string()),
        }
    }
}

/// Create an analyzer with the specified backend, with automatic fallback
pub fn create_analyzer(backend: AnalyzerBackend, mode: AnalysisMode) -> Box<dyn OwnershipAnalyzer> {
    create_analyzer_with_status(backend, mode).0
}

/// Create an analyzer with the specified backend and return status information
pub fn create_analyzer_with_status(
    backend: AnalyzerBackend,
    mode: AnalysisMode,
) -> (Box<dyn OwnershipAnalyzer>, BackendStatus) {
    match backend {
        AnalyzerBackend::Simple => {
            let status = BackendStatus {
                simple_available: true,
                mir_available: cfg!(feature = "mir"),
                mir_error: None,
            };
            (Box::new(SimpleAnalyzer::new(mode)), status)
        }
        AnalyzerBackend::Mir => {
            #[cfg(feature = "mir")]
            {
                let status = BackendStatus {
                    simple_available: true,
                    mir_available: true,
                    mir_error: None,
                };
                (Box::new(ownsight_mir::MirAnalyzer::new(mode)), status)
            }
            #[cfg(not(feature = "mir"))]
            {
                eprintln!("⚠ MIR backend requested but not available. Falling back to Simple analyzer.");
                eprintln!("ℹ To use MIR backend:");
                eprintln!("  • Download a pre-built binary with MIR support from GitHub releases");
                eprintln!("  • Or compile from source with: cargo build --features mir");
                
                let status = BackendStatus {
                    simple_available: true,
                    mir_available: false,
                    mir_error: Some("MIR feature not enabled in this build".to_string()),
                };
                (Box::new(SimpleAnalyzer::new(mode)), status)
            }
        }
    }
}
