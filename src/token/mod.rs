pub mod code;
pub mod tag;
pub mod text;

use std::collections::btree_map::BTreeMap;
use text::{TextToken, get_text_tokens};
use tag::{TagToken, get_tag_token};
use code::{CodeToken};
use crate::error;

#[derive(Debug)]
pub struct TextualToken {
    pub text: Option<TextToken>,
    pub tag: Option<TagToken>
}

/// Get Textual Tokens
///
/// # Description
/// Retrieve tokens related to the text styling and markdown tag
///
/// # Arguments
/// * `content` &str
///
/// # Return
/// Result<BTreeMap<usize, TextualToken>, ParserError>
pub fn get_textual_tokens(content: &str) -> Result<BTreeMap<usize, TextualToken>, error::ParserError> {
    if content.is_empty() {
        return Err(error::ParserError{message: error::EMPTY_CONTENT.to_string()});
    }

    let mut tokens = BTreeMap::new();

    for (idx, line) in content.lines().enumerate() {
        let tag_token = get_tag_token(line.trim());
        if let Some(token) = tag_token {
            tokens.insert(idx, TextualToken {
                text: None,
                tag: Some(token)
            });
        } else {
            let text_token = get_text_tokens(line.trim(), idx);
            tokens.insert(idx, TextualToken {
                text: text_token,
                tag: None
            });
        }
    }

    Ok(tokens)
}

/// Get Code Tokens
///
/// # Description
/// Get code tokens
///
/// # Arguments
/// * `content` &str
///
/// # Return
/// Result<BTreeMap<usize, CodeToken>, ParseError> 
pub fn get_code_tokens(content: &str) -> Result<BTreeMap<usize, CodeToken>, error::ParserError> {
    if content.is_empty() {
        return Err(error::ParserError{message: error::EMPTY_CONTENT.to_string()});
    }
    
    Ok(code::get_code_block_tokens(content))
}