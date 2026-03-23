//! Partial move analysis (struct fields, enum variants)

use ownsight_core::*;
use std::collections::HashMap;

extern crate rustc_middle;

use rustc_middle::ty::TyCtxt;
use rustc_middle::mir::{Body, Place, PlaceElem, ProjectionElem, Local};

/// Partial move information
#[derive(Debug, Clone)]
pub struct PartialMoveInfo {
    pub base_var: VariableId,
    pub field_path: Vec<String>,
    pub moved_fields: Vec<String>,
    pub remaining_fields: Vec<String>,
}

/// Partial move tracker
pub struct PartialMoveAnalyzer<'tcx> {
    tcx: TyCtxt<'tcx>,
    partial_moves: HashMap<VariableId, PartialMoveInfo>,
}

impl<'tcx> PartialMoveAnalyzer<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self {
            tcx,
            partial_moves: HashMap::new(),
        }
    }

    /// Extract field path from a place projection
    pub fn extract_field_path(&self, place: &Place<'tcx>) -> Vec<String> {
        let mut path = Vec::new();
        
        for elem in place.projection.iter() {
            match elem {
                ProjectionElem::Field(field, _) => {
                    path.push(format!("field_{}", field.index()));
                }
                ProjectionElem::Index(_) => {
                    path.push("[]".to_string());
                }
                ProjectionElem::Downcast(Some(variant), _) => {
                    path.push(variant.to_string());
                }
                _ => {}
            }
        }
        
        path
    }

    /// Track a partial move from a place
    pub fn track_partial_move(&mut self, place: &Place<'tcx>, var_id: VariableId) {
        let field_path = self.extract_field_path(place);
        
        if !field_path.is_empty() {
            // This is a field access - track as partial move
            let info = self.partial_moves.entry(var_id).or_insert_with(|| {
                PartialMoveInfo {
                    base_var: var_id,
                    field_path: Vec::new(),
                    moved_fields: Vec::new(),
                    remaining_fields: Vec::new(),
                }
            });
            
            let field_name = field_path.join(".");
            if !info.moved_fields.contains(&field_name) {
                info.moved_fields.push(field_name);
            }
        }
    }

    /// Check if a variable is partially moved
    pub fn is_partially_moved(&self, var_id: VariableId) -> bool {
        self.partial_moves.contains_key(&var_id)
    }

    /// Get partial move info
    pub fn get_partial_move_info(&self, var_id: VariableId) -> Option<&PartialMoveInfo> {
        self.partial_moves.get(&var_id)
    }

    /// Analyze body for partial moves
    pub fn analyze_body(&mut self, body: &Body<'tcx>) {
        // Traverse all statements looking for moves of field projections
        for (_, bb_data) in body.basic_blocks.iter_enumerated() {
            for statement in &bb_data.statements {
                if let mir::StatementKind::Assign(box (place, rvalue)) = &statement.kind {
                    // Check if rvalue is a move from a field
                    if let mir::Rvalue::Use(operand) = rvalue {
                        if let mir::Operand::Move(source_place) = operand {
                            if !source_place.projection.is_empty() {
                                // This is a move from a projection (field access)
                                // Track it as a partial move
                                // Note: We'd need the var_id mapping here
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Check if a place represents a field access
    pub fn is_field_access(&self, place: &Place<'tcx>) -> bool {
        place.projection.iter().any(|elem| {
            matches!(elem, ProjectionElem::Field(_, _))
        })
    }
}
