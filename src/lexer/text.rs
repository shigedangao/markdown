use regex::Regex;
use lazy_static::lazy_static;
use super::token::Token;
use super::external;

lazy_static!{
    static ref STRIKE_RE: Regex = Regex::new(r"(\~\~(.*?)\~\~)").unwrap();
    static ref BOLD_RE: Regex = Regex::new(r"(\*\*(.*?)\*\*)|(\_\_(.*?)\_\_)").unwrap();
    static ref ITALIC_RE: Regex = Regex::new(r"(\*(.*?)\*)|(\_(.*?)\_)").unwrap();
}

#[derive(Debug, PartialEq)]
pub enum TextOperator {
    Bold,
    Underline,
    Strike
}

pub struct TextMetas {
    images: Vec<external::LinkMeta>
}

/// Get Test Tokens
///
/// # Description
/// Get token for text object
///
/// # Arguments
/// * `token` token::Token
pub fn get_text_tokens(token: &Token) {
    // get images token
    let images = external::get_image_metas(&token.content);
    println!("{:?}", images);
    
    let links = external::get_link_metas(&token.content, images);
    println!("{:?}", links);
}
