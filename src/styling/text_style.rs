use crate::styling::color::Color;
use crate::styling::style_effect::StyleFlags;

const ANSI_SEQUENCE_START: &str = "\x1B[";
const ANSI_SEQUENCE_END: &str = "m";

#[derive(Clone, Default, PartialEq, Eq)]
pub struct TextStyle {
    pub foreground: Color,
    pub background: Color,
    pub styles: StyleFlags,
}

impl TextStyle {
    fn render_to(&self, output: &mut String, prev: &Self) {
        // No need to render any ANSI escape sequence if nothing changed.
        if self == prev {
            return;
        }

        output.push_str(ANSI_SEQUENCE_START);

        // Choose the most efficient path (least amount of ANSI modifiers).
        if reset_count(prev, self) <= toggle_count(prev, self) {
            // Reset all colors and styles, then activate all new colors and styles.
            if *prev != Self::default() {
                output.push_str("0;");
            }

            if self.foreground != Color::Default {
                *output += self.foreground.foreground_code();
            }

            if self.background != Color::Default {
                *output += self.background.foreground_code();
            }

            self.styles.render_to(output);
        } else {
            // Enable/disable certain styles and change the colors to the new ones.
            if self.foreground != prev.foreground {
                output.push_str(self.foreground.foreground_code());
            }

            if self.background != prev.background {
                output.push_str(self.background.background_code());
            }

            self.styles.render_diff_to(output, &prev.styles);
        }

        // Pop the last semicolon.
        debug_assert_eq!(output.pop(), Some(';'));

        output.push_str(ANSI_SEQUENCE_END);
    }
}

impl super::Parser<'_> {
    pub fn render_style(&mut self) {
        #[cfg(feature = "color")]
        self.style.render_to(&mut self.output, &self.previous_style);
    }
}

/// How many ANSI modifiers it takes to fully reset the color and
/// style and then activate the new colors/styles.
fn reset_count(old: &TextStyle, new: &TextStyle) -> u8 {
    let mut count = 0;
    if *old != TextStyle::default() {
        count += 1;
    }
    if new.foreground != Color::Default {
        count += 1;
    }
    if new.background != Color::Default {
        count += 1;
    }
    if new.styles.bold {
        count += 1;
    }
    if new.styles.dimmed {
        count += 1;
    }
    if new.styles.italic {
        count += 1;
    }
    if new.styles.underline {
        count += 1;
    }
    if new.styles.strikethrough {
        count += 1;
    }
    count
}

/// How many ANSI modifiers it takes to toggle from the old colors/styles to the new ones.
fn toggle_count(old: &TextStyle, new: &TextStyle) -> u8 {
    let mut count = 0;
    if old.foreground != new.foreground {
        count += 1;
    }
    if old.background != new.background {
        count += 1;
    }
    if old.styles.bold != new.styles.bold {
        count += 1;
    }
    if old.styles.dimmed != new.styles.dimmed {
        count += 1;
    }
    if old.styles.italic != new.styles.italic {
        count += 1;
    }
    if old.styles.underline != new.styles.underline {
        count += 1;
    }
    if old.styles.strikethrough != new.styles.strikethrough {
        count += 1;
    }
    count
}
