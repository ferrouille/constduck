use constduck::{ConstDuck, ConstructFrom, WithField};

#[derive(ConstDuck, Debug)]
struct Foo {
    counter: u32,
    text: String,
}

macro_rules! create_struct {
    (@genmapping $firstname:ident: $firstvalue:expr, $($name:ident: $value:expr,)* { $($name2:ident: $value2:expr,)* }) => {
        #[allow(non_camel_case_types)]
        impl<$($name2,)*> WithField<{ stringify!($firstname) }> for WrapperStruct<$($name2,)*>
            where $firstname: Clone {
            type Output = $firstname;

            fn value(&self) -> Self::Output {
                self.$firstname.clone()
            }
        }

        create_struct!(@genmapping $($name: $value,)* { $($name2: $value2,)* });
    };
    (@genmapping { $($name2:ident: $value2:expr,)* }) => {};
    ($struct:ident; $($name:ident: $value:expr),*) => {{
        #[allow(non_camel_case_types)]
        struct WrapperStruct<$($name,)*> {
            $($name: $name,)*
        }

        create_struct!(@genmapping $($name: $value,)* { $($name: $value,)* });

        #[allow(non_camel_case_types)]
        impl<$($name,)*> From<WrapperStruct<$($name,)*>> for $struct
            where $struct: ConstructFrom<WrapperStruct<$($name,)*>> {
            fn from(wrapper: WrapperStruct<$($name,)*>) -> Self {
                // We cannot do this:
                //
                // $struct {
                //     counter: wrapper.counter,
                //     text: wrapper.text,
                // }
                //
                // Because wrapper.counter is not guaranteed to be a u32, and $struct might not contain the same fields.

                $struct::construct(wrapper)
            }
        }

        WrapperStruct {
            $($name: $value,)*
        }
    }}
}

fn main() {
    let foo = create_struct!(
        Foo;
        counter: 5u32,
        text: String::new()
    );

    let real_foo: Foo = foo.into();
    println!("Foo: {:?}", real_foo);
}
