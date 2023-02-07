#![allow(clippy::uninlined_format_args)]

#[macro_use]
mod macros;

use syn::{parse_quote, TraitItemFn};

#[test]
fn test_by_value() {
    let TraitItemFn { sig, .. } = parse_quote! {
        fn by_value(self: Self);
    };
    snapshot!(&sig.inputs[0], @r###"
    FnArg::Typed(PatType {
        pat: Pat::Ident {
            ident: "self",
        },
        ty: Type::Path {
            path: Path {
                segments: [
                    PathSegment {
                        ident: "Self",
                    },
                ],
            },
        },
    })
    "###);
}

#[test]
fn test_by_mut_value() {
    let TraitItemFn { sig, .. } = parse_quote! {
        fn by_mut(mut self: Self);
    };
    snapshot!(&sig.inputs[0], @r###"
    FnArg::Typed(PatType {
        pat: Pat::Ident {
            mutability: Some,
            ident: "self",
        },
        ty: Type::Path {
            path: Path {
                segments: [
                    PathSegment {
                        ident: "Self",
                    },
                ],
            },
        },
    })
    "###);
}

#[test]
fn test_by_ref() {
    let TraitItemFn { sig, .. } = parse_quote! {
        fn by_ref(self: &Self);
    };
    snapshot!(&sig.inputs[0], @r###"
    FnArg::Typed(PatType {
        pat: Pat::Ident {
            ident: "self",
        },
        ty: Type::Reference {
            elem: Type::Path {
                path: Path {
                    segments: [
                        PathSegment {
                            ident: "Self",
                        },
                    ],
                },
            },
        },
    })
    "###);
}

#[test]
fn test_by_box() {
    let TraitItemFn { sig, .. } = parse_quote! {
        fn by_box(self: Box<Self>);
    };
    snapshot!(&sig.inputs[0], @r###"
    FnArg::Typed(PatType {
        pat: Pat::Ident {
            ident: "self",
        },
        ty: Type::Path {
            path: Path {
                segments: [
                    PathSegment {
                        ident: "Box",
                        arguments: PathArguments::AngleBracketed {
                            args: [
                                GenericArgument::Type(Type::Path {
                                    path: Path {
                                        segments: [
                                            PathSegment {
                                                ident: "Self",
                                            },
                                        ],
                                    },
                                }),
                            ],
                        },
                    },
                ],
            },
        },
    })
    "###);
}

#[test]
fn test_by_pin() {
    let TraitItemFn { sig, .. } = parse_quote! {
        fn by_pin(self: Pin<Self>);
    };
    snapshot!(&sig.inputs[0], @r###"
    FnArg::Typed(PatType {
        pat: Pat::Ident {
            ident: "self",
        },
        ty: Type::Path {
            path: Path {
                segments: [
                    PathSegment {
                        ident: "Pin",
                        arguments: PathArguments::AngleBracketed {
                            args: [
                                GenericArgument::Type(Type::Path {
                                    path: Path {
                                        segments: [
                                            PathSegment {
                                                ident: "Self",
                                            },
                                        ],
                                    },
                                }),
                            ],
                        },
                    },
                ],
            },
        },
    })
    "###);
}

#[test]
fn test_explicit_type() {
    let TraitItemFn { sig, .. } = parse_quote! {
        fn explicit_type(self: Pin<MyType>);
    };
    snapshot!(&sig.inputs[0], @r###"
    FnArg::Typed(PatType {
        pat: Pat::Ident {
            ident: "self",
        },
        ty: Type::Path {
            path: Path {
                segments: [
                    PathSegment {
                        ident: "Pin",
                        arguments: PathArguments::AngleBracketed {
                            args: [
                                GenericArgument::Type(Type::Path {
                                    path: Path {
                                        segments: [
                                            PathSegment {
                                                ident: "MyType",
                                            },
                                        ],
                                    },
                                }),
                            ],
                        },
                    },
                ],
            },
        },
    })
    "###);
}

#[test]
fn test_value_shorthand() {
    let TraitItemFn { sig, .. } = parse_quote! {
        fn value_shorthand(self);
    };
    snapshot!(&sig.inputs[0], @"FnArg::Receiver(Receiver)");
}

#[test]
fn test_mut_value_shorthand() {
    let TraitItemFn { sig, .. } = parse_quote! {
        fn mut_value_shorthand(mut self);
    };
    snapshot!(&sig.inputs[0], @r###"
    FnArg::Receiver(Receiver {
        mutability: Some,
    })
    "###);
}

#[test]
fn test_ref_shorthand() {
    let TraitItemFn { sig, .. } = parse_quote! {
        fn ref_shorthand(&self);
    };
    snapshot!(&sig.inputs[0], @r###"
    FnArg::Receiver(Receiver {
        reference: Some(None),
    })
    "###);
}

#[test]
fn test_ref_shorthand_with_lifetime() {
    let TraitItemFn { sig, .. } = parse_quote! {
        fn ref_shorthand(&'a self);
    };
    snapshot!(&sig.inputs[0], @r###"
    FnArg::Receiver(Receiver {
        reference: Some(Some(Lifetime {
            ident: "a",
        })),
    })
    "###);
}

#[test]
fn test_ref_mut_shorthand() {
    let TraitItemFn { sig, .. } = parse_quote! {
        fn ref_mut_shorthand(&mut self);
    };
    snapshot!(&sig.inputs[0], @r###"
    FnArg::Receiver(Receiver {
        reference: Some(None),
        mutability: Some,
    })
    "###);
}

#[test]
fn test_ref_mut_shorthand_with_lifetime() {
    let TraitItemFn { sig, .. } = parse_quote! {
        fn ref_mut_shorthand(&'a mut self);
    };
    snapshot!(&sig.inputs[0], @r###"
    FnArg::Receiver(Receiver {
        reference: Some(Some(Lifetime {
            ident: "a",
        })),
        mutability: Some,
    })
    "###);
}
