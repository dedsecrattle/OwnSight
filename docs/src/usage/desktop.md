# Desktop App Guide

## Interface Overview

The Ownsight desktop app provides an interactive environment for understanding Rust ownership.

### Main Components

#### 1. Top Bar
- **Mode Selector**: Switch between Teaching and Debug modes
- **Backend Selector**: Choose Simple or MIR backend
- **Analyze Button**: Run analysis on your code

#### 2. Source Editor (Left)
- Monaco editor with Rust syntax highlighting
- Line numbers and code folding
- Highlights active lines during playback

#### 3. Visualization Panel (Right)
- **Timeline View**: Chronological event list
- **Graph View**: Visual ownership graph
- Toggle between views with tabs

#### 4. Step Controller (Bottom)
- Play/pause button
- Previous/next step buttons
- Progress slider
- Current step indicator

#### 5. Query Panel (Bottom)
- Interactive query interface
- Ask questions about ownership
- Get contextual explanations

## Features

### Teaching Mode
- Simplified explanations
- Focus on core concepts
- Great for learning

### Debug Mode
- Detailed technical information
- Line numbers and locations
- Useful for debugging

### Backend Selection

#### Simple Backend (Default)
- ✅ Fast analysis
- ✅ No compilation needed
- ✅ Works offline
- ❌ Limited to basic patterns

#### MIR Backend (Layer 2)
- ✅ Compiler-accurate
- ✅ Advanced features
- ✅ Partial moves, closures, async
- ❌ Requires nightly Rust
- ❌ Slower analysis

See [Layer 2: MIR Backend](layer2.md) for setup.

## Keyboard Shortcuts

- `Ctrl/Cmd + Enter`: Analyze code
- `Space`: Play/pause
- `←/→`: Previous/next step
- `Ctrl/Cmd + S`: Save code (coming soon)

## Tips

1. **Start Simple**: Use Teaching mode when learning
2. **Step Through**: Don't rush - understand each event
3. **Ask Questions**: Use the query panel
4. **Try Examples**: Modify example code to experiment
5. **Compare Backends**: See how Simple vs MIR differ

## Troubleshooting

### Analysis Fails
- Check for syntax errors
- Ensure code is valid Rust
- Try Simple backend first

### MIR Backend Unavailable
- Click the info icon (ℹ️) next to backend selector
- Follow the setup guide
- Or use Simple backend

### Slow Performance
- Use Simple backend for quick checks
- MIR backend is slower but more accurate
- Close other applications if needed