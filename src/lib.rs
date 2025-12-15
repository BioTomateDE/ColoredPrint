//! This crate provides an easy and concise way to
//! print colored and styled text to the console using
//! ANSI escape sequences.
//!
//! The regular crate for coloring output in a CLI is
//! [`colored`](https://crates.io/crates/colored).
//! However, this crate requires you to use methods on strings
//! to add colors to them, which can bloat your print and format macro invocations:
//! ```no_run
//! use colored::Colorize;
//!
//! println!(
//!    "{}",
//!    format!(
//!        "You won {}$ because the dealer busted with a sum of {}!",
//!        bet.bold(),
//!        dealer_sum.bold()
//!    )
//!    .bright_green()
//! );
//! ```
//!
//! This library makes this a lot more concise:
//! ```
//! use colored_print::cprintln;
//!
//! cprintln!("%G:You won %b^{bet}$%_^ because the dealer busted with a sum of %b^{dealer_sum}%_^!");
//! ```
//!
//! Albeit, the syntax does look a little confusing.
//! You'll get used to it, though (probably).
//! If you prefer a slightly longer but more comprehensible syntax,
//! check out the [`color-print`](https://crates.io/crates/color-print) crate.
//!
//! # Syntax Guide
//! The escape character is `%`.
//! It should be followed by a letter indicating the color or style
//! and then the action type character.
//!
//! To type a literal `%`, use `%%` (repeat the escape character twice).
//! To type a literal `%%`, use `%%%%` (you get the point).
//!
//! ## Action type characters
//! * `:` - Foreground / Font color
//! * `#` - Background color
//! * `^` - Style
//! * `_` - Wildcard; can clear all styles
//!
//! ## Colors
//! * `k` - Black  (Black is 'k', not 'b')
//! * `r` - Red
//! * `g` - Green
//! * `y` - Yellow
//! * `b` - Blue
//! * `m` - Magenta
//! * `c` - Cyan
//! * `w` - White
//!
//! These colors have **bright** variants, which causes the letters to be capitalized:
//! * `K` - Bright Black  (Black is 'K', not 'B')
//! * `R` - Bright Red
//! * `G` - Bright Green
//! * `Y` - Bright Yellow
//! * `B` - Bright Blue
//! * `M` - Bright Magenta
//! * `C` - Bright Cyan
//! * `W` - Bright White
//!
//! ## Styles
//!
//! * `b` - **Bold** (makes text stand out)
//! * `d` - Dim (reduces brightness, less prominent)
//! * `i` - *Italic* (slanted text for emphasis)
//! * `u` - <u>Underline</u> (adds line beneath text)
//! * `s` - ~~Strikethrough~~ (draws line through text)
//!
//! **Note**: Not all terminals support every style. Some common limitations:
//! - Windows terminals may not support dim or italic.
//! - Some terminals render italic as inverse video (swapped foreground and backgrounbd colors).
//! - Some terminals may not render italic at all.
//! - Strikethrough is the least widely supported.
//! - There are more ANSI styles than available in this library,
//!   but I will not allow `blink` for the sanity of end users.
//!
//! # Examples
//!
//! ### Basic usage
//! ```
//! use colored_print::*;
//!
//! // Red text
//! cprintln!("%r:Hello, world!");
//! // Output: Hello, world! (in red)
//!
//! // Green background
//! ceprintln!("%#gText on green");
//! // Output (in stderr): Text on green (with green background)
//!
//! // Bold blue text
//! cprint!("%b^%b:Important notice");
//! // Output (without newline): Important notice (bold blue)
//! ```
//!
//! ### Multiple styles
//! ```
//! // Red text on yellow background
//! cprintln!("%r:%#yWarning!");
//! // Output: Warning! (red text on yellow background)
//!
//! // Bold, underlined cyan
//! cprintln!("%b^%u^%c:Alert");
//! // Output: Alert (bold, underlined cyan)
//! ```
//!
//! ### Bright colors
//! ```
//! // Bright magenta text
//! let foo: String = cformat!("%M:Vibrant!");
//!
//! // Bright white on bright blue
//! let bar: String = cformat!("%W:%B#Highlight");
//! ```
//!
//! ### Clearing styles
//! ```
//! // Make text red, then clear foreground color
//! cprintln!("%r:Error%_: (fixed)");
//! // Output: "Error" in red, then "(fixed)" in default color
//!
//! // Note: In this case, this is equivalent to clearing all styles:
//! cprintln!("%r:Error%__ (fixed)");
//! ```
//!
//! ### Mixing formatted and plain text
//! ```
//! ceprint!("Status: %g:OK%_:, %r:FAIL");
//! // Output (stderr, without newline): "Status: OK, FAIL" (with OK in green, FAIL in red)
//! ```
//!
//! ### Escaping the escape character
//! ```
//! cprintln!("10%% discount");
//! // Output: 10% discount
//!
//! cprintln!("Path: %%AppData%%");
//! // Output: Path: %AppData%
//! ```
//!
//! # Pros of this crate
//! * Styling is at compile time; no runtime overhead at all.
//! * Short and concise syntax.
//!
//! # Cons of this crate
//! * If the terminal does not support ANSI,
//!   it will most likely print out weird looking arrows:
//!   `←[31mRed Text←[0m`.
//!
//!   However, the terminal emulator cannot be detected at compile time
//!   since any terminal could run any CLI binary.
//!   Since this library functions purely in compile-time, this is unavoidable.
//!
//!   On the flip side, most terminals do support ANSI (even modern Windows!).

#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![allow(clippy::negative_feature_names)]

#[cfg(not(feature = "no-color"))]
mod editing;

mod styling;

#[cfg(not(feature = "no-color"))]
use editing::format_color_impl;

use proc_macro::TokenStream;

#[proc_macro]
pub fn cformat(input: TokenStream) -> TokenStream {
    #[cfg(feature = "no-color")]
    {
        input
    }
    #[cfg(not(feature = "no-color"))]
    format_color_impl(input, "format")
}

#[proc_macro]
pub fn cprint(input: TokenStream) -> TokenStream {
    #[cfg(feature = "no-color")]
    {
        input
    }
    #[cfg(not(feature = "no-color"))]
    format_color_impl(input, "print")
}

#[proc_macro]
pub fn cprintln(input: TokenStream) -> TokenStream {
    #[cfg(feature = "no-color")]
    {
        input
    }
    #[cfg(not(feature = "no-color"))]
    format_color_impl(input, "println")
}

#[proc_macro]
pub fn ceprint(input: TokenStream) -> TokenStream {
    #[cfg(feature = "no-color")]
    {
        input
    }
    #[cfg(not(feature = "no-color"))]
    format_color_impl(input, "eprint")
}

#[proc_macro]
pub fn ceprintln(input: TokenStream) -> TokenStream {
    #[cfg(feature = "no-color")]
    {
        input
    }
    #[cfg(not(feature = "no-color"))]
    format_color_impl(input, "eprintln")
}
