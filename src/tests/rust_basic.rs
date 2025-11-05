
// tests/rust_basics.rs
// Day 1: Rust Ownership & Borrowing

// ---------- Ownership ----------
#[test]
fn ownership_example() {
    let x = String::from("Hello");
    let y = x; // Ownership moves from x to y
    println!("{}", y); // ✅ Works
    // println!("{}", x); // ❌ Error: x is no longer valid
}

// ---------- Borrowing ----------
#[test]
fn borrowing_example() {
    let mut s = String::from("Hello");

    // Immutable borrow
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    // Mutable borrow
    let r3 = &mut s;
    r3.push_str(", Rust!");
    println!("{}", r3);

    // ❌ Cannot use r1 or r2 after mutable borrow
}

// ---------- Slices ----------
#[test]
fn slice_borrowing() {
    let v = vec![1, 2, 3, 4, 5];
    let slice = &v[1..3]; // Borrow part of vector
    println!("{:?}", slice);
}

// ---------- Ownership in Functions ----------
fn take_ownership(s: String) { println!("{}", s); }
fn borrow_string(s: &String) { println!("{}", s); }

#[test]
fn function_ownership() {
    let s1 = String::from("Blockchain");
    take_ownership(s1); // Ownership moved
    // println!("{}", s1); // ❌ Cannot use s1 anymore

    let s2 = String::from("Rust");
    borrow_string(&s2); // Borrowed, still usable
    println!("{}", s2); // ✅ Works fine
}

// ---------- Lifetimes ----------
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

#[test]
fn lifetime_example() {
    let s1 = "hello";
    let s2 = "world!";
    let result = longest(s1, s2);
    println!("Longest: {}", result);
}
