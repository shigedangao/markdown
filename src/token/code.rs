use std::default::Default;
use std::collections::BTreeMap;
use std::clone::Clone;
use crate::parser::code;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct CodeToken {
    pub line: usize,
    pub content: String,
    pub operator: code::CodeOperator
}

/// Get Code Block Tokens
///
/// # Description
/// Get block code tokens
///
/// # Argument
/// * `content` &str
///
/// # Return
/// Option<CodeToken>
pub fn get_code_block_tokens(content: &str) -> BTreeMap<usize, CodeToken> {
    let mut code_tokens: BTreeMap<usize, CodeToken> = BTreeMap::new();
    let mut previous_token: Option<CodeToken> = None;

    for (idx, line) in content.lines().enumerate() {
        let operator = code::get_block_code_operator(line, &previous_token);
        if let Some(op) = operator {
            let token = CodeToken {
                line: idx,
                content: line.trim().to_string(),
                operator: op
            };

            previous_token = Some(token.clone());
            code_tokens.insert(idx, token);
        }
    }

    code_tokens
}