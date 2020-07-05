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
pub enum TextOptionKind {
    Bold,
    Underline,
    Strike
}

pub struct TextMetas {
    images: Vec<external::ImageMeta>,
    links: Vec<external::LinkMeta>
}

#[derive(Debug)]
pub struct TextOption {
    kind: TextOptionKind,
    word: String,
    col: Option<usize>
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

    let strike = get_strikethrough_content(&token.content);
    println!("{:?}", strike);
}

/// Get Strikethrough content
///
/// # Description
/// Get a list of strikethrough content on the line's content
///
/// # Arguments
/// * `content` String
fn get_strikethrough_content(content: &String) -> Option<Vec<TextOption>> {
    let captures = STRIKE_RE.captures_iter(content);

    let strike: Vec<TextOption> = captures
        .map(|c| {
            let word = c.get(2).unwrap().as_str();

            TextOption {
                kind: TextOptionKind::Strike,
                word: word.to_string(),
                col: get_indices("~~", word, content)
            }
        })
        .collect();

    Some(strike)
}

/// Get Indices
///
/// # Description
/// Return the first index of the word that is match
///
/// # Arguments
/// * `pattern` &str
/// * `word` &str
///
/// # Return
/// usize
fn get_indices(pattern: &str, word: &str, content: &str) -> Option<usize> {
    let mut indices = String::new();
    indices.push_str(pattern);
    indices.push_str(word);
    indices.push_str(pattern);

    let matches: Vec<_> = content.match_indices(indices.as_str()).collect();
    let value = matches.first();
    if let Some(v) = value {
        return Some(v.0);
    }

    None
}