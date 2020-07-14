use lazy_static::lazy_static;
use regex::Regex;
use super::token::{Token, BaseOperator};


lazy_static!{
    static ref ORDERED_LIST: Regex = Regex::new(r"([1-9]\.)").unwrap();
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
/// Option<Token>
pub fn get_ordered_list_token(content: &str) -> Option<Token> {
    let has = ORDERED_LIST.is_match(content);
    if !has {
        return None;
    }

    let trimmed_content = content
        .trim_start_matches(|c: char| c.is_numeric() && c == '.')
        .trim()
        .to_string();

    Some(Token {
        operator: BaseOperator::OrderedList,
        content: trimmed_content,
        ..Default::default()
    })
}