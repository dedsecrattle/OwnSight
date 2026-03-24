# CLI Tool Guide

## Installation

```bash
cargo install ownsight-cli
```

## Basic Usage

### Analyze a File

```bash
cargo ownership-viz --file example.rs
```

### Analyze from Stdin

```bash
echo 'fn main() { let s = String::from("hello"); }' | cargo ownership-viz --stdin
```

### Analyze with Specific Backend

```bash
# Simple backend (default)
cargo ownership-viz --file example.rs --backend simple

# MIR backend (requires nightly + rustc-dev)
cargo ownership-viz --file example.rs --backend mir
```

## Command-Line Options

### Required (one of)
- `--file <PATH>`: Analyze a Rust source file
- `--stdin`: Read code from standard input

### Optional
- `--mode <MODE>`: Analysis mode (`teaching` or `debug`)
- `--backend <BACKEND>`: Analysis backend (`simple` or `mir`)
- `--output <FORMAT>`: Output format (`json` or `timeline`)
- `--function <NAME>`: Analyze specific function (coming soon)

## Examples

### Teaching Mode
```bash
cargo ownership-viz --file example.rs --mode teaching
```

Output:
```
Step 1: Variable 's' created
  Line 2: let s = String::from("hello");
  Explanation: A new String is allocated on the heap

Step 2: Variable 's' moved to 's2'
  Line 3: let s2 = s;
  Explanation: Ownership transfers from s to s2
```

### Debug Mode
```bash
cargo ownership-viz --file example.rs --mode debug
```

Output:
```
Event 1: Create
  Variable: s (id: 0)
  Location: example.rs:2:9
  Type: String
  
Event 2: MoveOut
  Variable: s (id: 0)
  Location: example.rs:3:14
  Target: s2 (id: 1)
```

### JSON Output
```bash
cargo ownership-viz --file example.rs --output json
```

```json
{
  "variables": [
    {
      "id": 0,
      "name": "s",
      "type": "String",
      "location": { "line": 2, "column": 9 }
    }
  ],
  "events": [
    {
      "kind": "Create",
      "variable_id": 0,
      "location": { "line": 2, "column": 9 }
    }
  ]
}
```

## MIR Backend

### Prerequisites
```bash
rustup toolchain install nightly
rustup component add rustc-dev llvm-tools-preview --toolchain nightly
```

### Build with MIR
```bash
cargo +nightly build --release --features mir
```

### Use MIR Backend
```bash
cargo ownership-viz --file example.rs --backend mir
```

## Integration

### With Other Tools

#### Pipe to jq
```bash
cargo ownership-viz --file example.rs --output json | jq '.events'
```

#### Save to File
```bash
cargo ownership-viz --file example.rs > analysis.txt
```

#### Batch Processing
```bash
for file in src/*.rs; do
  echo "Analyzing $file"
  cargo ownership-viz --file "$file"
done
```

## Troubleshooting

### Command Not Found
```bash
# Ensure cargo bin is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Or use full path
~/.cargo/bin/cargo-ownership-viz --file example.rs
```

### MIR Backend Not Available
```bash
# Check if built with mir feature
cargo ownership-viz --version

# Rebuild with mir
cargo +nightly build --release --features mir
```

### Analysis Errors
- Ensure file exists and is readable
- Check for valid Rust syntax
- Try Simple backend first