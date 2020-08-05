use std::default::Default;
use std::clone::Clone;
use std::collections::BTreeMap;
use super::{heading, text, list, code};
use super::operator::bytes;
use crate::error::ParserError;

// Minimum character length
const MIN_CHAR_LENGTH: usize = 2;

#[derive(Debug, PartialEq, Clone)]
pub enum BaseOperator {
    Heading,
    UnorderedList,
    OrderedList,
    BlockQuotes,
    BlockCodeStart,
    BlockCodeContent,
    BlockCodeEnd,
    Text
}

impl Default for BaseOperator {
    fn default() -> Self { BaseOperator::Text }
}

#[derive(Default, Debug, Clone)]
pub struct Meta {
    pub heading: Option<heading::HeadingLevel>,
    pub text_metas: Option<text::TextMetas>
}

#[derive(Default, Debug, Clone)]
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
/// Result<BTreeMap<usize, Token>, Error>
pub fn get_tokens(content: &str) -> Result<BTreeMap<usize, Token>, ParserError> {
    let mut tokens = BTreeMap::new();
    let mut previous_token: Option<Token> = None;

    for (idx, line) in content.lines().enumerate() {
        let token = match_basic_token(line.trim(), &previous_token);
        if let Some(mut t) = token {
            t.line = idx;
            if t.operator == BaseOperator::Text {
                let text_opts = text::get_text_tokens(&t);
                t.metas = Some(Meta {
                    heading: None,
                    text_metas: text_opts
                });
            }

            previous_token = Some(t.clone());
            tokens.insert(idx, t);
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
fn match_basic_token(line: &str, previous_token: &Option<Token>) -> Option<Token> {
    if line.len() < MIN_CHAR_LENGTH {
        return None;
    }

    let list = list::get_any_list(line);
    if list.is_some() {
        return list;
    }

    let code_token = code::get_block_code_token(line, previous_token);
    if code_token.is_some() {
        return code_token;
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
                metas: Some(Meta {
                    heading: Some(depth),
                    ..Default::default()
                }),
                ..Default::default()
            })
        },
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
        bytes::BLOCKQUOTE as char
    ];


    content.trim_matches(matches).trim().to_string()
}
