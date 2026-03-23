// Example demonstrating Layer 2 features

fn main() {
    // Basic ownership
    let s = String::from("hello");
    let r = &s;
    println!("{}", r);
    drop(s);
    
    // Partial move (will be detected by Layer 2)
    let point = Point { x: 10, y: 20 };
    let x_val = point.x; // Partial move
    // println!("{}", point.x); // Error: x was moved
    println!("{}", point.y); // OK: y still valid
    
    // Closure capture
    let value = String::from("captured");
    let closure = || {
        println!("{}", value); // Captured by reference
    };
    closure();
    
    // Async/await (requires async runtime)
    // async_example().await;
}

struct Point {
    x: i32,
    y: i32,
}

// Async function example
async fn async_example() {
    let data = String::from("async data");
    some_async_fn().await;
    println!("{}", data); // data must be valid across await
}

async fn some_async_fn() {
    // Simulate async work
}
