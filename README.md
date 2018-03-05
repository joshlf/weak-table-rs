# weak-table-rs: weak hash maps and sets for Rust

[![Build Status](https://travis-ci.org/tov/weak-table-rs.svg?branch=master)](https://travis-ci.org/tov/weak-table-rs)
[![Crates.io](https://img.shields.io/crates/v/weak-table.svg?maxAge=2592000)](https://crates.io/crates/weak-table)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)

This crate defines several kinds of weak hash maps and sets. See 
the [full API documentation](https://tov.github.io/weak-table-rs/).

## Usage

```rust
use weak_table::PtrWeakHashSet;
use std::sync::{Arc, Weak};

type Table = PtrWeakHashSet<Weak<String>>;

let mut set = Table::new();
let a = Arc::new("hello".to_string());
let b = Arc::new("hello".to_string());

set.insert(a.clone());

assert!(   set.contains(&a) );
assert!( ! set.contains(&b) );

set.insert(b.clone());

assert!(   set.contains(&a) );
assert!(   set.contains(&b) );
```
