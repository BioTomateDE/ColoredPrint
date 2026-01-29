#![doc = include_str!("../README.md")]
#![allow(clippy::negative_feature_names)]

mod editing;

#[cfg_attr(
    feature = "no-color",
    allow(dead_code, unused_variables, unused_imports)
)]
mod styling;

use editing::format_color_impl;
use proc_macro::TokenStream;

#[proc_macro]
pub fn cformat(input: TokenStream) -> TokenStream {
    format_color_impl(input, "format")
}

#[proc_macro]
pub fn cprint(input: TokenStream) -> TokenStream {
    format_color_impl(input, "print")
}

#[proc_macro]
pub fn cprintln(input: TokenStream) -> TokenStream {
    format_color_impl(input, "println")
}

#[proc_macro]
pub fn ceprint(input: TokenStream) -> TokenStream {
    format_color_impl(input, "eprint")
}

#[proc_macro]
pub fn ceprintln(input: TokenStream) -> TokenStream {
    format_color_impl(input, "eprintln")
}
