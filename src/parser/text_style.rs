use regex::Regex;
use lazy_static::lazy_static;
use super::external;
use super::operator::pattern;

lazy_static!{
    static ref STRIKE_RE: Regex = Regex::new(r"(\~\~(.*?)\~\~)").unwrap();
    static ref BOLD_RE: Regex = Regex::new(r"(\*\*(.*?)\*\*)").unwrap();
    static ref ITALIC_RE: Regex = Regex::new(r"(\*(.*?)\*)").unwrap();
    static ref INLINE_CODE: Regex = Regex::new(r"(`([a-z].*)`)").unwrap();
}

#[derive(Debug)]
pub struct TextOption {
    pub word: String,
    pub col: Option<usize>
}

/// TextMetas
#[derive(Debug)]
pub struct TextMetas {
    pub images: Option<Vec<external::ImageMeta>>,
    pub links: Option<Vec<external::LinkMeta>>,
    pub bold: Option<Vec<TextOption>>,
    pub italic: Option<Vec<TextOption>>,
    pub strike: Option<Vec<TextOption>>,
    pub inline_code: Option<Vec<TextOption>>
}

/// Get Test Metas
///
/// # Description
/// Get token for text object
///
/// # Arguments
/// * TextMetas
pub fn get_text_metas(content: &str) -> Option<TextMetas> {
    // get images token
    let images = external::get_image_metas(content);
    let links = external::get_link_metas(content, &images);
    let strike = get_kind_content(content, pattern::STRIKE, &STRIKE_RE);
    let bold = get_kind_content(content, pattern::BOLD, &BOLD_RE);
    let italic = get_kind_content(content, pattern::ITALIC, &ITALIC_RE);
    let inline_code = get_kind_content(content, pattern::CODE_PATTERN, &INLINE_CODE);

    Some(TextMetas {
        images,
        links,
        bold,
        italic,
        strike,
        inline_code
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
fn get_kind_content(content: &str, pattern: &str, re: &Regex) -> Option<Vec<TextOption>> {
    let captures = re.captures_iter(content);
    let k: Vec<TextOption> = captures
        .map(|c| {
            // Index 2 is the content of the capture
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
/// Return the first index of the word that is match. This is use to retrieve the column index
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