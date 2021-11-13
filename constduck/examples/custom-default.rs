#![allow(incomplete_features)]
#![feature(adt_const_params)]

use constduck::{ConstDuck, ConstructFrom, Field, WithField};
use std::marker::PhantomData;

#[derive(PartialEq, Debug)]
struct Money(i64);

#[derive(PartialEq, Debug, ConstDuck)]
struct Donald {
    name: String,
    money: Money,
}

#[derive(PartialEq, Debug, ConstDuck)]
struct Pond {
    d1: Donald,
    d2: Donald,
    d3: Donald,
}

#[derive(ConstDuck)]
struct Foo<T: Into<String>> {
    t: T,
}

// A custom implementation for `Default`.
pub trait MyDefault {
    fn default() -> Self;
}

// Helper struct.
pub struct CreateDefault<T>(PhantomData<T>);

// `CreateDefault` implements `WithField` for all possible fields.
// All fields must be types for which `MyDefault` is also implemented.
impl<T, const NAME: &'static str> WithField<NAME> for CreateDefault<T>
where
    T: Field<NAME>,
    T::Ty: MyDefault,
{
    type Output = T::Ty;

    fn value(&self) -> Self::Output {
        // To obtain a value for the field, we invoke `MyDefault::default`.
        T::Ty::default()
    }
}

// MyDefault 'leaf' implementations for `String` and `Money`
impl MyDefault for String {
    fn default() -> Self {
        "".to_string()
    }
}

impl MyDefault for Money {
    fn default() -> Self {
        Money(5)
    }
}

// A MyDefault implementation for any type that implements `CreateDefault<T>`.
// Any type that derives `ConstDuck` implements `CreateDefault<T>` for `T: WithField<FIELD1> + WithField<FIELD2> + ...`.
impl<T: ConstructFrom<CreateDefault<T>>> MyDefault for T {
    fn default() -> Self {
        Self::construct(CreateDefault(PhantomData))
    }
}

pub fn main() {
    println!("Duckburg: {:#?}", <Pond as MyDefault>::default());
}

#[cfg(test)]
mod tests {
    use crate::*;
    use constduck::Field;

    #[test]
    fn field_access() {
        let duck = Donald {
            name: "Donald".to_string(),
            money: Money(-42),
        };

        assert_eq!(<Donald as Field<"name">>::get(&duck), "Donald");
        assert_eq!(<Donald as Field<"money">>::get(&duck), &Money(-42));
        assert_eq!(<Donald as Field<"money">>::get_consume(duck), Money(-42));
    }

    #[test]
    fn my_default() {
        let duck = <Donald as MyDefault>::default();
        assert_eq!(
            duck,
            Donald {
                name: String::new(),
                money: Money(5),
            }
        );

        let pond = <Pond as MyDefault>::default();
        assert_eq!(
            pond,
            Pond {
                d1: Donald {
                    name: String::new(),
                    money: Money(5),
                },
                d2: Donald {
                    name: String::new(),
                    money: Money(5),
                },
                d3: Donald {
                    name: String::new(),
                    money: Money(5),
                },
            }
        );
    }
}
