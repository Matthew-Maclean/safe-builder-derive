use syn;
use std::collections::HashMap;

use super::*;

macro_rules! syn_type {
    ($e: expr) => ({
        syn::Ty::Path(
            None,
            syn::Path
            {
                global: false,
                segments: vec![
                    syn::PathSegment
                    {
                        ident: syn::Ident::from($e),
                        parameters: syn::PathParameters::AngleBracketed(
                            syn::AngleBracketedParameterData
                            {
                                lifetimes: vec![],
                                types: vec![],
                                bindings: vec![]
                            }
                        )
                    }
                ]
            }
        )
    })
}

#[test]
fn test_complete_struct()
{
    unimplemented!()
}