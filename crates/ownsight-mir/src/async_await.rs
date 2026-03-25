//! Async/await and Future analysis

use ownsight_core::*;
use std::collections::HashSet;

extern crate rustc_middle;
extern crate rustc_hir;

use rustc_middle::ty::{TyCtxt, TyKind};
use rustc_middle::mir::{Body, TerminatorKind, Local};
use rustc_hir::def_id::DefId;

/// Async suspension point
#[derive(Debug, Clone)]
pub struct SuspensionPoint {
    pub location: usize,
    pub line_number: usize,
    pub variables_live: Vec<VariableId>,
}

/// Async context information
#[derive(Debug, Clone)]
pub struct AsyncContext {
    pub is_async: bool,
    pub suspension_points: Vec<SuspensionPoint>,
    pub requires_send: bool,
    pub requires_sync: bool,
}

/// Async/await analyzer
pub struct AsyncAnalyzer<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl<'tcx> AsyncAnalyzer<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self { tcx }
    }

    /// Check if a function is async
    pub fn is_async_fn(&self, def_id: DefId) -> bool {
        // Check if the function returns a Future/Coroutine
        let ty = self.tcx.type_of(def_id).instantiate_identity();
        
        // Async functions are represented as coroutines in MIR
        matches!(ty.kind(), TyKind::Coroutine(_, _))
    }

    /// Analyze async function
    pub fn analyze_async_fn(&mut self, body: &Body<'tcx>) -> AsyncContext {
        let mut context = AsyncContext {
            is_async: false,
            suspension_points: Vec::new(),
            requires_send: false,
            requires_sync: false,
        };

        // Check if this is a coroutine (async function)
        // Note: In newer rustc, coroutine_kind is not directly accessible from Body
        // We'll check for Yield terminators to identify async points
        
        // Find all yield/await points
        for (bb_idx, bb_data) in body.basic_blocks.iter_enumerated() {
            if let Some(ref terminator) = bb_data.terminator {
                if let TerminatorKind::Yield { .. } = terminator.kind {
                    context.is_async = true;
                    // This is an await point
                            let suspension_point = SuspensionPoint {
                                location: bb_idx.index(),
                                line_number: 0, // Would need source map lookup
                                variables_live: Vec::new(), // Would need liveness analysis
                            };
                            context.suspension_points.push(suspension_point);
                        }
                    }
                }

                // Check Send/Sync requirements
                // This is simplified - full implementation would check trait bounds
                context.requires_send = true; // Most async functions require Send
                context.requires_sync = false;
            }
        }

        context
    }

    /// Check if a variable is live across an await point
    pub fn is_live_across_await(&self, _var_id: VariableId, _await_point: usize) -> bool {
        // TODO: Implement proper liveness analysis
        // This would require dataflow analysis to determine which variables
        // are live at each suspension point
        false
    }

    /// Get variables that must be Send for an async function
    pub fn get_send_required_vars(&self, body: &Body<'tcx>) -> HashSet<Local> {
        let mut send_vars = HashSet::new();

        // Any variable live across an await point must be Send
        // Check for Yield terminators to identify async functions
        let has_yields = body.basic_blocks.iter().any(|bb_data| {
            bb_data.terminator.as_ref()
                .map_or(false, |t| matches!(t.kind, TerminatorKind::Yield { .. }))
        });
        
        if has_yields {
            // Simplified: assume all locals might need to be Send
            for local in body.local_decls.indices() {
                send_vars.insert(local);
            }
        }

        send_vars
    }

    /// Analyze generator state machine
    pub fn analyze_generator_states(&self, body: &Body<'tcx>) -> Vec<usize> {
        let mut states = Vec::new();

        // Each suspension point creates a new state
        for (bb_idx, bb_data) in body.basic_blocks.iter_enumerated() {
            if let Some(ref terminator) = bb_data.terminator {
                if matches!(terminator.kind, TerminatorKind::Yield { .. }) {
                    states.push(bb_idx.index());
                }
            }
        }

        states
    }

    /// Check if a type implements Send
    pub fn is_send(&self, def_id: DefId) -> bool {
        let ty = self.tcx.type_of(def_id).instantiate_identity();
        
        // Simplified check - full implementation would query trait system
        // Most primitive types and references are Send
        match ty.kind() {
            TyKind::Int(_) | TyKind::Uint(_) | TyKind::Float(_) | TyKind::Bool | TyKind::Char => true,
            TyKind::Ref(_, _, _) => true,
            _ => false, // Conservative: assume not Send unless proven
        }
    }

    /// Check if a type implements Sync
    pub fn is_sync(&self, def_id: DefId) -> bool {
        let ty = self.tcx.type_of(def_id).instantiate_identity();
        
        // Similar to is_send, but for Sync trait
        match ty.kind() {
            TyKind::Int(_) | TyKind::Uint(_) | TyKind::Float(_) | TyKind::Bool | TyKind::Char => true,
            TyKind::Ref(_, _, mutability) => {
                // &T is Sync if T is Sync
                // &mut T is Sync if T is Sync
                matches!(mutability, rustc_middle::mir::Mutability::Not)
            }
            _ => false,
        }
    }
}
