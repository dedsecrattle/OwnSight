//! MIR Analyzer - Main entry point for MIR-based analysis

use ownsight_core::*;
use anyhow::{Result, bail};

#[cfg(feature = "rustc")]
use crate::driver::RustcDriver;

/// MIR-based ownership analyzer
pub struct MirAnalyzer {
    mode: AnalysisMode,
    #[cfg(feature = "rustc")]
    driver: Option<RustcDriver>,
}

impl MirAnalyzer {
    pub fn new(mode: AnalysisMode) -> Self {
        Self {
            mode,
            #[cfg(feature = "rustc")]
            driver: None,
        }
    }

    #[cfg(feature = "rustc")]
    pub fn analyze(&mut self, source: &str, filename: &str) -> Result<ProgramAnalysis> {
        // Initialize driver if not already done
        if self.driver.is_none() {
            self.driver = Some(RustcDriver::new(self.mode.clone())?);
        }

        let driver = self.driver.as_mut().unwrap();
        driver.analyze_source(source, filename)
    }

    #[cfg(not(feature = "rustc"))]
    pub fn analyze(&mut self, _source: &str, _filename: &str) -> Result<ProgramAnalysis> {
        bail!("MIR analysis requires the 'rustc' feature to be enabled. Please compile with --features rustc")
    }

    #[cfg(feature = "rustc")]
    pub fn analyze_crate(&mut self, crate_path: &str) -> Result<ProgramAnalysis> {
        if self.driver.is_none() {
            self.driver = Some(RustcDriver::new(self.mode.clone())?);
        }

        let driver = self.driver.as_mut().unwrap();
        driver.analyze_crate(crate_path)
    }

    #[cfg(not(feature = "rustc"))]
    pub fn analyze_crate(&mut self, _crate_path: &str) -> Result<ProgramAnalysis> {
        bail!("MIR analysis requires the 'rustc' feature to be enabled. Please compile with --features rustc")
    }
}

impl crate::MirOwnershipAnalyzer for MirAnalyzer {
    fn analyze(&mut self, source: &str, filename: &str) -> Result<ProgramAnalysis> {
        self.analyze(source, filename)
    }

    fn analyze_crate(&mut self, crate_path: &str) -> Result<ProgramAnalysis> {
        self.analyze_crate(crate_path)
    }
}
