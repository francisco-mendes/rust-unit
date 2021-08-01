extern crate proc_macro;

use darling::{
    Error,
    Result,
};
use proc_macro::TokenStream;
use syn::{
    spanned::Spanned,
    *,
};

enum Test {
    Function,
    Static(LitStr),
    Format(LitStr, Vec<NestedMeta>),
}

impl Test {
    fn from_test_attr(attr_body: Vec<NestedMeta>) -> Result<Self> {
        match attr_body.len() {
            0 => Ok(Self::Function),
            1 => match &attr_body[0] {
                NestedMeta::Lit(Lit::Str(s)) => Ok(Self::Static(s.clone())),
                NestedMeta::Lit(lit) => Err(Error::unexpected_lit_type(lit))?,
                NestedMeta::Meta(meta) => {
                    Err(Error::unsupported_format("expected a string literal").with_span(meta))?
                }
            },
            _ => {
                let s = match &attr_body[0] {
                    NestedMeta::Lit(Lit::Str(s)) => s.clone(),
                    meta => Err(
                        Error::unsupported_format("expected a format string literal")
                            .with_span(meta),
                    )?,
                };
                let args = attr_body.into_iter().skip(1).collect::<Vec<_>>();

                Ok(Self::Format(s, args))
            }
        }
    }
}

struct TestFn {
    tags: Option<Vec<LitStr>>,
    source: Option<Expr>,
    func: ItemFn,
}

impl TestFn {
    fn parse_meta(m: NestedMeta) -> Result<LitStr> {
        match m {
            NestedMeta::Lit(Lit::Str(lit)) => Ok(lit),
            meta => Err(Error::unsupported_format("expected string literal").with_span(&meta)),
        }
    }

    fn from_test_fn(func: syn::ItemFn) -> Result<Self> {
        let tags = func.attrs.iter().find(|a| a.path.is_ident("tags")).cloned();

        let tags = if let Some(tags) = tags {
            match tags.parse_meta()? {
                Meta::List(MetaList { nested, .. }) => Some(
                    nested
                        .into_iter()
                        .map(|m| Self::parse_meta(m))
                        .collect::<Result<Vec<_>>>()?,
                ),
                meta => {
                    Err(Error::unsupported_format("tags attribute must be a list").with_span(&meta))?
                }
            }
        } else {
            None
        };

        let source = func
            .attrs
            .iter()
            .find(|a| a.path.is_ident("source"))
            .cloned();

        let source = if let Some(source) = source {
            match source.parse_meta()? {
                Meta::List(MetaList { nested, .. }) => match nested.len() {
                    0 => Err(Error::too_few_items(1).with_span(&nested))?,
                    1 => Some(
                        nested
                            .into_iter()
                            .map(|m| match m {
                                NestedMeta::Meta(Meta::Path(path)) => Ok(Expr::from(ExprPath {
                                    attrs: vec![],
                                    qself: None,
                                    path,
                                })),
                                meta => Err(Error::unsupported_format(
                                    "source value must be a path to a function",
                                )
                                .with_span(&meta)),
                            })
                            .next()
                            .unwrap()?,
                    ),
                    _ => Err(Error::too_many_items(1).with_span(&nested))?,
                },
                _ => None,
            }
        } else {
            None
        };

        Ok(Self { tags, source, func })
    }
}

#[proc_macro_attribute]
pub fn test(attribute: TokenStream, test_fn: TokenStream) -> TokenStream {
    let attribute = syn::parse_macro_input!(attribute as AttributeArgs);
    let test_fn = syn::parse_macro_input!(test_fn as ItemFn);

    let attribute = match Test::from_test_attr(attribute) {
        Ok(test) => test,
        Err(err) => return err.write_errors().into(),
    };

    let test_fn = match TestFn::from_test_fn(test_fn) {
        Ok(test) => test,
        Err(err) => return err.write_errors().into(),
    };

    let TestFn {
        tags,
        source,
        func:
            ItemFn {
                attrs,
                vis,
                sig,
                block,
            },
    } = test_fn;

    let tags = tags.unwrap_or(vec![]);

    let attrs = attrs
        .into_iter()
        .filter(|a| !a.path.is_ident("tags") && !a.path.is_ident("source"))
        .collect::<Vec<_>>();

    let mut outer_sig = sig.clone();
    outer_sig.inputs.clear();
    outer_sig.output = ReturnType::Default;

    let func_name = sig.ident.clone();
    let (test_name, format) = match attribute {
        Test::Function => (
            quote::quote! { ::rust_unit::TestName::Static(#func_name) },
            false,
        ),
        Test::Static(str) => (quote::quote! { ::rust_unit::TestName::Static(#str) }, false),
        Test::Format(str, args) => (
            quote::quote! {
                ::rust_unit::TestName::Dynamic(Box::new(
                    |(#(#args),*)| { format!(#str, #(#args),*) }
                ))
            },
            !args.is_empty(),
        ),
    };

    let test = match source {
        None => {
            if format {
                return Error::custom("Cannot use format name without data source")
                    .write_errors()
                    .into();
            }
            if !sig.inputs.is_empty() {
                return Error::custom("Cannot use arguments without data source")
                    .write_errors()
                    .into();
            }
            quote::quote! {
                ::std::boxed::Box::new(::std::iter::once(::rust_unit::test::simple::SimpleTest::new(
                    #test_name, &[#(#tags),*], #func_name
                )))
            }
        }
        Some(source) => {
            let args = sig
                .inputs
                .iter()
                .enumerate()
                .map(|(i, arg)| match arg {
                    FnArg::Receiver(s) => {
                        let s = s.self_token;
                        quote::quote! { #s }
                    }
                    FnArg::Typed(arg) => {
                        let s = Ident::new(&format!("__arg_{}", i), arg.span());
                        quote::quote! { #s }
                    }
                })
                .collect::<Vec<_>>();

            quote::quote! {
                let common = ::rust_unit::test::data::DataTestCommon::new(
                    #test_name, &[#(#tags),*], |(#(#args),*)| #func_name(#(#args),*)
                );
                let source = #source();
                let common = ::std::iter::repeat(common);
                ::std::boxed::Box::new(source.zip(common).map(|(data, common)|
                    ::rust_unit::test::data::DataTest::new(
                    common, data
                    )
                ))
            }
        }
    };

    let out = quote::quote! {
        #[test_case]
        #vis fn #func_name() -> ::std::boxed::Box<dyn Iterator<Item = ::std::boxed::Box<dyn ::rust_unit::Test>>> {
            #(#attrs)*
            #sig #block
            #test
        }
    };
    out.into()
}
