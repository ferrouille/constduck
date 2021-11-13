[![crates.io](https://img.shields.io/crates/v/constduck.svg)](https://crates.io/crates/constduck/) [![docs](https://img.shields.io/badge/docs-rs.svg)](https://docs.rs/constduck/)

# `constduck`: compile-time duck typing and reflection
`constduck` provides a procmacro that can enable compile time duck typing and reflection on arbitrary struct types. It allows you to auto-generate implementations for traits, like `#[derive(..)]`, without needing a procmacro. See [`constduck/examples/debug-print.rs`](constduck/examples/debug-print.rs) for an example. With `constduck`, you can also write generic implementations that work for any struct that has the right fields.

# Usage
Derive `ConstDuck` on a struct:
```rust
#![feature(adt_const_params)]
use constduck::*;

#[derive(ConstDuck)]
struct Donald {
    money: i64,
}
```

This implements `Field<"money">` and `ConstructFrom<T: WithField<"money">>` for the struct `Donald`. You can use these traits to write generic implementations. For example:

```rust
fn deduct_money<N, T: Field<"money", Ty = N>>(t: &mut T) 
    where N: Clone,
        N: Sub<N, Output = N>,
        N: From<i8> {
    t.set(t.get().clone() - N::from(5i8));
}
```

`deduct_money` will work for any struct that has a field `money` and derives `ConstDuck`.

## (In)stability
This project requires Rust nightly, and uses the incomplete `adt_const_params` feature. You might encounter ICEs.
The current API will likely break when support for tuple structs and enums is added.

# License
`constduck` is licensed under the Mozilla Public License 2.0 (MPL2.0). See the [`LICENSE`](LICENSE) file.