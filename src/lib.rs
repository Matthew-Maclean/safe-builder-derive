#![allow(dead_code)]

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

#[cfg(test)]
mod test;

use proc_macro::TokenStream;

#[proc_macro_derive(SafeBuilder)]
pub fn safe_builder(input: TokenStream) -> TokenStream
{
    unimplemented!()
}

use syn::{Body, VariantData, Ty};

#[derive(PartialEq, Debug)]
struct CompleteStruct
{
    name: String,
    fields: Vec<(String, Ty)>,
}

impl CompleteStruct
{
    pub fn new(input: &syn::MacroInput) -> CompleteStruct
    {
        assert!(input.generics.lifetimes.len() == 0, "safe-builder-derive does not support lifetime parameters");
        assert!(input.generics.ty_params.len() == 0, "safe-builder-derive does not support generic type parameters");
        
        let name = input.ident.to_string();

        if let Body::Struct(VariantData::Struct(ref fields)) = input.body
        {
            match fields.first()
            {
                None => CompleteStruct
                {
                    name: name,
                    fields: vec![]
                },
                Some(ref field) => match field.ident
                {
                    Some(_) => CompleteStruct
                    {
                        name: name,
                        fields: fields.iter()
                            .map(|field| (field.ident.clone().unwrap().to_string(), field.ty.clone()))
                            .collect()
                    },
                    None => panic!("safe-builder-derive does not support tuple structs")
                }
            }
        }
        else
        {
            panic!("safe-builder-derive does not support enums");
        }
    }
}

struct PartialStruct
{
    name: String,
    contained: Vec<(String, Ty)>,
    remaining: Vec<(String, Ty)>,
}