use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    LitStr, Token,
    parse::{Parse, ParseStream},
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

#[cfg(not(feature = "no-color"))]
pub fn format_color_impl(input: TokenStream, macro_name: &str) -> TokenStream {
    let FormatInput { format_str, args } = syn::parse_macro_input!(input as FormatInput);

    let original = format_str.value();
    let result = crate::styling::process_string(&original);
    let processed = match result {
        Ok(str) => str,
        Err(err) => {
            return syn::Error::new(format_str.span(), err)
                .into_compile_error()
                .into();
        }
    };

    let format_macro = format_ident!("{macro_name}");

    TokenStream::from(quote! {
        #format_macro!(#processed, #(#args),*)
    })
}

#[cfg(feature = "no-color")]
pub fn format_color_impl(input: TokenStream, macro_name: &str) -> TokenStream {
    let format_macro = format_ident!("{macro_name}");

    TokenStream::from(quote! {
        #format_macro!(#input)
    })
}
