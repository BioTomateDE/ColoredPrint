#[derive(Default)]
pub struct Styles {
    bold: bool,
    dimmed: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
    // There are more styles possible in ANSI escape sequences, but they suck.
    // I'm not gonna add a "reverse" style.
}

impl Styles {
    pub(super) const fn is_default(&self) -> bool {
        !(self.bold || self.dimmed || self.italic || self.underline || self.strikethrough)
    }

    pub fn modify_from_char(&mut self, character: char) -> Result<(), String> {
        match character {
            'b' => self.bold = true,
            'd' => self.dimmed = true,
            'i' => self.italic = true,
            'u' => self.underline = true,
            's' => self.strikethrough = true,
            _ => return Err(format!("Invalid style character {character:?}")),
        }
        Ok(())
    }

    pub(super) fn render_to(&self, string: &mut String) {
        if self.bold {
            string.push_str("1;");
        }
        if self.dimmed {
            string.push_str("2;");
        }
        if self.italic {
            string.push_str("3;");
        }
        if self.underline {
            string.push_str("4;");
        }
    }
}
