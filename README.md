This crate provides an easy and concise way to print colored and styled text to
the console using ANSI escape sequences.

It provides modified printing and formatting macros that process colors and
styles at compile time:

- `format!` --> `cformat!`
- `println!` --> `cprintln!`
- `print!` --> `cprint!`
- `eprintln!` --> `ceprintln!`
- `eprint!` --> `ceprint!`

# What makes this crate useful?

The standard go-to crate for coloring output in a terminal is the
[`colored`](https://crates.io/crates/colored) crate. However, this crate
requires you to use methods on strings to add colors to them, which can bloat
your print and format macro invocations:

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

Albeit, the syntax does look a little confusing. You'll get used to it, though
(probably).

The alternative [`color-print`](https://crates.io/crates/color-print) crate is
similar to my crate, except it uses XML tags instead. This syntax does look more
readable, however it is a lot longer than my crate's 3 character syntax. You can
choose whether you prefer concision over slightly poor readability.

# Syntax Guide

The style escape character is `%` (percent). It should be followed by a letter
indicating the color or style and then a character indicating your desired
action.

To type a literal `%`, use `%%` (repeat the escape character twice). To type a
literal `%%`, use `%%%%` (you get the point).

## Actions

- `:` - Foreground Color (aka Font Color)
- `#` - Background Color
- `^` - Style Effect
- `_` - Wildcard; can clear all styles

## Colors

- `k` - Black (Black is 'k', not 'b')
- `r` - Red
- `g` - Green
- `y` - Yellow
- `b` - Blue
- `m` - Magenta
- `c` - Cyan
- `w` - White

These colors have **bright** variants, which causes the letters to be
capitalized:

- `K` - Bright Black (Black is 'K', not 'B')
- `R` - Bright Red
- `G` - Bright Green
- `Y` - Bright Yellow
- `B` - Bright Blue
- `M` - Bright Magenta
- `C` - Bright Cyan
- `W` - Bright White

To reset the foreground/background color to its original terminal color, you can
use the special `_` character in place of the color character.

## Style Effects

- `b` - **Bold** (makes text stand out)
- `d` - Dim (reduces brightness, less prominent)
- `i` - _Italic_ (slanted text for emphasis)
- `u` - <ins>Underline</ins> (adds line beneath text)
- `s` - ~~Strikethrough~~ (draws line through text)

These style effects are **stackable**; you can activate as many as you want at
the same time. To reset them, you can use the special `_` character in place of
the style character which resets/deactivates **all** style effects.

**Note**: Not all terminals support every style effect. Some common limitations:

- Windows terminals may not support dim or italic.
- Some terminals render italic as inverse video (swapped foreground and
  background colors).
- Some terminals may not render italic at all.
- Strikethrough is the least widely supported.
- There are more ANSI style effects than available in this library, but I will
  not allow things like `blink` for the sanity of end users.

# Examples

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
ceprint!("Status: %g:OK%_:, %r:FAIL");
// Output (stderr, without newline): "Status: OK, FAIL" (with OK in green, FAIL in red)
```

### Escaping the escape character

```rust
cprintln!("10%% discount");
// Output: 10% discount

cprintln!("Path: %%AppData%%");
// Output: Path: %AppData%
```

# Pros of this crate

- Styling is processed entirely at compile time; there is no runtime overhead at
  all.
- Short and concise syntax.

# Cons of this crate

- If the terminal does not support ANSI, it will most likely print out weird
  looking arrows: `←[31mRed Text←[0m`.

  However, the terminal emulator cannot be detected at compile time since any
  terminal could run any CLI binary. Since this library functions purely in
  compile-time, this is unavoidable.

  On the flip side, most terminals do support ANSI (even modern Windows!).

# Crate features

There is one opt-in feature available: `no-color`. When this feature is
activated, no ANSI escape sequences will generated, leading to normal output
without any colors or styles.

Note the following things:

- Your format string still has to be valid, otherwise it will not compile.
- Escape sequences such as `%r:` will not be part of the output, just like in
  normal mode.
- `%%` will still be needed to output a literal `%`.

Basically, your format style string should stay the same, only the generated
output is different.

## Contributing

All contributions are welcome!

By contributing, you agree to:

- License your contributions under this project's license
- Certify you have the right to submit the code
- Allow the project maintainer to use your contributions
