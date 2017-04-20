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
    let input =
    "
    struct S
    {
        a: A,
        b: B,
        c: C,
        d: D,
    }
    ";

    let macro_input = syn::parse_macro_input(input).unwrap();

    let actual = CompleteStruct::new(&macro_input);

    let expected = CompleteStruct
    {
        name: "S".to_owned(),
        fields:
        {
            let mut h = HashMap::new();

            h.insert("a".to_owned(), syn_type!("A"));
            h.insert("b".to_owned(), syn_type!("B"));
            h.insert("c".to_owned(), syn_type!("C"));
            h.insert("d".to_owned(), syn_type!("D"));
            h
        }
    };

    assert_eq!(expected, actual);

    let partials = actual.partials();

    assert!(partials.len() == 16);
}