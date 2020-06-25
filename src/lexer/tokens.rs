use std::default::Default;
use super::heading;
use crate::error::LexerError;

const MIN_CHAR_LENGTH: usize = 2;

#[derive(Debug, PartialEq)]
pub enum BaseOperator {
    Heading,
    UnorderedList,
    OrderedList,
    InlineCode,
    BlockQuotes,
    Text
}

impl Default for BaseOperator {
    fn default() -> Self { BaseOperator::Text }
}

#[derive(Default, Debug)]
pub struct Meta {
    HeadingLevel: heading::HeadingLevel
}

#[derive(Default, Debug)]
pub struct Token { 
    pub line: usize,
    pub content: String,
    pub operator: BaseOperator,
    pub metas: Meta
}

/// Get Tokens
///
/// # Description
/// Retrieve the tokens that are going to fed the parser
///
/// # Arguments
/// * `content` &str
///
/// # Return
/// Result<(), Error>
pub fn get_tokens(content: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();

    for (idx, line) in content.lines().enumerate() {
        let token = match_basic_token(line.trim());
        if let Some(mut t) = token {
            t.content = line.trim().to_string();
            t.line = idx;
            tokens.push(t);
        }
    }

    Ok(tokens)
}

/// Match Basic Token
///
/// # Description
/// Match the basic token for each line
///
/// # Arguments
/// * `line` &str
///
/// # Return
/// Token
fn match_basic_token(line: &str) -> Option<Token> {
    if line.len() < MIN_CHAR_LENGTH {
        return None;
    }

    let start_chars = line.split_at(MIN_CHAR_LENGTH);

    // match ordered list (i.e: <number>. , 1., 2.)
    let ordered_list = start_chars.0.find(|c: char| c.is_numeric() && c > '.');
    if ordered_list.is_some() {
        return Some(Token { operator: BaseOperator::OrderedList, ..Default::default() });
    }

    // match each byte of the byte array
    // See: https://www.reddit.com/r/rust/comments/f4usb4/pattern_matching_on_string_content_as_chars/fhtwe1f?utm_source=share&utm_medium=web2x
    // See: https://doc.rust-lang.org/edition-guide/rust-2018/slice-patterns.html
    match start_chars.0.as_bytes() {
        // #
        [35, ..] => {
            let depth = heading::get_heading_depth(line);
            Some(Token {
                operator: BaseOperator::Heading,
                metas: Meta {
                    HeadingLevel: depth
                },
                ..Default::default()
            })
        },
        // * | - | +
        [42, ..] | [45, ..] | [43, ..] => Some(Token { operator: BaseOperator::UnorderedList, ..Default::default() }),
        // `
        [96, ..] => Some(Token { operator: BaseOperator::InlineCode, ..Default::default() }),
        // >
        [62, ..] => Some(Token { operator: BaseOperator::BlockQuotes, ..Default::default() }),
        _ => Some(Token { operator: BaseOperator::Text, ..Default::default() })
    }
}
