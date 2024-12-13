# Unexpected Behavior when Modifying Vector via Raw Pointer in Rust

This repository demonstrates a potential issue when working with raw pointers and vectors in Rust. The code in `bug.rs` shows how modifying a vector's value via its raw pointer does not always update the vector, potentially leading to unexpected behavior.

The solution in `bugSolution.rs` shows a safer approach using vector indexing.