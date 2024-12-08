# Mutable Borrowing Error in Rust

This repository demonstrates a common error in Rust related to mutable references.  The code attempts to create multiple mutable references to the same variable, which is not allowed in Rust due to its borrowing rules. This is designed to prevent data races and ensure memory safety.