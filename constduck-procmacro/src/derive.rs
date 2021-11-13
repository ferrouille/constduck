use proc_macro2::{Punct, Spacing, Span, TokenStream, TokenTree};
use quote::{format_ident, ToTokens};
use syn::{Data, DeriveInput, LitStr};

pub fn gen(input: &DeriveInput) -> TokenStream {
    let ident = input.ident.clone();
    if let Data::Struct(s) = &input.data {
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        let mut tokens = TokenStream::new();
        for field in s.fields.iter() {
            let fieldident = field
                .ident
                .as_ref()
                .expect("All fields must have names")
                .clone();
            let fieldname = fieldident.to_string();
            let ty = field.ty.clone();
            tokens.extend(quote::quote! {
                #[automatically_derived]
                impl #impl_generics ::constduck::Field<#fieldname> for #ident #ty_generics #where_clause {
                    type Ty = #ty;

                    fn get(&self) -> &Self::Ty {
                        &self.#fieldident
                    }

                    fn get_consume(self) -> Self::Ty {
                        self.#fieldident
                    }

                    fn set(&mut self, new_value: Self::Ty) {
                        self.#fieldident = new_value;
                    }
                }
            });
        }

        let fieldidents = s
            .fields
            .iter()
            .map(|field| field.ident.as_ref().expect("All fields must have names"))
            .collect::<Vec<_>>();
        let fieldnames = s
            .fields
            .iter()
            .map(|field| field.ident.as_ref().expect("All fields must have names"))
            .map(|ident| LitStr::new(&ident.to_string(), Span::call_site()))
            .collect::<Vec<_>>();
        let fieldtypes = s
            .fields
            .iter()
            .map(|field| field.ty.clone())
            .collect::<Vec<_>>();
        let gs = s
            .fields
            .iter()
            .enumerate()
            .map(|(n, _)| format_ident!("__K{}", n))
            .collect::<Vec<_>>();

        let mut field_iter = quote::quote! { ::constduck::FieldListNil };
        for fieldname in fieldnames.iter() {
            field_iter = quote::quote! {
                ::constduck::FieldListCons<#fieldname, #field_iter>
            };
        }

        // Strip < and > from the generics
        let generics_base = impl_generics.to_token_stream();
        let mut gens = TokenStream::new();
        let mut first = true;
        let mut prev = None;
        for token in generics_base {
            if first {
                first = false;
            } else {
                if let Some(prev) = prev {
                    gens.extend([prev]);
                }

                prev = Some(token);
            }
        }

        if !gens.is_empty() {
            gens.extend([TokenTree::Punct(Punct::new(',', Spacing::Alone))]);
        }

        let structname = ident.to_string();
        tokens.extend(quote::quote! {
            impl<#gens #(#gs: Into<#fieldtypes>,)* __D: #(::constduck::WithField<#fieldnames, Output = #gs> +)*> ::constduck::ConstructFrom<__D> for #ident #ty_generics #where_clause {
                fn construct(data: __D) -> Self {
                    Self {
                        #(
                            #fieldidents: <__D as ::constduck::WithField<{ #fieldnames }>>::value(&data).into(),
                        )*
                    }
                }
            }

            #[automatically_derived]
            impl #impl_generics ::constduck::ConstDuck for #ident #ty_generics #where_clause {
                const NAME: &'static str = #structname;
                type Fields = #field_iter;
            }

            #[automatically_derived]
            impl<#gens __T> ::constduck::ConstDuckGeneric<__T> for #ident #ty_generics #where_clause {
                const NAME: &'static str = #structname;
                type Fields = #field_iter;
            }
        });

        tokens
    } else {
        unreachable!()
    }
}
