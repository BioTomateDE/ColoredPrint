#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]
#![allow(clippy::negative_feature_names, clippy::struct_excessive_bools)]

mod editing;
mod styling;

use editing::format_color_impl;
use proc_macro::TokenStream;

/// `format!` but with colors and styles.
#[proc_macro]
pub fn cformat(input: TokenStream) -> TokenStream {
    format_color_impl(input, "format")
}

/// `print!` but with colors and styles.
#[proc_macro]
pub fn cprint(input: TokenStream) -> TokenStream {
    format_color_impl(input, "print")
}

/// `println!` but with colors and styles.
#[proc_macro]
pub fn cprintln(input: TokenStream) -> TokenStream {
    format_color_impl(input, "println")
}

/// `eprint!` but with colors and styles.
#[proc_macro]
pub fn ceprint(input: TokenStream) -> TokenStream {
    format_color_impl(input, "eprint")
}

/// `eprintln!` but with colors and styles.
#[proc_macro]
pub fn ceprintln(input: TokenStream) -> TokenStream {
    format_color_impl(input, "eprintln")
}
