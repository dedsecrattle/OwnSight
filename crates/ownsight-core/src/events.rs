use crate::model::{Event, EventId, EventKind, VariableId, Span};

pub struct EventBuilder {
    next_id: usize,
}

impl EventBuilder {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }
    
    fn next_id(&mut self) -> EventId {
        let id = EventId(self.next_id);
        self.next_id += 1;
        id
    }
    
    pub fn create_event(
        &mut self,
        kind: EventKind,
        variable_id: VariableId,
        span: Span,
        line_number: usize,
    ) -> Event {
        let explanation = self.generate_explanation(&kind, variable_id);
        Event::new(self.next_id(), kind, variable_id, span, line_number, explanation)
    }
    
    fn generate_explanation(&self, kind: &EventKind, var_id: VariableId) -> String {
        match kind {
            EventKind::Create => format!("Variable created"),
            EventKind::MoveOut => format!("Value moved out, variable no longer usable"),
            EventKind::MoveIn => format!("Value moved in"),
            EventKind::BorrowShared => format!("Immutable borrow created"),
            EventKind::BorrowMut => format!("Mutable borrow created"),
            EventKind::Reborrow => format!("Reference reborrowed"),
            EventKind::Use => format!("Variable used"),
            EventKind::Drop => format!("Value dropped"),
            EventKind::StorageLive => format!("Storage allocated"),
            EventKind::StorageDead => format!("Storage deallocated"),
            EventKind::Reinit => format!("Variable reinitialized"),
            EventKind::Conflict => format!("Borrow conflict detected"),
        }
    }
    
    pub fn create_detailed_event(
        &mut self,
        kind: EventKind,
        variable_id: VariableId,
        span: Span,
        line_number: usize,
        var_name: &str,
        custom_explanation: Option<String>,
    ) -> Event {
        let explanation = custom_explanation.unwrap_or_else(|| {
            self.generate_detailed_explanation(&kind, var_name)
        });
        Event::new(self.next_id(), kind, variable_id, span, line_number, explanation)
    }
    
    fn generate_detailed_explanation(&self, kind: &EventKind, var_name: &str) -> String {
        match kind {
            EventKind::Create => {
                format!("Variable `{}` is created and owns its value", var_name)
            }
            EventKind::MoveOut => {
                format!(
                    "`{}` was moved. You cannot use `{}` after this point unless it is reassigned",
                    var_name, var_name
                )
            }
            EventKind::MoveIn => {
                format!("Ownership of a value is moved into `{}`", var_name)
            }
            EventKind::BorrowShared => {
                format!(
                    "Immutable borrow of `{}` created. Multiple immutable borrows are allowed",
                    var_name
                )
            }
            EventKind::BorrowMut => {
                format!(
                    "Mutable borrow of `{}` created. No other borrows of `{}` are allowed while this is active",
                    var_name, var_name
                )
            }
            EventKind::Reborrow => {
                format!("Reference to `{}` is reborrowed", var_name)
            }
            EventKind::Use => {
                format!("`{}` is used here", var_name)
            }
            EventKind::Drop => {
                format!(
                    "`{}` is dropped and its memory is freed",
                    var_name
                )
            }
            EventKind::StorageLive => {
                format!("Memory allocated for `{}`", var_name)
            }
            EventKind::StorageDead => {
                format!("Memory for `{}` is deallocated", var_name)
            }
            EventKind::Reinit => {
                format!("`{}` is reinitialized with a new value", var_name)
            }
            EventKind::Conflict => {
                format!(
                    "Borrow conflict: cannot borrow `{}` because it is already borrowed",
                    var_name
                )
            }
        }
    }
}

impl Default for EventBuilder {
    fn default() -> Self {
        Self::new()
    }
}
