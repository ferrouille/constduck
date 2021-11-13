#![allow(incomplete_features)]
#![feature(adt_const_params)]

//! `constduck` implements compile-time duck typing via const generics.
//! It only works on structs.
//!
//! **Note:** if you can, use a custom trait instead of `constduck`.
//! Only use `constduck` in cases where you can't implement a custom trait.
//!
//! You must `#[derive(ConstDuck)]` on all traits that you want to duck-type.
//! For example:
//! ```rust
//! # #![feature(adt_const_params)]
//! # use constduck::*;
//! #[derive(ConstDuck)]
//! struct Donald {
//!     money: i64,
//! }
//! ```
//!
//! This implements [`Field<"money">`](Field) and [`ConstructFrom<T: WithField<"money">>`](ConstructFrom) for `Donald`.
//!
//! You can use `constduck` to derive implementations for traits, like #[derive(..)].
//! See [constduck/examples/debug-print.rs] for an example.
//! The example implements a trait (`MyPrint`) for all types that derive `ConstDuck`.
//!
//! You can use [`Field::get`], [`Field::get_consume`] and [`Field::set`] to access the fields of the struct. For example:
//!
//! ```rust
//! # #![feature(adt_const_params)]
//! # use constduck::*;
//! use std::ops::Sub;
//!
//! #[derive(ConstDuck)]
//! struct Donald {
//!     money: i8,
//! }
//!
//! #[derive(ConstDuck)]
//! struct Scrooge {
//!     money: i64,
//! }
//!
//! let mut donald = Donald { money: 5 };
//! let mut scrooge = Scrooge { money: 1_000_000_000 };
//!
//! fn deduct_money<N, T: Field<"money", Ty = N>>(t: &mut T)
//!     where N: Clone,
//!         N: Sub<N, Output = N>,
//!         N: From<i8> {
//!     t.set(t.get().clone() - N::from(5i8));
//! }
//!
//! assert_eq!(donald.money, 5);
//! deduct_money(&mut donald);
//! assert_eq!(donald.money, 0);
//!
//! assert_eq!(scrooge.money, 1_000_000_000);
//! deduct_money(&mut scrooge);
//! assert_eq!(scrooge.money, 0_999_999_995);
//! ```
//!
//! The main use case for `constduck` is in macros.
//! You sometimes need to specify a type in a macro.
//! If you only know the type of the struct, it is normally impossible to obtain the type of a field.
//! With `constduck`, you can write this type using generics. For example:
//!
//! ```rust
//! # #![feature(adt_const_params)]
//! # use constduck::*;
//! macro_rules! make_getter {
//!     ($struct:ident.$field:ident) => {
//!         impl<T> $struct
//!             where Self: Field<{ stringify!($field) }, Ty = T> {
//!             pub fn $field(&self) -> &T {
//!                 <Self as Field<{ stringify!($field) }>>::get(self)
//!             }
//!         }
//!     }
//! }
//!
//! #[derive(ConstDuck)]
//! struct Foo {
//!     bar: String,
//!     baz: u32,
//! }
//!
//! make_getter!(Foo.bar);
//! ```

/// Derives [`Field`] for each field of a struct and [`ConstructFrom`].
/// `ConstructFrom<T>` is derived for `T`s that implement [`WithField`].
/// For example, if a struct `foo` has fields `bar` and `baz`,
/// `ConstructFrom<T>` will be implemented for all `T: WithField<"bar"> + WithField<"baz">`.
pub use constduck_procmacro::ConstDuck;

pub trait ConstDuck {
    const NAME: &'static str;
    type Fields: FieldList;
}

/// Identical to [`ConstDuck`], implemented for all `T`.
/// This trait can be helpful when implementing your own trait for all types that implement ConstDuck.
/// For example, let's say we want to implement a custom trait `MyTrait`.
/// We provide a custom implementation for `u32`: `impl MyTrait for u32 { .. }`.
/// Now we cannot implement `MyTrait` for all `T: ConstDuck` anymore.
/// We *can* still `impl<T: ConstDuckGeneric<ImplGuard>> MyTrait for T`, if `ImplGuard` is a local type.
///
/// I don't understand this well enough to explain why this works.
/// See [Rust's orphan rules](https://github.com/rust-lang/rfcs/blob/master/text/2451-re-rebalancing-coherence.md).
pub trait ConstDuckGeneric<T> {
    const NAME: &'static str;
    type Fields: FieldList;
}

/// A list of fields, available at compile time.
pub trait FieldList {}

/// The first entry in the list (`HEAD`) and the tail of the [`FieldList`].
pub struct FieldListCons<const HEAD: &'static str, T: FieldList>(T);

/// The end of the [`FieldList`].
pub struct FieldListNil;

impl<const HEAD: &'static str, T: FieldList> FieldList for FieldListCons<HEAD, T> {}
impl FieldList for FieldListNil {}

/// `Field<NAME>` is normally implemented for structs that have a field named `NAME`.
/// Access to the field's value is provided via [`Field::get`], [`Field::get_consume`] and [`Field::set`].
pub trait Field<const NAME: &'static str> {
    type Ty;

    fn get(&self) -> &Self::Ty;
    fn get_consume(self) -> Self::Ty;
    fn set(&mut self, value: Self::Ty);
}

/// `WithField<NAME>` indicates that a type can create a new value for a field named `NAME`.
/// This trait is used for [`ConstructFrom`]. See [`ConstDuck`].
pub trait WithField<const NAME: &'static str> {
    type Output;

    fn value(&self) -> Self::Output;
}

/// When a type implements `ConstructFrom<T>`, a new instance of the type can be constructed from a `T`.
pub trait ConstructFrom<T> {
    fn construct(t: T) -> Self;
}
