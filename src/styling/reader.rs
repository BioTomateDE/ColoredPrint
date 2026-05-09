use super::ESCAPE;

pub struct Reader<'a> {
    // It would probably be cleaner if this wasnt public tbh
    pub string: &'a str,
    position: usize,
}

impl<'a> Reader<'a> {
    #[must_use]
    pub const fn new(string: &'a str) -> Self {
        Self {
            string,
            position: 0,
        }
    }

    // Not the cleanest code ever.
    // I blame UTF-8 tho

    #[must_use]
    pub fn peek(&self) -> Option<char> {
        if self.position >= self.string.len() {
            return None;
        }

        let slice = &self.string[self.position..];
        let character = slice.chars().next().unwrap();
        Some(character)
    }

    pub fn next(&mut self) -> Option<char> {
        self.peek().inspect(|ch| self.position += ch.len_utf8())
    }

    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }

    pub fn next_escape_char(&mut self) -> Result<char, String> {
        self.next()
            .ok_or_else(|| format!("Unexpected end of string at {ESCAPE:?} color escape character"))
    }
}
