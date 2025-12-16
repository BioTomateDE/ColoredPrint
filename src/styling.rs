mod color;
mod reader;
mod style;

use color::Color;
use style::Styles;

use crate::styling::reader::Reader;

const ANSI_SEQUENCE_START: &str = "\x1B[";
const ANSI_SEQUENCE_END: &str = "m";

const ESCAPE: char = '%';
const RESET: char = '_';
const BACKGROUND: char = '#';
const FOREGROUND: char = ':';
const STYLE: char = '^';
const ALL: char = '_';

#[derive(Clone, Default, PartialEq, Eq)]
struct StyleState {
    foreground: Option<Color>,
    background: Option<Color>,
    styles: Styles,
}

impl StyleState {
    const fn is_default(&self) -> bool {
        self.foreground.is_none() && self.background.is_none() && self.styles.is_default()
    }

    fn render_to(&self, string: &mut String) {
        if self.is_default() {
            return;
        }

        string.push_str(ANSI_SEQUENCE_START);

        if let Some(fg) = self.foreground {
            fg.render_foreground_to(string);
        }

        if let Some(bg) = self.background {
            bg.render_background_to(string);
        }

        self.styles.render_to(string);

        // Pop last semicolon. No idea if this is needed for most terminals.
        string.pop();

        string.push_str(ANSI_SEQUENCE_END);
    }

    fn render_reset_to(&self, string: &mut String) {
        string.push_str(ANSI_SEQUENCE_START);
        string.push('0');
        string.push_str(ANSI_SEQUENCE_END);
    }
}

struct Parser<'a> {
    reader: Reader<'a>,
    output: String,
    style_state: StyleState,
    text_start_pos: usize,
    previous_styles: StyleState,
}

impl<'a> Parser<'a> {
    #[must_use]
    fn new(input_string: &'a str) -> Self {
        let reader = Reader::new(input_string);
        let output = String::with_capacity(input_string.len() + 10);
        Self {
            reader,
            output,
            style_state: StyleState::default(),
            text_start_pos: 0,
            previous_styles: StyleState::default(),
        }
    }

    fn process(mut self) -> Result<String, String> {
        while let Some(char) = self.reader.next() {
            if char != ESCAPE {
                continue;
            }

            // Double ESCAPE character in a row => push literal ESCAPE character
            if self.reader.peek() == Some(ESCAPE) {
                self.output.push(ESCAPE);
                self.reader.next();
                self.text_start_pos = self.reader.position() + 1;
                continue;
            }

            if let Err(err) = self.process_escape_sequence() {
                let pos = self.reader.position();
                let rest = self.reader.finish(pos);
                let err = format!("{err} at position {pos}: {rest:?}");
                return Err(err);
            }
        }

        self.output += self.reader.finish(self.text_start_pos);
        self.style_state.render_reset_to(&mut self.output);
        Ok(self.output)
    }

    fn process_escape_sequence(&mut self) -> Result<(), String> {
        let escape_start = self.reader.position();
        let param: char = self.reader.next_escape_char()?;
        let action: char = self.reader.next_escape_char()?;

        match action {
            BACKGROUND if param == RESET => self.style_state.background = None,
            BACKGROUND => self.style_state.background = Some(Color::from_char(param)?),
            FOREGROUND if param == RESET => self.style_state.foreground = None,
            FOREGROUND => self.style_state.foreground = Some(Color::from_char(param)?),
            STYLE if param == RESET => self.style_state.styles = Styles::default(),
            STYLE => self.style_state.styles.modify_from_char(param)?,
            ALL if param == RESET => self.style_state = StyleState::default(),
            c => {
                return Err(format!("Invalid action character {c:?}"));
            }
        }

        // If there is another escape sequence directly after, don't write yet.
        if self.reader.peek() == Some(ESCAPE) {
            return Ok(());
        }

        // Now, this is the last escape sequence in a row.

        // Only write ANSI escape sequences if something actually changed.
        if self.style_state != self.previous_styles {
            // Only reset style if it was not the default.
            if !self.previous_styles.is_default() {
                self.style_state.render_reset_to(&mut self.output);
            }
            self.style_state.render_to(&mut self.output);
        }

        // Append buffered output from the input string
        self.output += &self.reader.string[self.text_start_pos..escape_start];

        self.text_start_pos = self.reader.position() + 1;
        self.previous_styles = self.style_state.clone();
        Ok(())
    }
}

pub fn process_string(string: &str) -> Result<String, String> {
    Parser::new(string).process()
}
