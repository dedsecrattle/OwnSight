# Architecture

## Overview

Ownsight is built with a modular architecture supporting two analysis backends.

## System Architecture

```
┌─────────────────────────────────────────────────┐
│              User Interfaces                     │
│  ┌──────────────┐      ┌──────────────┐         │
│  │ Desktop App  │      │  CLI Tool    │         │
│  │ (Tauri+React)│      │  (Rust)      │         │
│  └──────┬───────┘      └──────┬───────┘         │
└─────────┼──────────────────────┼─────────────────┘
          │                      │
          └──────────┬───────────┘
                     │
          ┌──────────▼──────────┐
          │  ownsight-driver    │
          │  (Backend Factory)  │
          └──────────┬──────────┘
                     │
          ┌──────────┴──────────┐
          │                     │
    ┌─────▼──────┐      ┌──────▼──────┐
    │  Simple    │      │    MIR      │
    │  Backend   │      │  Backend    │
    │ (Layer 1)  │      │ (Layer 2)   │
    └─────┬──────┘      └──────┬──────┘
          │                     │
          └──────────┬──────────┘
                     │
          ┌──────────▼──────────┐
          │   ownsight-core     │
          │   (Data Model)      │
          └─────────────────────┘
```

## Core Components

### ownsight-core
Central data model shared by all components.

**Key Types:**
- `ProgramAnalysis`: Complete analysis result
- `Variable`: Variable information
- `OwnershipEvent`: Ownership state changes
- `EventKind`: Event types (Create, Move, Borrow, etc.)

### ownsight-driver
Factory for creating analyzers.

**Responsibilities:**
- Backend selection
- Analyzer instantiation
- Common interface (`OwnershipAnalyzer` trait)

### Simple Backend (Layer 1)
Syntax-based analysis using `syn` crate.

**Features:**
- Fast parsing
- Basic ownership tracking
- No compilation needed

**Limitations:**
- Syntax-level only
- No type information
- Limited pattern detection

### MIR Backend (Layer 2)
Compiler-based analysis using rustc internals.

**Features:**
- Compiler-accurate
- Full type information
- Advanced patterns (partial moves, closures, async)

**Requirements:**
- Nightly Rust
- rustc-dev component
- Compilation needed

## Data Flow

### Analysis Pipeline

```
Source Code
    │
    ▼
┌─────────────┐
│   Parser    │
│ (syn/rustc) │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Analyzer   │
│  (Backend)  │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Events    │
│ Generation  │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ ProgramAnalysis │
└─────────────┘
```

### Event Generation

1. **Parse** source code
2. **Traverse** AST/MIR
3. **Detect** ownership patterns
4. **Generate** events with explanations
5. **Build** timeline

## Desktop App Architecture

### Frontend (React)
- Monaco editor for code input
- Timeline and graph visualizations
- Query interface
- Step controller

### Backend (Tauri)
- Rust commands for analysis
- File system access
- Native performance

### Communication
- Tauri IPC for command invocation
- JSON serialization for data transfer

## Extension Points

### Adding New Event Types

1. Add to `EventKind` enum in `ownsight-core`
2. Implement detection in analyzer
3. Add explanation generation
4. Update UI to display

### Adding New Analyzers

1. Implement `OwnershipAnalyzer` trait
2. Add to `AnalyzerBackend` enum
3. Update factory in `ownsight-driver`
4. Add UI option

### Adding New Visualizations

1. Create React component
2. Process `ProgramAnalysis` data
3. Add to view selector
4. Update UI layout

## Performance Considerations

### Simple Backend
- O(n) parsing with `syn`
- Minimal memory usage
- Instant results

### MIR Backend
- Compilation overhead
- Higher memory usage
- More accurate results

## Security

- No network access required
- Local analysis only
- No data collection
- Open source and auditable