use regex::Regex;
use lazy_static::lazy_static;
use super::token::Token;
use super::external;
use super::operator::pattern;

lazy_static!{
    static ref STRIKE_RE: Regex = Regex::new(r"(\~\~(.*?)\~\~)").unwrap();
    static ref BOLD_RE: Regex = Regex::new(r"(\*\*(.*?)\*\*)").unwrap();
    static ref ITALIC_RE: Regex = Regex::new(r"(\*(.*?)\*)").unwrap();
}

/// TextMetas
#[derive(Debug)]
pub struct TextMetas {
    images: Option<Vec<external::ImageMeta>>,
    links: Option<Vec<external::LinkMeta>>,
    bold: Option<Vec<TextOption>>,
    italic: Option<Vec<TextOption>>,
    strike: Option<Vec<TextOption>>
}

#[derive(Debug)]
pub struct TextOption {
    word: String,
    col: Option<usize>
}

/// Get Test Tokens
///
/// # Description
/// Get token for text object
///
/// # Arguments
/// * TextMetas
pub fn get_text_tokens(token: &Token) -> Option<TextMetas> {
    // get images token
    let images = external::get_image_metas(&token.content);
    let links = external::get_link_metas(&token.content, &images);
    let strike = get_kind_content(&token.content, pattern::STRIKE, &STRIKE_RE);
    let bold = get_kind_content(&token.content, pattern::BOLD, &BOLD_RE);
    let italic = get_kind_content(&token.content, pattern::ITALIC, &ITALIC_RE);

    Some(TextMetas {
        images,
        links,
        bold,
        italic,
        strike
    })

}

/// Get Kind Content
///
/// # Description
/// Get a list of kind content on the line's content
///
/// # Arguments
/// * `content` &String
/// * `pattern` &str
/// * `re` Regex
///
/// # Return
/// Option<Vec<TextOption>>
fn get_kind_content(content: &String, pattern: &str, re: &Regex) -> Option<Vec<TextOption>> {
    let captures = re.captures_iter(content);

    let k: Vec<TextOption> = captures
        .map(|c| {
            let word = c.get(2).unwrap().as_str();

            TextOption {
                word: word.to_string(),
                col: get_indices(pattern, word, content)    
            }
        })
        .collect();

    if k.len() == 0 {
        return None;
    }

    Some(k)
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