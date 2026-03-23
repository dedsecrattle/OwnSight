export interface ProgramAnalysis {
  files: SourceFile[];
  functions: FunctionInfo[];
  variables: Variable[];
  scopes: Scope[];
  events: Event[];
  ownership_graph: OwnershipGraph;
  diagnostics: Diagnostic[];
  metadata: AnalysisMetadata;
}

export interface SourceFile {
  path: string;
  content: string;
  lines: string[];
}

export interface Variable {
  id: VariableId;
  name: string;
  ty: string;
  scope_id: ScopeId;
  span: Span;
  is_mutable: boolean;
}

export interface VariableId {
  0: number;
}

export interface ScopeId {
  0: number;
}

export interface FunctionId {
  0: number;
}

export interface EventId {
  0: number;
}

export interface Span {
  file: string;
  start_line: number;
  start_col: number;
  end_line: number;
  end_col: number;
}

export type EventKind =
  | "Create"
  | "MoveOut"
  | "MoveIn"
  | "BorrowShared"
  | "BorrowMut"
  | "Reborrow"
  | "Use"
  | "Drop"
  | "StorageLive"
  | "StorageDead"
  | "Reinit"
  | "Conflict";

export interface Event {
  id: EventId;
  kind: EventKind;
  variable_id: VariableId;
  related_variable_id: VariableId | null;
  span: Span;
  explanation: string;
  line_number: number;
}

export interface Scope {
  id: ScopeId;
  parent: ScopeId | null;
  start_line: number;
  end_line: number;
  kind: ScopeKind;
}

export type ScopeKind = "Function" | "Block" | "Loop" | "If" | "Match";

export interface FunctionInfo {
  id: FunctionId;
  name: string;
  span: Span;
  parameters: Parameter[];
  return_type: string;
  summary: FunctionSummary | null;
}

export interface Parameter {
  name: string;
  ty: string;
  ownership_behavior: OwnershipBehavior;
}

export type OwnershipBehavior = "Consumes" | "SharedBorrow" | "MutableBorrow";

export interface FunctionSummary {
  consumes: string[];
  borrows_shared: string[];
  borrows_mut: string[];
  returns_ownership: boolean;
  references_escape: boolean;
}

export interface OwnershipGraph {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

export type GraphNode =
  | { Variable: VariableId }
  | { Reference: VariableId }
  | { Function: FunctionId }
  | { Scope: ScopeId };

export interface GraphEdge {
  source: GraphNode;
  target: GraphNode;
  kind: EdgeKind;
  label: string | null;
}

export type EdgeKind =
  | "Owns"
  | "Borrows"
  | "MutablyBorrows"
  | "MovesTo"
  | "Reborrows"
  | "DropsAt"
  | "LivesInScope";

export interface Diagnostic {
  level: DiagnosticLevel;
  message: string;
  span: Span;
  code: string | null;
  suggestion: string | null;
}

export type DiagnosticLevel = "Error" | "Warning" | "Note" | "Help";

export interface AnalysisMetadata {
  mode: AnalysisMode;
  timestamp: string;
  version: string;
}

export type AnalysisMode = "Teaching" | "Debug";
