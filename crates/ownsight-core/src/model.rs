use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VariableId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScopeId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub file: String,
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

impl Span {
    pub fn new(file: String, start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self { file, start_line, start_col, end_line, end_col }
    }
    
    pub fn single_line(file: String, line: usize, start_col: usize, end_col: usize) -> Self {
        Self::new(file, line, start_col, line, end_col)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub id: VariableId,
    pub name: String,
    pub ty: String,
    pub scope_id: ScopeId,
    pub span: Span,
    pub is_mutable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKind {
    // Layer 1 events
    Create,
    MoveOut,
    MoveIn,
    BorrowShared,
    BorrowMut,
    Reborrow,
    Use,
    Drop,
    StorageLive,
    StorageDead,
    Reinit,
    Conflict,
    
    // Layer 2 events
    PartialMove,
    ClosureCapture,
    AwaitSuspend,
    AwaitResume,
    TwoPhaseActivate,
    ReborrowShared,
    ReborrowMut,
    FieldAccess,
    MethodCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub kind: EventKind,
    pub variable_id: VariableId,
    pub related_variable_id: Option<VariableId>,
    pub span: Span,
    pub explanation: String,
    pub line_number: usize,
}

impl Event {
    pub fn new(
        id: EventId,
        kind: EventKind,
        variable_id: VariableId,
        span: Span,
        line_number: usize,
        explanation: String,
    ) -> Self {
        Self {
            id,
            kind,
            variable_id,
            related_variable_id: None,
            span,
            explanation,
            line_number,
        }
    }
    
    pub fn with_related(mut self, related: VariableId) -> Self {
        self.related_variable_id = Some(related);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scope {
    pub id: ScopeId,
    pub parent: Option<ScopeId>,
    pub start_line: usize,
    pub end_line: usize,
    pub kind: ScopeKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScopeKind {
    Function,
    Block,
    Loop,
    If,
    Match,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub id: FunctionId,
    pub name: String,
    pub span: Span,
    pub parameters: Vec<Parameter>,
    pub return_type: String,
    pub summary: Option<FunctionSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub ty: String,
    pub ownership_behavior: OwnershipBehavior,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OwnershipBehavior {
    Consumes,
    SharedBorrow,
    MutableBorrow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSummary {
    pub consumes: Vec<String>,
    pub borrows_shared: Vec<String>,
    pub borrows_mut: Vec<String>,
    pub returns_ownership: bool,
    pub references_escape: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFile {
    pub path: String,
    pub content: String,
    pub lines: Vec<String>,
}

impl SourceFile {
    pub fn new(path: String, content: String) -> Self {
        let lines = content.lines().map(|s| s.to_string()).collect();
        Self { path, content, lines }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub span: Span,
    pub code: Option<String>,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
    Help,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramAnalysis {
    pub files: Vec<SourceFile>,
    pub functions: Vec<FunctionInfo>,
    pub variables: Vec<Variable>,
    pub scopes: Vec<Scope>,
    pub events: Vec<Event>,
    pub ownership_graph: crate::graph::OwnershipGraph,
    pub diagnostics: Vec<Diagnostic>,
    pub metadata: AnalysisMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisMetadata {
    pub mode: AnalysisMode,
    pub timestamp: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalysisMode {
    Teaching,
    Debug,
}

impl ProgramAnalysis {
    pub fn new(mode: AnalysisMode) -> Self {
        Self {
            files: Vec::new(),
            functions: Vec::new(),
            variables: Vec::new(),
            scopes: Vec::new(),
            events: Vec::new(),
            ownership_graph: crate::graph::OwnershipGraph::new(),
            diagnostics: Vec::new(),
            metadata: AnalysisMetadata {
                mode,
                timestamp: chrono::Utc::now().to_rfc3339(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        }
    }
    
    pub fn get_variable(&self, id: VariableId) -> Option<&Variable> {
        self.variables.iter().find(|v| v.id == id)
    }
    
    pub fn get_events_for_variable(&self, id: VariableId) -> Vec<&Event> {
        self.events.iter()
            .filter(|e| e.variable_id == id || e.related_variable_id == Some(id))
            .collect()
    }
    
    pub fn get_events_at_line(&self, line: usize) -> Vec<&Event> {
        self.events.iter()
            .filter(|e| e.line_number == line)
            .collect()
    }
    
    pub fn get_ownership_state_at_line(&self, line: usize) -> OwnershipState {
        let mut state = OwnershipState::new();
        
        for event in self.events.iter().filter(|e| e.line_number <= line) {
            state.apply_event(event);
        }
        
        state
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipState {
    pub valid_variables: HashMap<VariableId, VariableState>,
    pub active_borrows: Vec<BorrowInfo>,
}

impl OwnershipState {
    pub fn new() -> Self {
        Self {
            valid_variables: HashMap::new(),
            active_borrows: Vec::new(),
        }
    }
    
    pub fn apply_event(&mut self, event: &Event) {
        match event.kind {
            EventKind::Create | EventKind::StorageLive => {
                self.valid_variables.insert(event.variable_id, VariableState::Valid);
            }
            EventKind::MoveOut => {
                self.valid_variables.insert(event.variable_id, VariableState::MovedOut);
            }
            EventKind::MoveIn => {
                self.valid_variables.insert(event.variable_id, VariableState::Valid);
            }
            EventKind::BorrowShared => {
                self.active_borrows.push(BorrowInfo {
                    borrowed_var: event.variable_id,
                    borrow_var: event.related_variable_id,
                    is_mutable: false,
                });
            }
            EventKind::BorrowMut => {
                self.active_borrows.push(BorrowInfo {
                    borrowed_var: event.variable_id,
                    borrow_var: event.related_variable_id,
                    is_mutable: true,
                });
            }
            EventKind::Drop | EventKind::StorageDead => {
                self.valid_variables.remove(&event.variable_id);
                self.active_borrows.retain(|b| b.borrowed_var != event.variable_id);
            }
            EventKind::Reinit => {
                self.valid_variables.insert(event.variable_id, VariableState::Valid);
            }
            _ => {}
        }
    }
    
    pub fn is_valid(&self, var_id: VariableId) -> bool {
        matches!(self.valid_variables.get(&var_id), Some(VariableState::Valid))
    }
    
    pub fn is_moved(&self, var_id: VariableId) -> bool {
        matches!(self.valid_variables.get(&var_id), Some(VariableState::MovedOut))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariableState {
    Valid,
    MovedOut,
    PartiallyMoved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowInfo {
    pub borrowed_var: VariableId,
    pub borrow_var: Option<VariableId>,
    pub is_mutable: bool,
}

// ============================================================================
// Future Data Structures
// ============================================================================

/// Location for advanced analysis features
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AdvancedLocation {
    pub index: usize,
    pub sub_index: usize,
}

/// Lifetime identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LifetimeId(pub usize);

/// Lifetime region information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lifetime {
    pub id: LifetimeId,
    pub name: Option<String>,
    pub region: Region,
}

/// Region bounds (start and end locations)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub start: MirLocation,
    pub end: MirLocation,
}

/// Closure capture mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaptureMode {
    ByValue,
    ByRef,
    ByMutRef,
}

/// Closure capture information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosureCapture {
    pub var_id: VariableId,
    pub capture_mode: CaptureMode,
    pub by_ref: bool,
}

/// Async context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncContext {
    pub is_async: bool,
    pub await_points: Vec<usize>,
    pub send_required: bool,
    pub sync_required: bool,
}

/// Partial move information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialMoveInfo {
    pub base_var: VariableId,
    pub field_path: Vec<String>,
    pub moved_fields: Vec<String>,
}
