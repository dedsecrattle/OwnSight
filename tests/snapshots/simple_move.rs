fn main() {
    let s = String::from("hello");
    takes_ownership(s);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
