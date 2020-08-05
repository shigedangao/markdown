use std::default::Default;
use std::clone::Clone;
use lazy_static::lazy_static;
use regex::Regex;
use crate::token::code::{CodeToken};

lazy_static!{
    static ref BLOCK_CODE: Regex = Regex::new(r"(```[a-z]*)").unwrap();
}

#[derive(Debug, PartialEq, Clone)]
pub enum CodeOperator {
    BlockCodeStart,
    BlockCodeContent,
    BlockCodeEnd
}

impl Default for CodeOperator {
    fn default() -> Self { CodeOperator::BlockCodeStart }
}

/// Get Block Code Operator
///
/// # Description
/// Return a block of code token
/// 
/// # Arguments
/// * `&str` content
/// * `&Option<Token>` previous_token
///
/// # Return
/// Option<Token>
pub fn get_block_code_operator(content: &str, previous_token: &Option<CodeToken>) -> Option<CodeOperator> {
    let matches = BLOCK_CODE.is_match(content);

    if matches && previous_token.is_none() {
        return Some(CodeOperator::BlockCodeStart);
    }

    if let Some(ptoken) = previous_token {
        // Match if we detect a new block code
        if !matches && ptoken.operator == CodeOperator::BlockCodeStart ||
            !matches && ptoken.operator == CodeOperator::BlockCodeContent {
            return Some(CodeOperator::BlockCodeContent);
        }

        if matches && ptoken.operator == CodeOperator::BlockCodeContent {
            return Some(CodeOperator::BlockCodeEnd);
        }

        if matches {
            return Some(CodeOperator::BlockCodeStart);
        }
    }

    None
}

/// Is Code
///
/// # Description
/// Return if the content is a code line
///
/// # Argument
/// * `content` &str
///
/// # Return
/// bool
pub fn is_code(content: &str) -> bool {
    BLOCK_CODE.is_match(content)
} 