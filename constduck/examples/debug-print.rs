#![allow(incomplete_features)]
#![feature(adt_const_params)]

use constduck::{ConstDuck, ConstDuckGeneric, Field, FieldList, FieldListCons, FieldListNil};
use std::fmt;

// Some boring formatting stuff
#[derive(Copy, Clone)]
struct Indent {
    n: usize,
}

impl Indent {
    pub fn next(&self) -> Indent {
        Indent { n: self.n + 1 }
    }

    pub fn new() -> Indent {
        Indent { n: 0 }
    }
}

impl fmt::Display for Indent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..self.n {
            write!(f, "    ")?;
        }

        Ok(())
    }
}

// The trait we're going to implement.
// MyPrint will simply print all the fields of a struct to stdout, recursively.
trait MyPrint {
    fn print(&self, indent: Indent);
}

// Implementations for some basic types
impl MyPrint for &'static str {
    fn print(&self, indent: Indent) {
        println!("{}&'static str = {:?}", indent, self);
    }
}

impl MyPrint for u32 {
    fn print(&self, indent: Indent) {
        println!("{}u32 = {}", indent, self);
    }
}

// Implementation guard to make sure nobody else can implement conflicting implementations.
struct ImplGuard;

// Helper trait for printing fields in a field list
trait PrintField<O> {
    fn print(obj: &O, indent: Indent);
}

impl<O> PrintField<O> for FieldListNil {
    fn print(_: &O, _: Indent) {}
}

// `MyPrint` implementation for a field list
impl<O, const NAME: &'static str, T: FieldList> PrintField<O> for FieldListCons<NAME, T>
where
    O: Field<NAME>,
    O::Ty: MyPrint,
    T: PrintField<O>,
{
    fn print(obj: &O, indent: Indent) {
        println!("{}Field `{}`", indent, NAME);
        obj.get().print(indent.next());
        T::print(obj, indent);
    }
}

// `MyPrint` implementation for all types that derive `ConstDuck`.
impl<T: ConstDuckGeneric<ImplGuard>> MyPrint for T
where
    <T as ConstDuckGeneric<ImplGuard>>::Fields: PrintField<T>,
{
    fn print(&self, indent: Indent) {
        println!("{}struct `{}`:", indent, T::NAME);

        <T::Fields as PrintField<T>>::print(&self, indent.next())
    }
}

fn main() {
    // Derive ConstDuck for some types
    #[derive(ConstDuck)]
    struct Foo {
        bar: Bar,
        baz: Baz,
    }

    #[derive(ConstDuck)]
    struct Bar {
        life: u32,
        is_like: &'static str,
        a: u32,
    }

    #[derive(ConstDuck)]
    struct Baz {
        hurricane: &'static str,
    }

    // Use MyPrint for a simple type
    let baz = Baz { hurricane: "Hello" };
    baz.print(Indent::new());

    // Use MyPrint for a more complex type
    let foo = Foo {
        bar: Bar {
            life: 16,
            is_like: "Hello, world!",
            a: 100_234,
        },
        baz: Baz {
            hurricane: "Hello, world!",
        },
    };
    foo.print(Indent::new());
}
