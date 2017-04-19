use syn;

use super::*;

#[test]
fn test_complete_struct()
{
    let input =
    "
    struct S
    {
        a: A,
        b: B,
    }
    ";

    let macro_input = syn::parse_macro_input(input).unwrap();

    let actual = CompleteStruct::new(&macro_input);

    let expected = CompleteStruct
    {
        name: "S".to_owned(),
        fields: vec![
            ("a".to_owned(), syn::Ty::Path(
                None,
                syn::Path
                {
                    global: false,
                    segments: vec![
                        syn::PathSegment
                        {
                            ident: syn::Ident::from("A"),
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
            )),
            ("b".to_owned(), syn::Ty::Path(
                None,
                syn::Path
                {
                    global: false,
                    segments: vec![
                        syn::PathSegment
                        {
                            ident: syn::Ident::from("B"),
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
            ))
        ]
    };

    assert_eq!(expected, actual);
}