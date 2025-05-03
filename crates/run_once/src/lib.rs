use std::collections::VecDeque;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{ItemFn, parse_quote, parse2};

#[proc_macro_attribute]
pub fn run_once(_: TokenStream, input: TokenStream) -> TokenStream {
    run_once_inner(input.into()).unwrap().into()
}

fn run_once_inner(input: TokenStream2) -> Result<TokenStream2, syn::Error> {
    let mut input: ItemFn = parse2(input)?;

    input
        .sig
        .inputs
        .push(parse_quote!(mut __run_once: bevy::ecs::prelude::Local<bool>));

    let mut stmts = VecDeque::from(input.block.stmts);

    stmts.push_front(parse_quote! {
        if *__run_once {
            return;
        }
    });

    stmts.push_back(parse_quote! {
        *__run_once = true;
    });

    input.block.stmts = stmts.into();

    Ok(quote! {
        #input
    })
}
