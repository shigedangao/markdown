use std::fmt;

// Error message
pub const EMPTY_CONTENT: &str = "Cannot parse empty content";

#[derive(Debug, Clone)]
pub struct ParserError {
    pub message: String
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error has been encountered: message {:?}", &self.message)
    }
}