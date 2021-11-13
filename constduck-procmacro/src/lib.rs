use syn::{parse_macro_input, DeriveInput};

mod derive;

#[proc_macro_derive(ConstDuck)]
pub fn derive_field_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive::gen(&input).into()
}
