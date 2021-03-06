use std::clone::Clone;
use crate::parser::text_style::{TextMetas, get_text_metas, sanitze_content};
use crate::parser::code;

#[derive(Debug, Clone)]
pub struct TextToken { 
    pub line: usize,
    pub content: String,
    pub sanitize_content: String,
    pub metas: Option<TextMetas>
}

/// Get Text Tokens
///
/// # Description
/// Retrieve the text token
///
/// # Argument
/// * `content` &str
/// * `line` usize
///
/// # Return
/// Option<TextToken>
pub fn get_text_tokens(content: &str, line: usize) -> Option<TextToken> {
    if code::is_code(content) {
        return None;
    }

    let metas = get_text_metas(content);

    Some(
        TextToken {
            line,
            content: String::from(content.trim()),
            sanitize_content: sanitze_content(content),
            metas
        }
    )
}
