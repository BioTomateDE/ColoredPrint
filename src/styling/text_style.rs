use crate::styling::{color::Color, style_effect::StyleFlags};

const ANSI_SEQUENCE_START: &str = "\x1B[";
const ANSI_SEQUENCE_END: &str = "m";

#[derive(Clone, Default, PartialEq, Eq)]
pub struct TextStyle {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub styles: StyleFlags,
}

impl TextStyle {
    pub const fn is_default(&self) -> bool {
        self.foreground.is_none() && self.background.is_none() && self.styles.is_default()
    }

    fn render_to(&self, output: &mut String) {
        output.push_str(ANSI_SEQUENCE_START);

        if let Some(fg) = self.foreground {
            fg.render_foreground_to(output);
            output.push(';');
        }

        if let Some(bg) = self.background {
            bg.render_background_to(output);
            output.push(';');
        }

        self.styles.render_to(output);

        // Pop last semicolon. No idea if this is needed for most terminals.
        output.pop();

        output.push_str(ANSI_SEQUENCE_END);
    }

    fn render_reset_to(output: &mut String) {
        output.push_str(ANSI_SEQUENCE_START);
        output.push('0');
        output.push_str(ANSI_SEQUENCE_END);
    }
}

impl super::Parser<'_> {
    pub fn render_style(&mut self) {
        #[cfg(not(feature = "no-color"))]
        self.style.render_to(&mut self.output);
    }

    pub fn render_reset_style(&mut self) {
        #[cfg(not(feature = "no-color"))]
        TextStyle::render_reset_to(&mut self.output);
    }
}
