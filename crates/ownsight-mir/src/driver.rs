//! Rustc driver integration for MIR extraction

use ownsight_core::*;
use anyhow::{Result, Context};
use std::path::PathBuf;

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use rustc_driver::Compilation;
use rustc_interface::{interface, Queries};
use rustc_middle::ty::TyCtxt;
use rustc_session::config::{self, CheckCfg};

use crate::mir_visitor::MirVisitor;

/// Rustc driver for extracting MIR and performing analysis
pub struct RustcDriver {
    mode: AnalysisMode,
}

impl RustcDriver {
    pub fn new(mode: AnalysisMode) -> Result<Self> {
        Ok(Self { mode })
    }

    /// Analyze Rust source code
    pub fn analyze_source(&mut self, source: &str, filename: &str) -> Result<ProgramAnalysis> {
        // Create a temporary file for the source
        let temp_dir = std::env::temp_dir();
        let source_path = temp_dir.join(filename);
        std::fs::write(&source_path, source)
            .context("Failed to write temporary source file")?;

        let result = self.analyze_file(&source_path);

        // Clean up
        let _ = std::fs::remove_file(&source_path);

        result
    }

    /// Analyze a Rust file
    pub fn analyze_file(&mut self, path: &PathBuf) -> Result<ProgramAnalysis> {
        let mut analysis = ProgramAnalysis::new(self.mode.clone());

        // Set up rustc arguments
        let args = vec![
            "rustc".to_string(),
            path.to_string_lossy().to_string(),
            "--crate-type=lib".to_string(),
            "-Z".to_string(),
            "mir-opt-level=0".to_string(),
        ];

        // Create callbacks
        let mut callbacks = OwnsightCallbacks {
            analysis: &mut analysis,
            mode: self.mode.clone(),
        };

        // Run the compiler
        let compiler = rustc_driver::RunCompiler::new(&args, &mut callbacks);
        
        compiler.run().map_err(|_| {
            anyhow::anyhow!("Compilation failed")
        })?;

        Ok(analysis)
    }

    /// Analyze a crate
    pub fn analyze_crate(&mut self, crate_path: &str) -> Result<ProgramAnalysis> {
        let path = PathBuf::from(crate_path);
        
        if !path.exists() {
            anyhow::bail!("Crate path does not exist: {}", crate_path);
        }

        // Find Cargo.toml or main.rs/lib.rs
        let manifest_path = if path.is_dir() {
            path.join("Cargo.toml")
        } else {
            path
        };

        if !manifest_path.exists() {
            anyhow::bail!("Could not find Cargo.toml at: {:?}", manifest_path);
        }

        // For now, analyze the lib.rs or main.rs directly
        let src_path = manifest_path.parent().unwrap().join("src");
        let lib_path = src_path.join("lib.rs");
        let main_path = src_path.join("main.rs");

        let entry_point = if lib_path.exists() {
            lib_path
        } else if main_path.exists() {
            main_path
        } else {
            anyhow::bail!("Could not find lib.rs or main.rs in src/");
        };

        self.analyze_file(&entry_point)
    }
}

/// Callbacks for rustc compilation
struct OwnsightCallbacks<'a> {
    analysis: &'a mut ProgramAnalysis,
    mode: AnalysisMode,
}

impl rustc_driver::Callbacks for OwnsightCallbacks<'_> {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            // Extract MIR and analyze
            self.extract_mir(tcx);
        });

        // Continue compilation (we just want the analysis)
        Compilation::Stop
    }
}

impl<'a> OwnsightCallbacks<'a> {
    fn extract_mir<'tcx>(&mut self, tcx: TyCtxt<'tcx>) {
        // Get all local def ids
        let hir = tcx.hir();
        
        for item_id in hir.items() {
            let item = hir.item(item_id);
            
            // Check if this is a function
            if let rustc_hir::ItemKind::Fn(_, _, _) = item.kind {
                let def_id = item_id.owner_id.to_def_id();
                
                // Get the MIR for this function
                if let Ok(body) = tcx.mir_built(def_id.expect_local()) {
                    let body = body.borrow();
                    
                    // Create a visitor to traverse the MIR
                    let mut visitor = MirVisitor::new(
                        tcx,
                        self.analysis,
                        self.mode.clone(),
                        def_id,
                    );
                    
                    visitor.visit_body(&body);
                }
            }
        }
    }
}
