use std::default::Default;
use crate::parser::{list, heading};
use crate::parser::operator::bytes;

// Minimum character length
const MIN_CHAR_LENGTH: usize = 2;

#[derive(Debug, PartialEq)]
pub enum TagOperator {
    Heading,
    UnorderedList,
    OrderedList,
    BlockQuotes
}

impl Default for TagOperator {
    fn default() -> Self { TagOperator::Heading }
}

#[derive(Debug, Default)]
pub struct TagToken { 
    pub line: usize,
    pub content: String,
    pub operator: TagOperator,
    pub metas: Option<TagMeta>
}

#[derive(Debug)]
pub struct TagMeta {
    pub heading_kind: heading::HeadingLevel
}

/// Get Tag Token
///
/// # Description
/// Get Tag Token
///
/// # Arguments
/// * `line` &str
///
/// # Return
/// Option<TagToken>
pub fn get_tag_token(line: &str) -> Option<TagToken> {
    let list = list::get_any_list(line);
    if list.is_some() {
        return list;
    }

    if line.len() > MIN_CHAR_LENGTH {
        return match_single_indice(line);
    }

    None
}

/// Match Single Indice
///
/// # Description
/// Match byte indice i.e (#, >)
///
/// # Arguments
/// * `content` &str
///
/// # Return
/// TagToken
fn match_single_indice(content: &str) -> Option<TagToken> {
    let trimmed_content = bytes::trim_matches_content(content);
    // Getting the first character of the string
    let start_chars = content.split_at(MIN_CHAR_LENGTH);
    
    // match each byte of the byte array
    // See: https://www.reddit.com/r/rust/comments/f4usb4/pattern_matching_on_string_content_as_chars/fhtwe1f?utm_source=share&utm_medium=web2x
    // See: https://doc.rust-lang.org/edition-guide/rust-2018/slice-patterns.html
    match start_chars.0.as_bytes() {
        // #
        [bytes::HEADING, ..] => {
            Some(
                TagToken {
                    operator: TagOperator::Heading,
                    content: trimmed_content,
                    metas: Some(
                        TagMeta {
                            heading_kind: heading::get_heading_depth(content)
                        }
                    ),
                    ..Default::default()
                }
            )
        },
        // >
        [bytes::BLOCKQUOTE, ..] => Some(
            TagToken {
                operator: TagOperator::BlockQuotes,
                content: trimmed_content,
                ..Default::default()
            }
        ),
        // Defautl, return a BlockQuotes
        _ => None
    }
}
