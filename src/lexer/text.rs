use super::token::Token;

#[derive(Debug, PartialEq)]
pub enum TextOperator {
    Bold,
    Underline,
    Strike,
    Link,
    Image
}

pub fn get_text_tokens(token: Token) {
    // use split_terminator
}