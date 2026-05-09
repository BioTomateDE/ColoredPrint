# colored-print

This crate provides an easy and concise way to print colored and styled text to
the console using [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code).

It provides modified printing and formatting macros that process colors and
styles at compile time:

- `format!` --> `cformat!`
- `println!` --> `cprintln!`
- `print!` --> `cprint!`
- `eprintln!` --> `ceprintln!`
- `eprint!` --> `ceprint!`

## What makes this crate useful?

The standard go-to crate for coloring output in a terminal is the
[`colored`](https://crates.io/crates/colored) crate. However, that crate
requires you to use methods on strings to add colors to them, which results in
long, unreadable macro invocations and requires multiple allocations.

```rust
use colored::Colorize;

println!(
   "{}",
   format!(
       "You won {} because the dealer busted with a sum of {}!",
       format!("{bet}$").bold(),
       dealer_sum.bold()
   )
   .bright_green()
);
```

This library makes using colors and styles a lot more concise:

```rust
use colored_print::cprintln;

cprintln!("%G:You won %b^{bet}$%_^ because the dealer busted with a sum of %b^{dealer_sum}%_^!");
```

Because this crate processes colors and styles entirely at compile time,
it requires no allocations at runtime and adds zero overhead.

## Syntax Guide

The style escape character is `%` (percent). It should be followed by a letter
indicating the **color** or **style** and then a character indicating your desired
**action**.

To type a literal `%`, use `%%` (repeat the escape character twice). To type a
literal `%%`, use `%%%%` (and so on).

Note: All colors and styles are automatically reset at the end of the string.

### Actions

- `:` - Foreground Color (aka. Font Color)
- `#` - Background Color
- `^` - Style Effect
- `_` - Wildcard; can clear all styles

### Colors

- `k` - Black (Black is 'k', not 'b')
- `r` - Red
- `g` - Green
- `y` - Yellow
- `b` - Blue
- `m` - Magenta
- `c` - Cyan
- `w` - White
- `_` - Default terminal color

These colors have **bright** variants which use a capital letter:

- `K` - Bright Black (Black is 'K', not 'B')
- `R` - Bright Red
- `G` - Bright Green
- `Y` - Bright Yellow
- `B` - Bright Blue
- `M` - Bright Magenta
- `C` - Bright Cyan
- `W` - Bright White

### Style Effects

- `b` - **Bold** (makes text stand out)
- `u` - <ins>Underline</ins> (adds line beneath text)
- `s` - ~~Strikethrough~~ (draws line through text)
- `d` - Dim (reduces brightness, less prominent)
- `i` - _Italic_ (slanted text for emphasis)

These style effects are **stackable**; you can activate as many as you want at
the same time. To reset them, you can use the special `_` character in place of
the style character which resets/deactivates **all** style effects.

Note: Not all terminals support every style effect.
Especially older Windows terminals might not render italic or dim properly.

## Examples

### Basic usage

```rust
use colored_print::*;

// Red text
cprintln!("%r:Hello, world!");
// Output: Hello, world! (in red)

// Green background
ceprintln!("%#gText on green");
// Output (in stderr): Text on green (with green background)

// Bold blue text
cprint!("%b^%b:Important notice");
// Output (without newline): Important notice (bold blue)
```

### Multiple styles

```rust
// Red text on yellow background
cprintln!("%r:%#yWarning!");
// Output: Warning! (red text on yellow background)

// Bold, underlined cyan
cprintln!("%b^%u^%c:Alert");
// Output: Alert (bold, underlined cyan)
```

### Bright colors

```rust
// Bright magenta text
let foo: String = cformat!("%M:Vibrant!");

// Bright white on bright blue
let bar: String = cformat!("%W:%B#Highlight");
```

### Clearing styles

```rust
// Make text red, then clear foreground color
cprintln!("%r:Error%_:Justkidding");
// Output: "Error" in red, then "Justkidding" in default color

// Note: Since there is no background color or style effects set,
//       this is equivalent to clearing all styles in this scenario:
cprintln!("%r:Error%__Justkidding");
```

### Mixing formatted and plain text

```rust
cprint!("Status: %g:OK%_:, %r:FAIL");
// Output (without newline): "Status: OK, FAIL" (with OK in green, FAIL in red)
```

### Escaping the escape character

```rust
cprintln!("10%% discount");
// Output: 10% discount

cprintln!("Go to %%AppData%%/.minecraft");
// Output: Go to %AppData%/.minecraft
```

## Crate features

There is one crate feature activated by default: `color`.
This feature can be deactivated by using `default-features = false` in _Cargo.toml_.
When the `color` feature is disabled, the `%`-syntax is still processed like usual,
but no ANSI escape sequences will be inserted into the format string literal.
Deactivating this feature can be useful for terminals that
do not support ANSI escape codes (most of them do, though).
It can also be used to allow downstream users to easily disable unwanted colors.
