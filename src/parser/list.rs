use lazy_static::lazy_static;
use regex::Regex;
use crate::token::tag::{TagToken, TagOperator};


lazy_static!{
    static ref ORDERED_LIST: Regex = Regex::new(r"^([1-9]\.)").unwrap();
    static ref UNORDED_LIST: Regex = Regex::new(r"^(\+\.)|(\*\.)|(\-\.)").unwrap();
}

/// Get Any List
///
/// # Description
/// Decorator over the <kind>_ list method
///
/// # Arguments
/// * `content` &str
///
/// # Return
/// Option<TagToken>
pub fn get_any_list(content: &str) -> Option<TagToken> {
    let unordered_list = get_unordered_list_token(content);
    if unordered_list.is_some() {
        return unordered_list;
    }

    let ordered_list = get_ordered_list_token(content);
    if ordered_list.is_some() {
        return ordered_list;
    }

    None
}

/// Get Ordered List
///
/// # Description
/// Get an ordered list from a markdown line content
///
/// # Arguments
/// * `content` &str
///
/// # Return
/// Option<TagToken>
fn get_ordered_list_token(content: &str) -> Option<TagToken> {
    let has = ORDERED_LIST.is_match(content);
    if !has {
        return None;
    }

    let trimmed_content = content
        .trim_start_matches(|c: char| c.is_numeric() && c == '.')
        .trim()
        .to_string();

    Some(
        TagToken {
            operator: TagOperator::OrderedList,
            content: trimmed_content,
            ..Default::default()
        }
    )
}

/// Get Unordered List TagToken
///
/// # Description
/// Get unordered list tokens
///
/// # Arguments
/// * `content` &str
///
/// # Return
/// Option<TagToken>
fn get_unordered_list_token(content: &str) -> Option<TagToken> {
    let has = UNORDED_LIST.is_match(content);
    if !has {
        return None;
    }

    let trimmed = UNORDED_LIST.replace_all(content, "");

    Some(
        TagToken {
            operator: TagOperator::UnorderedList,
            content: trimmed.as_ref().trim().to_string(),
            ..Default::default()
        }
    )
}