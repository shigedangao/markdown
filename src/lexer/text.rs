use regex::Regex;
use lazy_static::lazy_static;
use super::token::Token;

lazy_static!{
    static ref LINK_RE: Regex = Regex::new(r"(?:__|[*#])|\[(.*?)\]\(.*?\)").unwrap();
    static ref IMG_RE: Regex = Regex::new(r"(?:__|[*#])|!\[(.*?)\]\(.*?\)").unwrap();
    static ref STRIKE_RE: Regex = Regex::new(r"(\~\~(.*?)\~\~)").unwrap();
    static ref BOLD_RE: Regex = Regex::new(r"(\*\*(.*?)\*\*)|(\_\_(.*?)\_\_)").unwrap();
    static ref ITALIC_RE: Regex = Regex::new(r"(\*(.*?)\*)|(\_(.*?)\_)").unwrap();
}

#[derive(Debug, PartialEq)]
pub enum TextOperator {
    Bold,
    Underline,
    Strike,
    Link,
    Image
}

pub struct LineMeta {
    
}

/// Get Test Tokens
///
/// # Description
/// Get token for text object
///
/// # Arguments
/// * `token` token::Token
pub fn get_text_tokens(token: &Token) {
    // use split_terminator
    get_link_object(&token.content);
}

/// Get Link Object
///
/// # Description
/// Get link object from line
///
/// # Arguments
/// * `line` &str
///
/// # Return
fn get_link_object(content: &String) {
    let captures = LINK_RE.captures_iter(content);
    for c in captures {
        println!("Capture {:?}", c);
    }
}