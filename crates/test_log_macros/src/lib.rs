use proc_macro2::{Ident, TokenStream as Tokens,};
use proc_macro_error::{proc_macro_error};
use quote::quote;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, ItemFn, Meta, MetaNameValue, Token};


#[proc_macro_error]
#[proc_macro_attribute]
pub fn test(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item_fn = parse_macro_input!(item as ItemFn);
    let out = try_test(
        attr, item_fn,
    )
        .unwrap_or_else(|err| err.to_compile_error())
        .into();
    out
}

fn try_test(attr: proc_macro::TokenStream, test: ItemFn) -> syn::Result<Tokens> {
    let attribute_args = if attr.is_empty() {
        AttributeArgs::default()
    } else {
        AttributeArgs::try_parse_args(attr)?
    };

    let test_harness = match attribute_args.harness {
        None => {
            quote! {::core::prelude::v1::test}
        }
        Some(s) => { s }
    };

    let settings = attribute_args.settings;
    let configure_options = settings
        .iter()
        .map(|(method, value)| {
            quote! {
                .#method(#value)
            }
        })
        .collect::<Tokens>();

    let init =
        if cfg!(feature = "tracing") {
            quote! {
                use ::common::tracing::LoggingOptions;
                use ::common::tracing::subscriber::util::SubscriberInitExt;
                use ::common::tracing::{Stdout, Stderr, File};
                use ::common::tracing::level_filters::LevelFilter;
                let mut options = LoggingOptions::new()
                    .files(true)
                    .lines(true)
                    .thread_ids(true)
                    #configure_options;

                if options.target_count() == 0 {
                    let level = options.level.clone();
                    options = options.target(
                        Stdout(level)
                    );
                }

                let subscriber = options.into_subscriber().expect("could not create subscriber");
                let _ = subscriber.try_init();
            }
        } else {
            quote! {}
        };

    let ItemFn {
        attrs, vis, sig, block
    } = test;
    let result = quote! {
        #[#test_harness]
        #(#[#attrs])*
        #vis #sig {
            mod init {
                use super::*;
                pub fn init() {
                    #init
                }
            }

            init::init();

            #block
        }
    };
    Ok(result)
}


#[derive(Debug, Default)]
struct AttributeArgs {
    harness: Option<Tokens>,
    settings: HashMap<Ident, Expr>,
}

impl AttributeArgs {
    fn try_parse_args(attr: proc_macro::TokenStream) -> syn::Result<Self> {
        let args = Punctuated::<Meta, Token![,]>::parse_terminated.parse(attr)?;
        let mut built_args = AttributeArgs::default();

        let mut args_deque = args.iter().collect::<VecDeque<_>>();

        if args_deque.len() >= 1 && matches!(args_deque.front(), Some(Meta::Path(_))) {
            let popped = args_deque.pop_front().unwrap();
            built_args.harness = Some(quote! { #popped });
        }

        for arg in args_deque {
            built_args.parse_single_arg(arg)?;
        }

        Ok(built_args)
    }

    fn parse_single_arg(&mut self, meta: &Meta) -> syn::Result<()> {
        let MetaNameValue {
            path, eq_token: _, value,
        } = meta.require_name_value()?;
        let ident = path.require_ident()?.clone();
        match self.settings.entry(ident.clone()) {
            Entry::Occupied(_) => {
                Err(syn::Error::new(ident.span(), "Duplicate setting"))
            }
            Entry::Vacant(v) => {
                v.insert(value.clone());
                Ok(())
            }
        }
    }
}