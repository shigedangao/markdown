use std::default::Default;
use super::{heading};
use super::operator::bytes;
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
    Heading: Option<heading::HeadingLevel>
}

#[derive(Default, Debug)]
pub struct Token { 
    pub line: usize,
    pub content: String,
    pub operator: BaseOperator,
    pub metas: Option<Meta>
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
/// Option<Token>
fn match_basic_token(line: &str) -> Option<Token> {
    if line.len() < MIN_CHAR_LENGTH {
        return None;
    }

    let ordered_list = get_ordered_list_token(line);
    if ordered_list.is_some() {
        return ordered_list;
    }

    let trimmed_content = trim_matches_content(line);
    
    // We only want to match for the few characters at the beginning
    let start_chars = line.split_at(MIN_CHAR_LENGTH);
    // match each byte of the byte array
    // See: https://www.reddit.com/r/rust/comments/f4usb4/pattern_matching_on_string_content_as_chars/fhtwe1f?utm_source=share&utm_medium=web2x
    // See: https://doc.rust-lang.org/edition-guide/rust-2018/slice-patterns.html
    match start_chars.0.as_bytes() {
        // #
        [bytes::HEADING, ..] => {
            let depth = heading::get_heading_depth(line);
            Some(Token {
                operator: BaseOperator::Heading,
                content: trimmed_content,
                metas: Some(Meta { Heading: Some(depth) }),
                ..Default::default()
            })
        },
        // * | - | +
        [bytes::UNORDERED_MUL, ..] | 
        [bytes::UNORDERED_MINUS, ..] |
        [bytes::UNORDERED_PLUS, ..] => Some(Token {
            operator: BaseOperator::UnorderedList,
            content: trimmed_content,
            ..Default::default()
        }),
        // `
        [bytes::CODE, ..] => Some(Token {
            operator: BaseOperator::InlineCode,
            content: trimmed_content,
            ..Default::default()
        }),
        // >
        [bytes::BLOCKQUOTE, ..] => Some(Token {
            operator: BaseOperator::BlockQuotes,
            content: trimmed_content,
            ..Default::default()
        }),
        _ => Some(Token {
            operator: BaseOperator::Text,
            content: trimmed_content,
            ..Default::default()
        })
    }
}

/// Trim Matches Content
///
/// # Description
/// Trim the matches content
///
/// # Arguments
/// * `content` &str
///
/// # Return
/// String
fn trim_matches_content(content: &str) -> String {
    let matches: &[char] = &[
        bytes::HEADING as char,
        bytes::UNORDERED_MUL as char,
        bytes::UNORDERED_MINUS as char,
        bytes::UNORDERED_PLUS as char,
        bytes::CODE as char,
        bytes::BLOCKQUOTE as char
    ];


    content.trim_matches(matches).trim().to_string()
}
 
/// Get Ordered List Token
///
/// # Description
/// For the ordered list we need to do more treatment as
/// we need to see if the preceding character is a number and 
/// if the next character is a "." 
///
/// # Arguments
/// * `content` &str
///
/// # Return
/// Option<Token>
fn get_ordered_list_token(content: &str) -> Option<Token> {
    // match ordered list (i.e: <number>. , 1., 2.)
    let ordered_list = content.find(|c: char| c.is_numeric() && c > '.');
    if ordered_list.is_none() {
        return None;
    }

    let trimmed_content = content
        .trim_start_matches(|c: char| c.is_numeric() || c == '.')
        .trim()
        .to_string();

    Some(Token {
        operator: BaseOperator::OrderedList,
        content: trimmed_content,
        ..Default::default()
    })
}