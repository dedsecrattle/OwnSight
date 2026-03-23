//! Function summary and cross-function analysis

use ownsight_core::*;
use std::collections::HashMap;

extern crate rustc_middle;
extern crate rustc_hir;

use rustc_middle::ty::TyCtxt;
use rustc_middle::mir::Body;
use rustc_hir::def_id::DefId;

/// Function ownership summary
#[derive(Debug, Clone)]
pub struct FunctionOwnershipSummary {
    pub def_id: DefId,
    pub name: String,
    pub consumes_params: Vec<usize>,
    pub borrows_shared_params: Vec<usize>,
    pub borrows_mut_params: Vec<usize>,
    pub returns_ownership: bool,
    pub references_escape: bool,
}

/// Function summary analyzer
pub struct FunctionAnalyzer<'tcx> {
    tcx: TyCtxt<'tcx>,
    summaries: HashMap<DefId, FunctionOwnershipSummary>,
}

impl<'tcx> FunctionAnalyzer<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self {
            tcx,
            summaries: HashMap::new(),
        }
    }

    /// Generate ownership summary for a function
    pub fn analyze_function(&mut self, def_id: DefId, _body: &Body<'tcx>) -> FunctionOwnershipSummary {
        // TODO: Analyze function parameters and return values
        // This will be implemented in Phase 3
        
        let name = self.tcx.def_path_str(def_id);
        
        FunctionOwnershipSummary {
            def_id,
            name,
            consumes_params: Vec::new(),
            borrows_shared_params: Vec::new(),
            borrows_mut_params: Vec::new(),
            returns_ownership: false,
            references_escape: false,
        }
    }

    /// Get summary for a function
    pub fn get_summary(&self, def_id: DefId) -> Option<&FunctionOwnershipSummary> {
        self.summaries.get(&def_id)
    }

    /// Store a function summary
    pub fn store_summary(&mut self, summary: FunctionOwnershipSummary) {
        self.summaries.insert(summary.def_id, summary);
    }
}
