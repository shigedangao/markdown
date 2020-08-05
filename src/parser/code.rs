use std::default::Default;
use lazy_static::lazy_static;
use regex::Regex;
use super::token::{Token, BaseOperator};

lazy_static!{
    static ref BLOCK_CODE: Regex = Regex::new(r"(```[a-z]*)").unwrap();
}

/// Get Block Code Token
///
/// # Description
/// Return a block of code token
/// 
/// # Arguments
/// * `&str` content
///
/// # Return
/// Option<Token>
pub fn get_block_code_token(content: &str, previous_token: &Option<Token>) -> Option<Token> {
    let matches = BLOCK_CODE.is_match(content);

    let mut token = Token {
        content: content.to_string(),
        operator: BaseOperator::BlockCodeStart,
        ..Default::default()
    };

    if matches && previous_token.is_none() {
        return Some(token);
    }

    if let Some(ptoken) = previous_token {
        if !matches && ptoken.operator == BaseOperator::BlockCodeStart ||
            !matches && ptoken.operator == BaseOperator::BlockCodeContent {
            token.operator = BaseOperator::BlockCodeContent;
            return Some(token);
        }

        if matches {
            token.operator = BaseOperator::BlockCodeEnd;
            return Some(token);
        }
    }

    None
}