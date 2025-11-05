
# Day 1: Rust Ownership & Borrowing

## Concepts Learned
- **Ownership:** Each value has a single owner; moves invalidate the old owner.
- **Borrowing:** Access data without taking ownership. 
  - Immutable: multiple allowed
  - Mutable: only one allowed
- **Slices:** Borrow parts of a collection without copying.
- **Functions:** Passing by value moves ownership; passing by reference borrows.
- **Lifetimes:** Ensure references are valid; prevent dangling pointers.

## Notes
- Ownership rules prevent memory leaks.
- Borrowing and lifetimes make Rust safe for concurrency.
- Examples implemented in `/tests/rust_basics.rs`.
