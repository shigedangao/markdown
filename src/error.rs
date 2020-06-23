use std::fmt;

#[derive(Debug, Clone)]
pub struct LexerError {
    pub line: u64,
    pub message: String
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unsupport markdown rules at line {:?}, code message {:?}", &self.line, &self.message)
    }
}