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

#[derive(Debug, Clone, PartialEq)]
struct TargetStruct
{
    name: String,
    fields: HashMap<String, Ty>,
    partials: Partials,
}

impl TargetStruct
{
    pub fn new(input: &syn::MacroInput) -> TargetStruct
    {
        assert!(input.generics.lifetimes.len() == 0, "safe-builder-derive does not support lifetimes");
        assert!(input.generics.ty_params.len() == 0, "safe-builder-derive does not support generic types");

        let name = input.ident.to_string();

        if let Body::Struct(VariantData::Struct(ref fields)) = input.body
        {
            match fields.first()
            {
                None => TargetStruct
                {
                    name: name,
                    fields: HashMap::new(),
                    partials: Partials::new(Vec::new())
                },
                Some(ref first) => match first.ident
                {
                    Some(_) =>
                    {
                        let mut map = HashMap::new();
                        let mut field_names = Vec::new();

                        for field in fields.iter()
                        {
                            let name = field.ident.clone().unwrap().to_string();

                            field_names.push(name.clone());
                            map.insert(name, field.ty.clone());
                        }

                        let field_combinations = (0..fields.len() + 1) // + 1 because .. ranges are inclusive..exclusive
                            .flat_map(|n| field_names.iter()
                                .combinations(n)
                                .map(|v| v.into_iter()
                                    .map(|s| s.to_owned())
                                    .collect::<Vec<_>>()))
                            .map(|mut v| {v.sort(); v});
                        
                        let mut names = Vec::new();
                        let mut partials = Vec::new();

                        for combo in field_combinations
                        {
                            let mut name = format!("{}BuilderWith{}", name,
                                combo.iter().fold(String::new(), |mut a, b|
                                {
                                    a.push_str(b);
                                    a
                                }));
                            
                            while names.contains(&name)
                            {
                                name.push('_'); // TODO: better way to make names unique
                            }

                            names.push(name.clone());

                            partials.push(PartialStruct::new(name, combo));
                        }

                        TargetStruct
                        {
                            name: name,
                            fields: map,
                            partials: Partials::new(partials)
                        }
                    }
                    None => panic!("safe-builder-derive does not support tuple struct")
                }
            }
        }
        else
        {
            panic!("safe-builder-derive does not support enums");
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Partials(HashMap<usize, Vec<PartialStruct>>);

impl Partials
{
    pub fn new(partials: Vec<PartialStruct>) -> Partials
    {
        let mut map: HashMap<usize, Vec<PartialStruct>> = HashMap::new();

        for partial in partials.into_iter()
        {
            let o = partial.order();

            if let Some(_) = map.get_mut(&o)
            {
                map.get_mut(&o).unwrap().push(partial);
            }
            else
            {
                map.insert(o, vec![partial]);
            }
        }

        Partials(map)
    }

    pub fn at_order<'a>(&'a self, order: usize) -> Option<&'a [PartialStruct]>
    {
        match self.0.get(&order)
        {
            Some(vec) => Some(&vec),
            None => None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PartialStruct
{
    name: String,
    fields: Vec<String>,
}

impl PartialStruct
{
    pub fn new(name: String, fields: Vec<String>) -> PartialStruct
    {
        PartialStruct
        {
            name: name,
            fields: fields
        }
    }

    pub fn order(&self) -> usize
    {
        self.fields.len()
    }

    pub fn step<'a>(&self, other: &'a PartialStruct) -> Option<&'a String>
    {
        if self.order() == other.order() - 1 && other.order() != 0
        {
            Some(&other.fields[other.fields.len() - 1])
        }
        else
        {
            None
        }
    }

    pub fn build(&self, target: &TargetStruct) -> quote::Tokens
    {
        let self_id = quote::Ident::from(self.name.as_str());

        let partial_struct =
        {
            let fields = self.fields.iter()
                .map(|name|
                {
                    let id = quote::Ident::from(name.as_str());
                    let ty = target.fields.get(name).unwrap();

                    quote!
                    {
                        #id: #ty
                    }
                })
                .collect::<Vec<_>>();

            quote!
            {
                pub struct #self_id
                {
                    #(#fields),*
                }

                impl ::safe_builder::PartialBuilder for #self_id { }
            }
        };

        let partial_steps = if self.fields.len() == target.fields.len()
        {
            let id = quote::Ident::from(self.name.as_str());

            let steps = target.partials.at_order(self.order() + 1).unwrap().iter()
                .map(|partial|
                {
                    let step = self.step(partial).unwrap().clone();

                    let step_id = quote::Ident::from(step.as_str());
                    let step_ty = target.fields.get(&step).unwrap();

                    let step_struct = quote::Ident::from(partial.name.as_str());

                    let step_field = quote!
                    {
                        #id: #id
                    };

                    let fields = self.fields.iter()
                        .map(|name|
                        {
                            let id = quote::Ident::from(name.as_str());

                            quote!
                            {
                                #id: self.#id
                            }
                        });

                    quote!
                    {
                        fn #step_id(self, #step_id: #step_ty) -> #step_struct
                        {
                            #step_struct
                            {
                                #(#fields),*
                                #step_field
                            }
                        }
                    }
                });

            quote!
            {
                impl #id
                {
                    #(#steps)*
                }
            }
        }
        else
        {
            let target = quote::Ident::from(target.name.as_str());

            let fields = self.fields.iter()
                .map(|name|
                {
                    let id = quote::Ident::from(name.as_str());

                    quote!
                    {
                        #id: self.#id
                    }
                });

            quote!
            {
                fn build(self) -> #target
                {
                    #target
                    {
                        #(#fields),*
                    }
                }
            }
        };

        quote!
        {
            #partial_struct

            impl #self_id
            {
                #partial_steps
            }
        }
    }
}