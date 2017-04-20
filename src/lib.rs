#![allow(dead_code)]

extern crate proc_macro;
extern crate syn;
extern crate itertools;

#[macro_use]
extern crate quote;

#[cfg(test)]
mod test;

use proc_macro::TokenStream;
use itertools::Itertools;

#[proc_macro_derive(SafeBuilder)]
pub fn safe_builder(input: TokenStream) -> TokenStream
{
    unimplemented!()
}

use syn::{Body, VariantData, Ty};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct CompleteStruct
{
    name: String,
    fields: HashMap<String, Ty>,
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
                    fields: HashMap::new()
                },
                Some(ref field) => match field.ident
                {
                    Some(_) =>
                    {
                        let mut h = HashMap::new();

                        for field in fields.iter()
                        {
                            h.insert(field.ident.clone().unwrap().to_string(), field.ty.clone());
                        }



                        CompleteStruct
                        {
                            name: name,
                            fields: h,
                        }
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

    pub fn partials(&self) -> HashMap<Vec<String>, String>
    {
        let partials = (0..self.fields.len() + 1)
            .flat_map(|n| self.fields.keys()
                .combinations(n)
                .map(|c| c.into_iter()
                    .map(|s| s.to_owned())
                    .collect::<Vec<_>>()))
            .map(|mut partial| { partial.sort(); partial });
        
        let mut names = HashMap::new();
        let mut values = Vec::new();

        for partial in partials
        {
            let mut name = format!("{}BuilderWith{}", self.name,
                partial.iter().fold(String::new(), |mut acc, item| { acc.push_str(item); acc}));
            
            while values.contains(&name)
            {
                name.push('_'); // TODO: a better way to make names unique?
            }

            values.push(name.clone());

            names.insert(partial, name);
        }

        names
    }
}