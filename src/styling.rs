mod color;
mod reader;
mod style_effect;
mod text_style;

use color::Color;
use style_effect::StyleFlags;

use {reader::Reader, text_style::TextStyle};

const ESCAPE: char = '%';

const RESET: char = '_';

const BACKGROUND: char = '#';
const FOREGROUND: char = ':';
const STYLE: char = '^';
const ALL: char = '_';

struct Parser<'a> {
    reader: Reader<'a>,
    output: String,
    style: TextStyle,
    previous_style: TextStyle,
    text_start_pos: usize,
}

impl<'a> Parser<'a> {
    #[must_use]
    fn new(input_string: &'a str) -> Self {
        let reader = Reader::new(input_string);
        let output = String::with_capacity(input_string.len() + 10);
        Self {
            reader,
            output,
            style: TextStyle::default(),
            previous_style: TextStyle::default(),
            text_start_pos: 0,
        }
    }

    fn process(mut self) -> Result<String, String> {
        'outer: while let Some(first_char) = self.reader.peek() {
            if first_char != ESCAPE {
                self.reader.next();
                continue;
            }
            let escape_start = self.reader.position();

            while self.reader.peek() == Some(ESCAPE) {
                // Consume the ESCAPE character.
                self.reader.next();

                // Two ESCAPE characters in a row; push literal ESCAPE character
                if self.reader.peek() == Some(ESCAPE) {
                    // Consume the second ESCAPE character.
                    self.reader.next();

                    // Write the buffer and styles immediately.
                    // This will also update `self.text_start_pos` accordingly.
                    self.write_buffer(escape_start);

                    // Then push the literal ESCAPE character.
                    self.output.push(ESCAPE);

                    // It's no longer an escape sequence; it reached text.
                    // Therefore, break out of this inner loop and restart.
                    continue 'outer;
                }

                let param: char = self.reader.next_escape_char()?;
                let action: char = self.reader.next_escape_char()?;

                if let Err(err) = self.process_escape_sequence(param, action) {
                    let pos = self.reader.position();
                    let rest = &self.reader.string[pos..];
                    let err = format!("{err} at position {pos}: {rest:?}");
                    return Err(err);
                }
            }

            // No more escape sequences (in a row); reached text or end.
            self.write_buffer(escape_start);
        }

        // Reached the end of the input string.
        // Push the last string slice and reset style, if needed.
        self.output += &self.reader.string[self.text_start_pos..];
        if !self.style.is_default() {
            self.render_reset_style();
        }

        Ok(self.output)
    }

    fn process_escape_sequence(&mut self, param: char, action: char) -> Result<(), String> {
        match action {
            BACKGROUND if param == RESET => self.style.background = None,
            BACKGROUND => self.style.background = Some(Color::from_char(param)?),
            FOREGROUND if param == RESET => self.style.foreground = None,
            FOREGROUND => self.style.foreground = Some(Color::from_char(param)?),
            STYLE if param == RESET => self.style.styles = StyleFlags::default(),
            STYLE => self.style.styles.modify_from_char(param)?,
            ALL if param == RESET => self.style = TextStyle::default(),
            _ => {
                return Err(format!("Invalid action character {action:?}"));
            }
        }

        Ok(())
    }

    /// Write the buffer and then the styles.
    fn write_buffer(&mut self, escape_start: usize) {
        // Append buffered output from the input stringd.
        self.output += &self.reader.string[self.text_start_pos..escape_start];

        // Only write ANSI escape sequence if something actually changed (very likely though).
        if self.style != self.previous_style {
            // Only reset the style if either:
            // * the current style it the default.
            // * the previous style WAS NOT the default.
            // Only one of these can be true since they were verified to be different.
            if self.style.is_default() || !self.previous_style.is_default() {
                self.render_reset_style();
            }

            // Render the style if it's not the default (would cause invalid ANSI escape sequence)
            if !self.style.is_default() {
                self.render_style();
            }
        }

        self.text_start_pos = self.reader.position();
        self.previous_style = self.style.clone();
    }
}

pub fn process_string(string: &str) -> Result<String, String> {
    Parser::new(string).process()
}
