# Event Types Reference

Complete reference for all ownership event types in Ownsight.

## Layer 1 Events (Simple Backend)

### Create
Variable is created and initialized.

```rust
let s = String::from("hello");
// Event: Create (s)
```

### MoveOut
Ownership moves from one variable to another.

```rust
let s1 = String::from("hello");
let s2 = s1;
// Event: MoveOut (s1 → s2)
```

### BorrowShared
Immutable borrow is created.

```rust
let s = String::from("hello");
let r = &s;
// Event: BorrowShared (s → r)
```

### BorrowMut
Mutable borrow is created.

```rust
let mut s = String::from("hello");
let r = &mut s;
// Event: BorrowMut (s → r)
```

### Use
Variable is used (read).

```rust
let s = String::from("hello");
println!("{}", s);
// Event: Use (s)
```

### Drop
Variable is dropped and memory freed.

```rust
{
    let s = String::from("hello");
}  // Event: Drop (s)
```

## Layer 2 Events (MIR Backend)

### PartialMove
Only part of a struct is moved.

```rust
struct Point { x: i32, y: i32 }
let p = Point { x: 1, y: 2 };
let x = p.x;
// Event: PartialMove (p.x)
```

### ClosureCapture
Variable is captured by a closure.

```rust
let s = String::from("hello");
let f = || println!("{}", s);
// Event: ClosureCapture (s, ByRef)
```

**Capture Modes:**
- `ByValue`: `move ||`
- `ByRef`: `||` (immutable)
- `ByMutRef`: `||` (mutable)

### AwaitSuspend
Async function suspends at await point.

```rust
async fn example() {
    let s = String::from("hello");
    fetch().await;
    // Event: AwaitSuspend
}
```

### AwaitResume
Async function resumes after await.

```rust
async fn example() {
    fetch().await;
    // Event: AwaitResume
}
```

### TwoPhaseActivate
Two-phase borrow is activated.

```rust
let mut v = vec![1, 2, 3];
v.push(v.len());
// Event: TwoPhaseActivate
```

### ReborrowShared
Immutable reborrow from existing borrow.

```rust
let s = String::from("hello");
let r1 = &s;
let r2 = &*r1;
// Event: ReborrowShared (r1 → r2)
```

### ReborrowMut
Mutable reborrow from existing borrow.

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut *r1;
// Event: ReborrowMut (r1 → r2)
```

### FieldAccess
Struct field is accessed.

```rust
struct Point { x: i32, y: i32 }
let p = Point { x: 1, y: 2 };
let x = p.x;
// Event: FieldAccess (p.x)
```

### MethodCall
Method is called on a value.

```rust
let mut s = String::from("hello");
s.push_str(" world");
// Event: MethodCall (s.push_str)
```

## Event Properties

### Common Fields

All events have:
- `kind`: Event type
- `location`: Source location (line, column)
- `variable_id`: Affected variable
- `explanation`: Human-readable description

### Layer 2 Additional Fields

MIR events may include:
- `mir_location`: Basic block and statement index
- `lifetime`: Lifetime region information
- `capture_mode`: For closure captures
- `field_path`: For partial moves

## Event Ordering

Events are ordered chronologically by:
1. Line number
2. Column number
3. Statement order (MIR)

## Filtering Events

### By Type
```rust
analysis.events.iter()
    .filter(|e| matches!(e.kind, EventKind::MoveOut))
```

### By Variable
```rust
analysis.events.iter()
    .filter(|e| e.variable_id == target_var)
```

### By Location
```rust
analysis.events.iter()
    .filter(|e| e.location.line == target_line)
```

## Event Explanations

### Teaching Mode
Simplified, beginner-friendly explanations.

Example:
```
"Variable 's' is moved to 's2'. After this, 's' can no longer be used."
```

### Debug Mode
Detailed technical information.

Example:
```
"MoveOut: Variable 's' (id: 0) ownership transferred to 's2' (id: 1) at line 3, column 14"
```

## Custom Events

To add custom events:

1. Define in `EventKind` enum
2. Implement detection logic
3. Add explanation generation
4. Update UI rendering

See [Layer 2 Development](../development/layer2.md) for details.