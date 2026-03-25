//! MIR visitor for extracting ownership events

use ownsight_core::*;
use std::collections::HashMap;

extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_hir;

use rustc_middle::mir::{self, Body, Local, Place, Rvalue, StatementKind, TerminatorKind, ProjectionElem};
use rustc_middle::ty::TyCtxt;
use rustc_span::def_id::DefId;
use rustc_span::Span as RustcSpan;

use crate::partial_move::PartialMoveAnalyzer;
use crate::lifetime::LifetimeAnalyzer;

/// Visitor for traversing MIR and extracting ownership events
pub struct MirVisitor<'tcx, 'a> {
    tcx: TyCtxt<'tcx>,
    analysis: &'a mut ProgramAnalysis,
    mode: AnalysisMode,
    def_id: DefId,
    
    // Mapping from MIR locals to our variable IDs
    local_to_var: HashMap<Local, VariableId>,
    var_counter: usize,
    event_counter: usize,
}

impl<'tcx, 'a> MirVisitor<'tcx, 'a> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        analysis: &'a mut ProgramAnalysis,
        mode: AnalysisMode,
        def_id: DefId,
    ) -> Self {
        Self {
            tcx,
            analysis,
            mode,
            def_id,
            local_to_var: HashMap::new(),
            var_counter: 0,
            event_counter: 0,
        }
    }

    pub fn visit_body(&mut self, body: &Body<'tcx>) {
        // First, create variables for all locals
        for (local, local_decl) in body.local_decls.iter_enumerated() {
            self.create_variable_for_local(local, local_decl, body);
        }

        // Then visit all basic blocks
        for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
            self.visit_basic_block(bb, bb_data, body);
        }
    }

    fn create_variable_for_local(
        &mut self,
        local: Local,
        local_decl: &mir::LocalDecl<'tcx>,
        _body: &Body<'tcx>,
    ) {
        let var_id = VariableId(self.var_counter);
        self.var_counter += 1;

        let ty_str = format!("{:?}", local_decl.ty);
        let name = if local.as_usize() == 0 {
            "_return".to_string()
        } else {
            format!("_{}", local.as_usize())
        };

        let span = self.convert_span(local_decl.source_info.span);
        
        let variable = Variable {
            id: var_id,
            name: name.clone(),
            ty: ty_str,
            scope_id: ScopeId(0), // TODO: proper scope tracking
            span: span.clone(),
            is_mutable: local_decl.mutability == mir::Mutability::Mut,
        };

        self.local_to_var.insert(local, var_id);
        self.analysis.variables.push(variable);

        // Create a StorageLive event
        let event = Event::new(
            EventId(self.event_counter),
            EventKind::StorageLive,
            var_id,
            span.clone(),
            span.start_line,
            format!("Variable `{}` storage allocated", name),
        );
        self.event_counter += 1;
        self.analysis.events.push(event);
    }

    fn visit_basic_block(
        &mut self,
        _bb: mir::BasicBlock,
        bb_data: &mir::BasicBlockData<'tcx>,
        _body: &Body<'tcx>,
    ) {
        // Visit all statements in the block
        for statement in &bb_data.statements {
            self.visit_statement(statement);
        }

        // Visit the terminator
        if let Some(ref terminator) = bb_data.terminator {
            self.visit_terminator(terminator);
        }
    }

    fn visit_statement(&mut self, statement: &mir::Statement<'tcx>) {
        match &statement.kind {
            StatementKind::Assign(box (place, rvalue)) => {
                self.visit_assign(place, rvalue, statement.source_info.span);
            }
            StatementKind::StorageLive(local) => {
                self.visit_storage_live(*local, statement.source_info.span);
            }
            StatementKind::StorageDead(local) => {
                self.visit_storage_dead(*local, statement.source_info.span);
            }
            _ => {}
        }
    }

    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, span: RustcSpan) {
        let target_var = self.get_var_for_place(place);
        
        match rvalue {
            Rvalue::Use(operand) => {
                self.visit_use_operand(target_var, operand, span);
            }
            Rvalue::Ref(_, borrow_kind, borrowed_place) => {
                self.visit_borrow(target_var, borrow_kind, borrowed_place, span);
            }
            Rvalue::RawPtr(mutability, place) => {
                // Raw pointer creation
                let var_id = self.get_or_create_variable(&place);
                
                let event = OwnershipEvent {
                    event_type: if matches!(mutability, rustc_middle::mir::Mutability::Mut) {
                        EventType::BorrowMut
                    } else {
                        EventType::BorrowShared
                    },
                    variable: var_id,
                    location: self.get_location(statement.source_info.span),
                    metadata: EventMetadata::default(),
                };
                
                self.analysis.add_event(event);
            }
            _ => {
                // Other rvalues - create a generic event
                if let Some(var_id) = target_var {
                    let event = Event::new(
                        EventId(self.event_counter),
                        EventKind::Use,
                        var_id,
                        self.convert_span(span),
                        self.get_line_number(span),
                        format!("Assignment to variable"),
                    );
                    self.event_counter += 1;
                    self.analysis.events.push(event);
                }
            }
        }
    }

    fn visit_use_operand(
        &mut self,
        target_var: Option<VariableId>,
        operand: &mir::Operand<'tcx>,
        span: RustcSpan,
    ) {
        match operand {
            mir::Operand::Move(place) => {
                let source_var = self.get_var_for_place(place);
                if let (Some(src), Some(tgt)) = (source_var, target_var) {
                    // This is a move
                    let event = Event::new(
                        EventId(self.event_counter),
                        EventKind::MoveOut,
                        src,
                        self.convert_span(span),
                        self.get_line_number(span),
                        format!("Value moved out"),
                    ).with_related(tgt);
                    self.event_counter += 1;
                    self.analysis.events.push(event);

                    let event = Event::new(
                        EventId(self.event_counter),
                        EventKind::MoveIn,
                        tgt,
                        self.convert_span(span),
                        self.get_line_number(span),
                        format!("Value moved in"),
                    );
                    self.event_counter += 1;
                    self.analysis.events.push(event);
                }
            }
            mir::Operand::Copy(place) => {
                let source_var = self.get_var_for_place(place);
                if let Some(src) = source_var {
                    let event = Event::new(
                        EventId(self.event_counter),
                        EventKind::Use,
                        src,
                        self.convert_span(span),
                        self.get_line_number(span),
                        format!("Value copied"),
                    );
                    self.event_counter += 1;
                    self.analysis.events.push(event);
                }
            }
            _ => {}
        }
    }

    fn visit_borrow(
        &mut self,
        target_var: Option<VariableId>,
        borrow_kind: &mir::BorrowKind,
        borrowed_place: &Place<'tcx>,
        span: RustcSpan,
    ) {
        let borrowed_var = self.get_var_for_place(borrowed_place);
        
        if let (Some(borrowed), Some(target)) = (borrowed_var, target_var) {
            let (event_kind, desc) = match borrow_kind {
                mir::BorrowKind::Shared => (EventKind::BorrowShared, "immutably borrowed"),
                mir::BorrowKind::Mut { .. } => (EventKind::BorrowMut, "mutably borrowed"),
                _ => (EventKind::BorrowShared, "borrowed"),
            };

            let event = Event::new(
                EventId(self.event_counter),
                event_kind,
                borrowed,
                self.convert_span(span),
                self.get_line_number(span),
                format!("Variable {}", desc),
            ).with_related(target);
            self.event_counter += 1;
            self.analysis.events.push(event);
        }
    }

    fn visit_address_of(
        &mut self,
        target_var: Option<VariableId>,
        mutability: &mir::Mutability,
        place: &Place<'tcx>,
        span: RustcSpan,
    ) {
        let source_var = self.get_var_for_place(place);
        
        if let (Some(src), Some(tgt)) = (source_var, target_var) {
            let event_kind = match mutability {
                mir::Mutability::Not => EventKind::BorrowShared,
                mir::Mutability::Mut => EventKind::BorrowMut,
            };

            let event = Event::new(
                EventId(self.event_counter),
                event_kind,
                src,
                self.convert_span(span),
                self.get_line_number(span),
                format!("Raw pointer created"),
            ).with_related(tgt);
            self.event_counter += 1;
            self.analysis.events.push(event);
        }
    }

    fn visit_storage_live(&mut self, local: Local, span: RustcSpan) {
        if let Some(&var_id) = self.local_to_var.get(&local) {
            let event = Event::new(
                EventId(self.event_counter),
                EventKind::StorageLive,
                var_id,
                self.convert_span(span),
                self.get_line_number(span),
                format!("Storage allocated"),
            );
            self.event_counter += 1;
            self.analysis.events.push(event);
        }
    }

    fn visit_storage_dead(&mut self, local: Local, span: RustcSpan) {
        if let Some(&var_id) = self.local_to_var.get(&local) {
            let event = Event::new(
                EventId(self.event_counter),
                EventKind::StorageDead,
                var_id,
                self.convert_span(span),
                self.get_line_number(span),
                format!("Storage deallocated"),
            );
            self.event_counter += 1;
            self.analysis.events.push(event);
        }
    }

    fn visit_terminator(&mut self, terminator: &mir::Terminator<'tcx>) {
        match &terminator.kind {
            TerminatorKind::Call { func: _, args, destination, .. } => {
                // Visit arguments (potential moves)
                for arg in args {
                    if let mir::Operand::Move(place) = arg.node {
                        let var = self.get_var_for_place(&place);
                        if let Some(var_id) = var {
                            let event = Event::new(
                                EventId(self.event_counter),
                                EventKind::MoveOut,
                                var_id,
                                self.convert_span(terminator.source_info.span),
                                self.get_line_number(terminator.source_info.span),
                                format!("Value moved into function call"),
                            );
                            self.event_counter += 1;
                            self.analysis.events.push(event);
                        }
                    }
                }

                // Visit destination
                let dest_var = self.get_var_for_place(destination);
                if let Some(var_id) = dest_var {
                    let event = Event::new(
                        EventId(self.event_counter),
                        EventKind::MoveIn,
                        var_id,
                        self.convert_span(terminator.source_info.span),
                        self.get_line_number(terminator.source_info.span),
                        format!("Return value received"),
                    );
                    self.event_counter += 1;
                    self.analysis.events.push(event);
                }
            }
            TerminatorKind::Drop { place, .. } => {
                let var = self.get_var_for_place(place);
                if let Some(var_id) = var {
                    let event = Event::new(
                        EventId(self.event_counter),
                        EventKind::Drop,
                        var_id,
                        self.convert_span(terminator.source_info.span),
                        self.get_line_number(terminator.source_info.span),
                        format!("Value dropped"),
                    );
                    self.event_counter += 1;
                    self.analysis.events.push(event);
                }
            }
            _ => {}
        }
    }

    fn get_var_for_place(&self, place: &Place<'tcx>) -> Option<VariableId> {
        self.local_to_var.get(&place.local).copied()
    }

    fn convert_span(&self, span: RustcSpan) -> Span {
        let source_map = self.tcx.sess.source_map();
        let lo = source_map.lookup_char_pos(span.lo());
        let hi = source_map.lookup_char_pos(span.hi());

        Span::new(
            lo.file.name.to_string_lossy().to_string(),
            lo.line,
            lo.col.0,
            hi.line,
            hi.col.0,
        )
    }

    fn get_line_number(&self, span: RustcSpan) -> usize {
        let source_map = self.tcx.sess.source_map();
        let lo = source_map.lookup_char_pos(span.lo());
        lo.line
    }
}
