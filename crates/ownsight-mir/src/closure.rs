//! Closure capture analysis

use ownsight_core::*;
use std::collections::HashMap;

extern crate rustc_middle;
extern crate rustc_hir;

use rustc_middle::ty::{TyCtxt, TyKind, UpvarCapture};
use rustc_middle::mir::Body;
use rustc_hir::def_id::DefId;

/// Closure capture mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureMode {
    ByValue,
    ByRef,
    ByMutRef,
}

/// Information about a captured variable
#[derive(Debug, Clone)]
pub struct CaptureInfo {
    pub var_id: VariableId,
    pub mode: CaptureMode,
    pub upvar_index: usize,
}

/// Closure analyzer
pub struct ClosureAnalyzer<'tcx> {
    tcx: TyCtxt<'tcx>,
    captures: HashMap<DefId, Vec<CaptureInfo>>,
}

impl<'tcx> ClosureAnalyzer<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self {
            tcx,
            captures: HashMap::new(),
        }
    }

    /// Analyze closure captures
    pub fn analyze_closure(&mut self, def_id: DefId, _body: &Body<'tcx>) -> Vec<CaptureInfo> {
        let mut captures = Vec::new();
        
        // Check if this is a closure
        let ty = self.tcx.type_of(def_id).instantiate_identity();
        
        if let TyKind::Closure(closure_def_id, _) = ty.kind() {
            // Get upvar captures
            let upvar_tys = self.tcx.upvars_mentioned(def_id);
            
            if let Some(upvars) = upvar_tys {
                for (idx, (_, upvar)) in upvars.iter().enumerate() {
                    // Determine capture mode based on how the variable is used
                    let mode = self.determine_capture_mode(*closure_def_id, idx);
                    
                    captures.push(CaptureInfo {
                        var_id: VariableId(idx), // This would need proper mapping
                        mode,
                        upvar_index: idx,
                    });
                }
            }
        }
        
        self.captures.insert(def_id, captures.clone());
        captures
    }

    /// Determine how a variable is captured
    fn determine_capture_mode(&self, closure_def_id: DefId, upvar_idx: usize) -> CaptureMode {
        // Try to get capture information from the type system
        if let Some(captures) = self.tcx.closure_captures(closure_def_id) {
            if let Some(capture) = captures.get(upvar_idx) {
                return match capture.info.capture_kind {
                    rustc_middle::ty::UpvarCapture::ByValue => CaptureMode::ByValue,
                    rustc_middle::ty::UpvarCapture::ByRef(borrow_kind) => {
                        match borrow_kind {
                            rustc_middle::mir::BorrowKind::Mut { .. } => CaptureMode::ByMutRef,
                            _ => CaptureMode::ByRef,
                        }
                    }
                };
            }
        }
        
        // Default to by-reference if we can't determine
        CaptureMode::ByRef
    }

    /// Get captures for a closure
    pub fn get_captures(&self, def_id: DefId) -> Option<&Vec<CaptureInfo>> {
        self.captures.get(&def_id)
    }
    
    /// Check if a DefId represents a closure
    pub fn is_closure(&self, def_id: DefId) -> bool {
        let ty = self.tcx.type_of(def_id).instantiate_identity();
        matches!(ty.kind(), TyKind::Closure(_, _))
    }
    
    /// Get closure trait (Fn, FnMut, or FnOnce)
    pub fn get_closure_trait(&self, def_id: DefId) -> Option<&'static str> {
        let ty = self.tcx.type_of(def_id).instantiate_identity();
        
        if let TyKind::Closure(_, _) = ty.kind() {
            // Check which closure trait is implemented
            // This is a simplified check - full implementation would query trait system
            let has_mut_captures = self.captures.get(&def_id)
                .map(|caps| caps.iter().any(|c| matches!(c.mode, CaptureMode::ByMutRef)))
                .unwrap_or(false);
            
            let has_move_captures = self.captures.get(&def_id)
                .map(|caps| caps.iter().any(|c| matches!(c.mode, CaptureMode::ByValue)))
                .unwrap_or(false);
            
            if has_move_captures {
                Some("FnOnce")
            } else if has_mut_captures {
                Some("FnMut")
            } else {
                Some("Fn")
            }
        } else {
            None
        }
    }
}
