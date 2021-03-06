use std::collections::BTreeMap;
mod parser;
mod error;
mod token;
mod tests;

/// Parse Markdown
///
/// # Description
/// Wrapper around the lexer::token::get_tokens methods
///
/// # Arguments
/// * `content` &str
///
/// # Return
/// Result<BTreeMap<usize, lexer::token::Token, error::ParserError>>
pub fn parse_markdown(content: &str) -> Result<BTreeMap<usize, token::TextualToken>, error::ParserError> {
    token::get_textual_tokens(content)
}

/// Parse Code Markdown
///
/// # Description
/// Parse the markdown codeblock
///
/// # Arguments
/// * `content` &str
///
/// # Return
/// Result<BTreeMap<usize, token::code::CodeToken>, error::ParserError>
pub fn parse_code_markdown(content: &str) -> Result<BTreeMap<usize, token::code::CodeToken>, error::ParserError> {
    token::get_code_tokens(content)
}