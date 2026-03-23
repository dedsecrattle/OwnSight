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
    
    fn generate_explanation(&self, kind: &EventKind, _var_id: VariableId) -> String {
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
            EventKind::PartialMove => format!("Partial move of struct field"),
            EventKind::ClosureCapture => format!("Variable captured by closure"),
            EventKind::AwaitSuspend => format!("Async function suspended at await point"),
            EventKind::AwaitResume => format!("Async function resumed after await"),
            EventKind::TwoPhaseActivate => format!("Two-phase borrow activated"),
            EventKind::ReborrowShared => format!("Immutable reborrow"),
            EventKind::ReborrowMut => format!("Mutable reborrow"),
            EventKind::FieldAccess => format!("Field accessed"),
            EventKind::MethodCall => format!("Method called"),
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
            EventKind::PartialMove => {
                format!("Field of `{}` is partially moved. The struct is now partially moved", var_name)
            }
            EventKind::ClosureCapture => {
                format!("`{}` is captured by a closure", var_name)
            }
            EventKind::AwaitSuspend => {
                format!("Async function suspended at await point. `{}` must remain valid across suspension", var_name)
            }
            EventKind::AwaitResume => {
                format!("Async function resumed. `{}` is accessible again", var_name)
            }
            EventKind::TwoPhaseActivate => {
                format!("Two-phase borrow of `{}` is activated", var_name)
            }
            EventKind::ReborrowShared => {
                format!("Immutable reference to `{}` is reborrowed", var_name)
            }
            EventKind::ReborrowMut => {
                format!("Mutable reference to `{}` is reborrowed", var_name)
            }
            EventKind::FieldAccess => {
                format!("Field of `{}` is accessed", var_name)
            }
            EventKind::MethodCall => {
                format!("Method called on `{}`", var_name)
            }
        }
    }
}

impl Default for EventBuilder {
    fn default() -> Self {
        Self::new()
    }
}
