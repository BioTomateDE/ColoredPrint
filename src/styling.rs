mod color;
mod style;

use std::str::CharIndices;

use color::Color;
use style::Styles;

const SEQUENCE_START: &str = "\x1B[";
const SEQUENCE_END: &str = "m";

#[derive(Default)]
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

        string.push_str(SEQUENCE_START);

        if let Some(fg) = self.foreground {
            fg.render_foreground_to(string);
        }

        if let Some(bg) = self.background {
            bg.render_background_to(string);
        }

        self.styles.render_to(string);

        // Pop last semicolon. No idea if this is needed for most terminals.
        string.pop();

        string.push_str(SEQUENCE_END);
    }

    fn render_reset_to(&self, string: &mut String) {
        string.push_str(SEQUENCE_START);
        string.push('0');
        string.push_str(SEQUENCE_START);
    }
}

pub fn process_string(string: &str) -> Result<String, String> {
    const ESCAPE: char = '%';
    const RESET: char = '_';
    const BACKGROUND: char = '#';
    const FOREGROUND: char = ':';
    const STYLE: char = '^';
    const ALL: char = '_';

    fn get_next_char(chars: &mut CharIndices) -> Result<char, String> {
        chars
            .next()
            .ok_or_else(|| format!("Unexpected end of string at {ESCAPE:?} color escape character"))
            .map(|(_i, ch)| ch)
    }

    let mut out = String::with_capacity(string.len());
    let mut state = StyleState::default();
    let mut start = 0;

    let mut chars = string.char_indices();

    while let Some((i, char)) = chars.next() {
        if char != ESCAPE {
            continue;
        }

        // Double ESCAPE character in a row => push literal ESCAPE character
        if i == start {
            out.push(ESCAPE);
            start = i + 1;
            continue;
        }

        let param = get_next_char(&mut chars)?;
        let action: char = get_next_char(&mut chars)?;

        match action {
            BACKGROUND if param == RESET => state.background = None,
            BACKGROUND => state.background = Some(Color::from_char(param).ctx(i, string)?),
            FOREGROUND if param == RESET => state.foreground = None,
            FOREGROUND => state.foreground = Some(Color::from_char(param).ctx(i, string)?),
            STYLE if param == RESET => state.styles = Styles::default(),
            STYLE => state.styles.modify_from_char(param).ctx(i, string)?,
            ALL if param == RESET => state = StyleState::default(),
            c => {
                return Err(format!("Invalid action character {c:?}")).ctx(i, string);
            }
        }

        state.render_reset_to(&mut out);
        state.render_to(&mut out);
        out += &string[start..i];
        start = i + 1;
    }

    out += &string[start..];
    state.render_reset_to(&mut out);
    Ok(out)
}

// TODO: no-color feature

pub(crate) trait PushContext<T> {
    fn ctx(self, i: usize, string: &str) -> Result<T, String>;
}
impl<T> PushContext<T> for Result<T, String> {
    fn ctx(self, i: usize, string: &str) -> Result<T, String> {
        self.map_err(|e| e + &format!(" at position {i}: {:?}", &string[i..]))
    }
}
