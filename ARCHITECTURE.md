# Ownsight Architecture

## Design Philosophy

Ownsight is built with a **two-layer architecture**:

- **Layer 1**: Exam and learning tool - simplified, clear explanations
- **Layer 2**: Real debugging and code intelligence - precise, compiler-backed analysis

The key principle: **The UI renders facts, it doesn't understand Rust.**

All Rust semantics are handled by the analysis engine, which produces a structured JSON model that any UI can consume.

## System Components

### 1. Core Analysis Engine (`ownsight-core`)

**Purpose**: The heart of the system. Produces ownership facts.

**Key Modules**:

- `model.rs` - Data structures for the entire system
  - `Variable`: name, type, scope, mutability
  - `Event`: ownership events (create, move, borrow, drop)
  - `Scope`: function, block, loop, if, match
  - `ProgramAnalysis`: complete analysis output
  - `OwnershipState`: valid/moved status at each line

- `events.rs` - Event generation and explanations
  - `EventBuilder`: creates events with auto-incrementing IDs
  - Human-readable explanations for teaching mode
  - Precise explanations for debug mode

- `graph.rs` - Ownership graph representation
  - Nodes: variables, references, functions, scopes
  - Edges: owns, borrows, mutably_borrows, moves_to, drops_at
  - Graph queries for visualization

- `analysis.rs` - High-level analysis interface
  - `Analyzer`: orchestrates analysis
  - Query methods: "why can't use?", "where moved?", "what borrows?"
  - Ownership state computation

**Output**: `ProgramAnalysis` struct serialized to JSON

### 2. Analysis Driver (`ownsight-driver`)

**Purpose**: Bridges between Rust source code and the core engine.

**Current Implementation**: `SimpleAnalyzer`

- Syntax-based parsing for MVP
- Handles basic ownership patterns:
  - `let` bindings
  - Function calls (moves)
  - Borrows (`&` and `&mut`)
  - Scope-based drops

**Future Enhancements**

- Enhanced pattern matching analysis
- Improved error detection
- Handle all Rust constructs accurately
- Support NLL, partial moves, closures, async

### 3. CLI Tool (`ownsight-cli`)

**Purpose**: Command-line interface for analysis.

**Commands**:

```bash
cargo ownership-viz --file <path>
cargo ownership-viz --stdin
cargo ownership-viz --output json|timeline|text
cargo ownership-viz --mode teaching|debug
```

**Output Formats**:

- `timeline`: Colored, step-by-step event list
- `json`: Structured `ProgramAnalysis` for tooling
- `text`: Summary of variables and graph

### 4. Desktop UI (`ui/`)

**Purpose**: Interactive visualization and debugging.

**Backend** (`src-tauri/`):

- Tauri commands expose analysis functions
- `analyze_snippet`: Analyze code from editor
- `analyze_file`: Analyze file from disk
- `query_*`: Answer debugging questions

**Frontend** (`src/`):

- React + TypeScript
- TailwindCSS for styling
- Monaco Editor for code editing
- React Flow for graph visualization

**Components**:

- `SourceView`: Code editor with line highlighting
- `TimelineView`: Step-by-step event list
- `GraphView`: Visual ownership graph
- `StepController`: Play/pause/navigate controls
- `QueryPanel`: Ask questions about ownership

## Data Flow

```
┌─────────────┐
│ Rust Source │
└──────┬──────┘
       │
       ▼
┌─────────────────┐
│ ownsight-driver │  (Parse & extract)
└──────┬──────────┘
       │
       ▼
┌─────────────┐
│ ownsight-   │  (Build model)
│ core        │
└──────┬──────┘
       │
       ▼
┌──────────────────┐
│ ProgramAnalysis  │  (JSON)
│ - variables      │
│ - events         │
│ - graph          │
│ - diagnostics    │
└──────┬───────────┘
       │
       ├──────────────┐
       │              │
       ▼              ▼
┌──────────┐   ┌──────────┐
│   CLI    │   │ Desktop  │
│  Output  │   │    UI    │
└──────────┘   └──────────┘
```

## Key Design Decisions

### 1. Separation of Analysis and Presentation

**Why**: Allows multiple UIs (CLI, desktop, web, IDE extension) to share the same analysis engine.

**How**: Core produces JSON, UIs consume it.

### 2. Event-Based Model

**Why**: Ownership changes are temporal - they happen at specific lines.

**How**: Each event has:

- Kind (create, move, borrow, drop)
- Variable ID
- Line number
- Explanation

### 3. Dual Views (Timeline + Graph)

**Why**: Different mental models for different use cases.

**Timeline**: Sequential, good for learning
**Graph**: Relational, good for debugging

### 4. Function-First Analysis

**Why**: Whole-program analysis doesn't scale.

**How**: Analyze one function at a time, cache results, compute summaries.

### 5. Teaching vs Debug Modes

**Why**: Students need clarity, engineers need precision.

**Teaching**: Simplified explanations, hides compiler complexity
**Debug**: Exact spans, branch-sensitive flow, compiler diagnostics

## Extensibility Points

### Adding New Event Types

1. Add variant to `EventKind` enum in `model.rs`
2. Add explanation in `EventBuilder::generate_detailed_explanation`
3. Add icon/color in UI components

### Adding New Queries

1. Add method to `Analyzer` in `analysis.rs`
2. Add Tauri command in `ui/src-tauri/commands.rs`
3. Add UI in `QueryPanel.tsx`

### Improving Analysis

**Short term**: Enhance `SimpleAnalyzer`

- Better pattern matching
- Handle more syntax forms
- Improve borrow tracking

**Long term**: Advanced analysis features

- Enhanced pattern matching
- Cross-file analysis

### Adding UI Features

All UI code is in `ui/src/`:

- New components in `components/`
- New types in `types/`
- Styling with TailwindCSS classes

## Performance Considerations

### Current (MVP)

- Single-threaded analysis
- No caching
- Full re-analysis on each change

### Future Optimizations

- Incremental analysis (only re-analyze changed functions)
- Parallel analysis of independent functions
- Result caching with content hashing
- Lazy graph generation

## Testing Strategy

### Unit Tests

- Core model operations
- Event generation
- Graph building
- Query methods

### Integration Tests

- End-to-end analysis of snippets
- CLI output validation
- Tauri command testing

### Snapshot Tests

- Store expected analysis for test cases
- Use `insta` crate for snapshot testing
- Test cases in `tests/snapshots/`

## Future Architecture

### Multi-Crate Support

```
Cargo Workspace
    ↓
cargo_metadata (discover crates)
    ↓
Analyze each crate
    ↓
Build inter-crate summaries
    ↓
Combined analysis
```

### IDE Integration

```
VS Code Extension
    ↓
Rust Analyzer (LSP)
    ↓
Ownsight Analysis
    ↓
Inline decorations + hover info
```

## Security Considerations

- Desktop app runs user code through analysis
- No code execution, only parsing
- Tauri sandboxing for web content
- File system access limited to user-selected files

## Deployment

### CLI

- Publish to crates.io
- Install with `cargo install ownsight-cli`

### Desktop App

- Build with `bun run tauri build`
- Distribute platform-specific bundles
- Auto-update with Tauri updater

### Web Version (Future)

- Compile analysis engine to WASM
- Run in browser
- No server needed for basic analysis

## Contributing Guidelines

1. **Core changes**: Ensure backward compatibility of JSON format
2. **UI changes**: Follow existing component patterns
3. **Analysis changes**: Add test cases in `tests/snapshots/`
4. **Documentation**: Update this file for architectural changes

## References

- [Tauri Architecture](https://tauri.app/v1/references/architecture/)
- [React Flow](https://reactflow.dev/learn)
