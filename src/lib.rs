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

