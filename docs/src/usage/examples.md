# Examples

## Basic Ownership

### Simple Move
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2
    println!("{}", s2);
    // println!("{}", s1);  // Error: value borrowed after move
}
```

**What happens:**
1. `s1` is created with a String
2. Ownership moves from `s1` to `s2`
3. `s1` is no longer valid
4. `s2` is dropped at end of scope

### Clone Instead of Move
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Deep copy
    println!("{}", s1);   // OK: s1 still valid
    println!("{}", s2);   // OK: s2 has its own copy
}
```

## Borrowing

### Immutable Borrow
```rust
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);  // Borrow s
    println!("Length of '{}' is {}", s, len);  // s still valid
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### Mutable Borrow
```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);  // Prints "hello, world"
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

### Multiple Immutable Borrows
```rust
fn main() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);  // OK: multiple immutable borrows
}
```

### Mutable Borrow Rules
```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s;  // Error: cannot borrow as mutable more than once
    println!("{}", r1);
}
```

## Lifetimes

### Function with Lifetime
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("long string");
    let s2 = String::from("short");
    let result = longest(&s1, &s2);
    println!("Longest: {}", result);
}
```

### Struct with Lifetime
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", excerpt.part);
}
```

## Advanced Patterns (Layer 2)

### Partial Moves
```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let x = p.x;  // Partial move
    // println!("{}", p.y);  // Error: p is partially moved
}
```

### Closure Captures
```rust
fn main() {
    let s = String::from("hello");
    
    // By reference
    let print = || println!("{}", s);
    print();
    println!("{}", s);  // OK: s still valid
    
    // By value
    let consume = move || drop(s);
    consume();
    // println!("{}", s);  // Error: s was moved
}
```

### Async/Await
```rust
async fn fetch_data() -> String {
    String::from("data")
}

async fn process() {
    let data = String::from("hello");
    let fetched = fetch_data().await;
    println!("{} {}", data, fetched);  // data still valid after await
}
```

## Common Mistakes

### Using After Move
```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);  // Error: value used after move
}

fn takes_ownership(s: String) {
    println!("{}", s);
}
```

**Fix:** Clone or borrow instead
```rust
fn main() {
    let s = String::from("hello");
    takes_reference(&s);
    println!("{}", s);  // OK
}

fn takes_reference(s: &String) {
    println!("{}", s);
}
```

### Dangling Reference
```rust
fn main() {
    let r;
    {
        let s = String::from("hello");
        r = &s;  // Error: s dropped while borrowed
    }
    // println!("{}", r);
}
```

**Fix:** Ensure lifetime is valid
```rust
fn main() {
    let s = String::from("hello");
    let r = &s;
    println!("{}", r);  // OK: s lives long enough
}
```