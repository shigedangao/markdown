/// Lexer
///
/// # Description
/// There are multiple markdown format. For the sake of simplicity we're going to follow the markdown cheatset
/// from github https://github.com/adam-p/markdown-here/wiki/Markdown-Cheatsheet
///
/// # Lexer
/// First we need to define the main token that we need to use in order to get the basic match
/// Matchers that we'll do for each line:
/// - # -> heading 
/// - --- / *** / ___ -> horizontal rules
/// - * / - / + -> unordered list
/// - <number>. -> ordered list
/// - ``` -> code
/// - > -> blockquotes
/// - <..></..> -> html
///
/// Other token are gonna be treat as a content marker and will later be process by a content lexer
/// The content lexer will have to match the following regex for each line
/// - *...* -> bold
/// - _..._ -> underscore
/// - --...-- -> strikethrought
/// - []() -> link
/// - ![]() -> image
use std::default::Default;
use crate::error::LexerError;

#[derive(Debug)]
enum BaseOperator {
    Heading,
    UnorderedList,
    Code,
    BlockQuotes,
    Text
}

impl Default for BaseOperator {
    fn default() -> Self { BaseOperator::Text }
}

#[derive(Default)]
struct Token { 
    line: usize,
    content: String,
    token: BaseOperator
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
pub fn get_tokens(content: &str) -> Result<(), LexerError> {
    for (idx, line) in content.lines().enumerate() {
        let mut token = match_basic_token(line);
        token.content = line.to_string();
        token.line = idx;
    }

    Ok(())
}

/// Match Basic Token
///
/// # Description
/// Match the basic token for each line
fn match_basic_token(line: &str) -> Token {
    let chars = line.split_at(3);
  
    match chars.0 {
        "#" => Token { token: BaseOperator::Heading, ..Default::default() },
        "*" | "-" | "+" => Token { token: BaseOperator::UnorderedList, ..Default::default() },
        "```" => Token { token: BaseOperator::Code, ..Default::default() },
        ">" => Token { token: BaseOperator::BlockQuotes, ..Default::default() },
        _ => Token { token: BaseOperator::Text, ..Default::default() }
    }
}
