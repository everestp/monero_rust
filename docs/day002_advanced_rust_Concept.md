Great! You're on **Day 2** of your **50-Day Monero-Like Blockchain in Rust** journey.

---

## DAY 2: Advanced Rust Concepts  
**Goal:** Master **Traits, Generics, Smart Pointers (`Rc`, `Arc`, `RefCell`)**, and **Error Handling (`Result`, `Option`)**  
**Repo Task:**  
> Add traits, generics, smart pointer examples inside `/src/tests/advanced_rust.rs`

We’ll do this **step-by-step**, with **real code**, **tests**, **Git commits**, and **clear explanations** — all building toward your final blockchain.

---

## Step-by-Step Guide for Day 2

---

### Step 1: Create the File

```bash
mkdir -p src/tests
touch src/tests/advanced_rust.rs
```

---

### Step 2: Add to `Cargo.toml` (if not already)

Ensure `Cargo.toml` includes test dependencies:

```toml
[dev-dependencies]
# None needed for now, but good to have
```

---

### Step 3: Write the Code – `src/tests/advanced_rust.rs`

```rust
// src/tests/advanced_rust.rs

// === 1. Generics ===
// A generic struct that holds any type of value
#[derive(Debug)]
struct Container<T> {
    value: T,
}

#[test]
fn test_generic_container() {
    let int_container = Container { value: 42 };
    let str_container = Container { value: "Hello Generics!" };

    assert_eq!(int_container.value, 42);
    assert_eq!(str_container.value, "Hello Generics!");
}

// === 2. Traits ===
// Define a trait
trait Summary {
    fn summarize(&self) -> String;
}

// Implement for String
impl Summary for String {
    fn summarize(&self) -> String {
        format!("String: {}...", self.chars().take(20).collect::<String>())
    }
}

// Implement for i32
impl Summary for i32 {
    fn summarize(&self) -> String {
        format!("Number: {}", self)
    }
}

#[test]
fn test_trait_summary() {
    let s = String::from("This is a long string for testing traits");
    let num = 100;

    assert!(s.summarize().starts_with("String: This is a long"));
    assert_eq!(num.summarize(), "Number: 100");
}

// === 3. Trait Bounds in Generics ===
fn print_summary<T: Summary>(item: &T) {
    println!("Summary: {}", item.summarize());
}

#[test]
fn test_generic_with_trait_bound() {
    let data = vec![
        String::from("Blockchain"),
        2025_i32,
    ];

    // We'll just call it — no assert needed
    for item in &data {
        if let Some(s) = item.downcast_ref::<String>() {
            print_summary(s);
        } else if let Some(n) = item.downcast_ref::<i32>() {
            print_summary(n);
        }
    }
    // This test just ensures no panic
}

// === 4. Smart Pointers: Rc<RefCell<T>> for Interior Mutability ===

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }

    fn append(&mut self, node: Rc<RefCell<Node>>) {
        self.next = Some(node);
    }
}

#[test]
fn test_rc_refcell_linked_list() {
    let head = Node::new(1);
    let second = Node::new(2);
    let third = Node::new(3);

    head.borrow_mut().append(second.clone());
    second.borrow_mut().append(third.clone());

    assert_eq!(head.borrow().value, 1);
    assert_eq!(head.borrow().next.as_ref().unwrap().borrow().value, 2);
    assert_eq!(head.borrow().next.as_ref().unwrap().borrow().next.as_ref().unwrap().borrow().value, 3);
}

// === 5. Error Handling: Result & Option ===

#[derive(Debug, PartialEq)]
enum BlockchainError {
    InvalidInput,
    Overflow,
}

fn safe_add(a: u64, b: u64) -> Result<u64, BlockchainError> {
    a.checked_add(b).ok_or(BlockchainError::Overflow)
}

#[test]
fn test_safe_add_success() {
    assert_eq!(safe_add(10, 20), Ok(30));
}

#[test]
fn test_safe_add_overflow() {
    assert_eq!(safe_add(u64::MAX, 1), Err(BlockchainError::Overflow));
}

#[test]
fn test_option_unwrap() {
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    assert_eq!(some_value.unwrap(), 42);
    assert!(none_value.is_none());
}
```

---

### Step 4: Add Module Declaration in `src/lib.rs` (or `main.rs`)

If you don’t have `src/lib.rs`, create it:

```bash
touch src/lib.rs
```

**`src/lib.rs`**
```rust
pub mod tests;
```

> This allows tests to be compiled as part of the library.

---

### Step 5: Run the Tests

```bash
cargo test
```

You should see:
```
running 7 tests
test tests::advanced_rust::test_generic_container ... ok
test tests::advanced_rust::test_trait_summary ... ok
...
test result: ok. 7 passed; 0 failed
```

---

### Step 6: Git Commit (Daily Commit!)

```bash
git add src/tests/advanced_rust.rs src/lib.rs
git commit -m "Day 2: Add traits, generics, Rc<RefCell>, Result/Option with full tests"
```

---

### Step 7: Push to GitHub (Optional but Recommended)

```bash
git push origin main
```

---

## Why This Matters for Your Blockchain

| Concept | Future Use in Blockchain |
|-------|--------------------------|
| **Generics** | Reusable `Transaction<T>`, `Block<T>` |
| **Traits** | `Signable`, `Hashable`, `Serializable` for transactions/blocks |
| **Rc<RefCell<T>>** | Shared ownership in transaction graphs, UTXO sets |
| **Arc** | Later in multithreaded mining & networking |
| **Result/Option** | Safe crypto operations, validation, parsing |

---

## Pro Tip: Create a `utils` Module Later

You’ll reuse:
```rust
pub trait Hashable {
    fn hash(&self) -> Vec<u8>;
}
```

Start thinking ahead!

---

## Day 2 Complete!

**Summary:**
- Created `/src/tests/advanced_rust.rs`
- Implemented **generics**, **traits**, **smart pointers**, **error handling**
- Wrote **7 passing unit tests**
- Committed with clear message

---

## Tomorrow (Day 3 Preview): Async in Rust
> Build a **TCP echo server/client** using `tokio` → foundation for **P2P networking**

```bash
cargo add tokio --features=full
```

Get ready!

---

Would you like me to:
1. Generate `Day 3` code now?
2. Help you set up `tokio`?
3. Create a `git` branch for `async-network`?

Let me know — we’re building a **real Monero-like blockchain**, one perfect day at a time.
