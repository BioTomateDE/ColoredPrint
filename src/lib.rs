#![warn(clippy::all)]
#![warn(clippy::cargo)]

mod styling;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct FormatInput {
    format_str: LitStr,
    args: Vec<syn::Expr>,
}

impl Parse for FormatInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let format_str: LitStr = input.parse()?;
        let mut args = Vec::new();

        while !input.is_empty() {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
            args.push(input.parse()?);
        }

        Ok(Self { format_str, args })
    }
}

fn format_impl(input: TokenStream, macro_name: &str) -> TokenStream {
    let FormatInput { format_str, args } = parse_macro_input!(input as FormatInput);

    let original = format_str.value();
    let processed = match styling::process_string(&original) {
        Ok(str) => str,
        Err(err) => {
            return syn::Error::new(format_str.span(), err)
                .into_compile_error()
                .into();
        }
    };

    let format_macro = format_ident!("{macro_name}");

    let expanded = quote! {
        #format_macro!(#processed, #(#args),*)
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn cformat(input: TokenStream) -> TokenStream {
    format_impl(input, "format")
}

#[proc_macro]
pub fn cprint(input: TokenStream) -> TokenStream {
    format_impl(input, "print")
}

#[proc_macro]
pub fn cprintln(input: TokenStream) -> TokenStream {
    format_impl(input, "println")
}

#[proc_macro]
pub fn ceprint(input: TokenStream) -> TokenStream {
    format_impl(input, "eprint")
}

#[proc_macro]
pub fn ceprintln(input: TokenStream) -> TokenStream {
    format_impl(input, "eprintln")
}
