use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(DodsSoA)]
pub fn dods_soa_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let soa_name = format_ident!("{}SoA", name);
    let soa_ref_name = format_ident!("{}Ref", name);
    let soa_ref_mut_name = format_ident!("{}RefMut", name);

    let fields = match input.data {
        Data::Struct(ref s) => match s.fields {
            Fields::Named(ref f) => &f.named,
            _ => panic!("DodsSoA only supports structs with named fields"),
        },
        _ => panic!("DodsSoA only supports structs"),
    };

    let field_idents: Vec<_> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    // Use the first field to check lengths/emptiness for the whole SoA
    let first_field = field_idents[0];

    let iter_parts: Vec<_> = field_idents
        .iter()
        .map(|ident| quote! { self.#ident.iter() })
        .collect();
    let iter_mut_parts: Vec<_> = field_idents
        .iter()
        .map(|ident| quote! { self.#ident.iter_mut() })
        .collect();

    let mut iter_expr = iter_parts[0].clone();
    for part in iter_parts.iter().skip(1) {
        iter_expr = quote! { #iter_expr.zip(#part) };
    }

    let mut iter_mut_expr = iter_mut_parts[0].clone();
    for part in iter_mut_parts.iter().skip(1) {
        iter_mut_expr = quote! { #iter_mut_expr.zip(#part) };
    }

    let mut tuple_pattern = quote! { #first_field };
    for ident in field_idents.iter().skip(1) {
        tuple_pattern = quote! { (#tuple_pattern, #ident) };
    }

    let expanded = quote! {
        pub struct #soa_name {
            #( pub #field_idents: Vec<#field_types>, )*
        }

        pub struct #soa_ref_name<'a> {
            #( pub #field_idents: &'a #field_types, )*
        }

        pub struct #soa_ref_mut_name<'a> {
            #( pub #field_idents: &'a mut #field_types, )*
        }

        impl #soa_name {
            pub fn new() -> Self {
                Self {
                    #( #field_idents: Vec::new(), )*
                }
            }

            pub fn push(&mut self, item: #name) {
                #( self.#field_idents.push(item.#field_idents); )*
            }

            /// Removes the element at `index` in O(1) time.
            /// It swaps the element at `index` with the last element and then pops.
            /// Note: This does NOT preserve the order of elements.
            pub fn swap_remove(&mut self, index: usize) -> #name {
                #name {
                    #( #field_idents: self.#field_idents.swap_remove(index), )*
                }
            }

            pub fn pop(&mut self) -> Option<#name> {
                if self.#first_field.is_empty() { return None; }
                Some(#name {
                    #( #field_idents: self.#field_idents.pop().unwrap(), )*
                })
            }

            pub fn len(&self) -> usize {
                self.#first_field.len()
            }

            pub fn get(&self, index: usize) -> Option<#soa_ref_name<'_>> {
                if index >= self.#first_field.len() { return None; }
                Some(#soa_ref_name {
                    #( #field_idents: &self.#field_idents[index], )*
                })
            }

            pub fn iter(&self) -> impl Iterator<Item = #soa_ref_name<'_>> {
                #iter_expr.map(|#tuple_pattern| #soa_ref_name {
                    #( #field_idents: #field_idents, )*
                })
            }

            pub fn iter_mut(&mut self) -> impl Iterator<Item = #soa_ref_mut_name<'_>> {
                #iter_mut_expr.map(|#tuple_pattern| #soa_ref_mut_name {
                    #( #field_idents: #field_idents, )*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
