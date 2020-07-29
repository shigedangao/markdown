use std::default::Default;
use std::sync::{Mutex};
use lazy_static::lazy_static;
use regex::Regex;
use super::token::{Token, BaseOperator};

lazy_static!{
    static ref BLOCK_CODE: Regex = Regex::new(r"(```[a-z]*)").unwrap();
    static ref FLAG: Mutex<BaseOperator> = Mutex::new(BaseOperator::Text);
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
pub fn get_block_code_token(content: &str) -> Option<Token> {
    let mut flag = FLAG.lock().unwrap();
    let matches = BLOCK_CODE.is_match(content);
    if !matches && *flag != BaseOperator::BlockCodeStart {
        return None;
    }

    let mut token = Token {
        content: content.to_string(),
        operator: BaseOperator::BlockCodeStart,
        ..Default::default()
    };

    // If something has match and that the matching process
    // has not begun then we return the token and the flag 
    // in order to begin the code matching process for the next lines
    if *flag == BaseOperator::Text {
        *flag = BaseOperator::BlockCodeStart;
        return Some(token);
    }

    // If nothing has match BUT the code block matching process has begun then
    // this mean that we're matching the code content itself
    if !matches && *flag == BaseOperator::BlockCodeStart {
        token.operator = BaseOperator::BlockCodeContent;
        return Some(token);
    }

    // If we're encountering a matches then this mean that we have 
    // arrived at the end of the code block thus ending the code block matching
    if matches {
        token.operator = BaseOperator::BlockCodeEnd;
    }
    
    *flag = BaseOperator::Text;
    Some(token)
}