[![crates.io](https://img.shields.io/crates/v/constduck.svg)](https://crates.io/crates/constduck/) [![docs](https://img.shields.io/badge/docs-rs.svg)](https://docs.rs/constduck/)

# `constduck`: compile-time duck typing and reflection
`constduck` provides a procmacro that can enable compile time duck typing and reflection on arbitrary struct types. It supports three main features:

* Accessing fields of any struct, using the field name
* Constructing instances from a mapping of fields to values
* Reflecting over field types at compile time

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

## Accessing fields
When deriving `ConstDuck`, the trait `Field<"fieldname">` is implemented for all fields of the struct. You can use the trait to write generic implementations. For example:

```rust
fn deduct_money<N, T: Field<"money", Ty = N>>(t: &mut T) 
    where N: Clone,
        N: Sub<N, Output = N>,
        N: From<i8> {
    t.set(t.get().clone() - N::from(5i8));
}
```

`deduct_money` will work for any struct that has a field `money` and derives `ConstDuck`.

You should just write a custom trait for the example above. It's not always possible to write a custom trait. For example, consider the following macro:

```rust
macro_rules! make_getter {
    ($struct:ident.$field:ident) => {
        impl $struct {
            pub fn $field(&self) -> &/* What to write here? */ {
                &self.$field
            }
        }
    }
}

struct Foo {
    bar: String,
    baz: u32,
}

make_getter!(Foo.bar);
```

In this case the function definition requires a return type, but you don't have enough information to specify the type. Using `constduck`, you can write this macro as:

```rust
macro_rules! make_getter {
    ($struct:ident.$field:ident) => {
        impl<T> $struct
            where Self: Field<{ stringify!($field) }, Ty = T> {
            pub fn $field(&self) -> &T {
                <Self as Field<{ stringify!($field) }>>::get(self)
            }
        }
    }
}

#[derive(ConstDuck)]
struct Foo {
    bar: String,
    baz: u32,
}

make_getter!(Foo.bar);
```

## Constructing instances
If you're storing arbitrary expressions in a new struct in a macro, you cannot use the type of the expression for the field. For example:

```rust
macro_rules! create_struct {
    ($struct:ident; $($name:ident: $value:expr),*) => {{
        // For some good reason we want to generate a separate wrapper struct first and return that instead of collecting directly into `$struct`.
        struct WrapperStruct<$($name,)*> {
            $($name: $name,)*
        }

        impl<$($name,)*> From<WrapperStruct<$($name,)*>> for $struct {
            fn from(wrapper: WrapperStruct<$($name,)*>) -> Self {
                // wrapper.counter is not guaranteed to be a u32
                // So we cannot do this:
                $struct {
                    counter: wrapper.counter,
                    text: wrapper.text,
                }
            }
        }

        WrapperStruct {
            $($name: $value,)*
        }
    }}
}
```

We cannot write a correct implementation for `From` here, because it's normally not possible to express "type must have two fields `counter: u32` and `text: String`" as a constraint.

By implementing `WithField` for the wrapper struct, we can write the implementation in the macro like this instead:

```rust
impl<$($name,)*> From<WrapperStruct<$($name,)*>> for $struct 
    where $struct: ConstructFrom<WrapperStruct<$($name,)*>> {
    fn from(wrapper: WrapperStruct<$($name,)*>) -> Self {
        $struct::construct(wrapper)
    }
}
```

See [`constduck/examples/construct-instances.rs`](constduck/examples/construct-instances.rs) for a full example.
 
## Reflecting over field types at compile time

Using `ConstDuck::Reflect` you can implement traits for any type (like `#[derive(..)]`) without needing a procmacro. See [`constduck/examples/debug-print.rs`](constduck/examples/debug-print.rs) for an example.

# (In)stability
This project requires Rust nightly, and uses the incomplete `adt_const_params` feature. You might encounter ICEs.
The current API will likely break when support for tuple structs and enums is added.

# License
`constduck` is licensed under the Mozilla Public License 2.0 (MPL2.0). See the [`LICENSE`](LICENSE) file.