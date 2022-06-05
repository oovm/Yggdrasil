use super::*;

pub enum ParseAdvance {
    Offset(usize),
    Character(char),
    String(&'static str),
}

impl From<usize> for ParseAdvance {
    fn from(v: usize) -> Self {
        ParseAdvance::Offset(v)
    }
}
impl From<char> for ParseAdvance {
    fn from(c: char) -> Self {
        ParseAdvance::Character(c)
    }
}
impl From<&'static str> for ParseAdvance {
    fn from(s: &'static str) -> Self {
        ParseAdvance::String(s)
    }
}
impl<'i> YState<'i> {
    pub fn advance<T>(self, term: T) -> YState<'i>
    where
        T: Into<ParseAdvance>,
    {
        let offset = match term.into() {
            ParseAdvance::Offset(v) => v,
            ParseAdvance::Character(v) => v.len_utf8(),
            ParseAdvance::String(v) => v.len(),
        };
        YState {
            partial_string: &self.partial_string[offset..],
            start_offset: self.start_offset + offset,
            farthest_error: self.farthest_error,
        }
    }
}