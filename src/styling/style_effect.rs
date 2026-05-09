#[derive(Clone, Default, PartialEq, Eq)]
pub struct StyleFlags {
    pub bold: bool,
    pub dimmed: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    // There are more styles possible in ANSI escape sequences, but they suck.
    // I'm not gonna add a "reverse" style.
}

impl StyleFlags {
    pub fn modify_from_char(&mut self, character: char) -> Result<(), String> {
        match character {
            '_' => *self = Self::default(),
            'b' => self.bold = true,
            'd' => self.dimmed = true,
            'i' => self.italic = true,
            'u' => self.underline = true,
            's' => self.strikethrough = true,
            _ => return Err(format!("Invalid style character {character:?}")),
        }
        Ok(())
    }

    pub fn render_to(&self, output: &mut String) {
        if self.bold {
            output.push_str("1;");
        }
        if self.dimmed {
            output.push_str("2;");
        }
        if self.italic {
            output.push_str("3;");
        }
        if self.underline {
            output.push_str("4;");
        }
        if self.strikethrough {
            output.push_str("9;");
        }
    }

    pub fn render_diff_to(&self, output: &mut String, prev: &Self) {
        // Bold and dim both have the same reset sequence for some reason.
        if (prev.bold && !self.bold) || (prev.dimmed && !self.dimmed) {
            output.push_str("22;");
        }
        if self.bold {
            output.push_str("1;");
        }
        if self.dimmed {
            output.push_str("2;");
        }

        if self.italic != prev.italic {
            output.push_str(if self.italic { "3;" } else { "23;" });
        }
        if self.underline != prev.underline {
            output.push_str(if self.underline { "4;" } else { "24;" });
        }
        if self.strikethrough != prev.strikethrough {
            output.push_str(if self.strikethrough { "9;" } else { "29;" });
        }
    }
}
