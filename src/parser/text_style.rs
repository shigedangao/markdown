use std::clone::Clone;
use regex::Regex;
use lazy_static::lazy_static;
use super::external;
use super::operator::pattern;

lazy_static!{
    static ref STRIKE_RE: Regex = Regex::new(r"(\~\~(.*?)\~\~)").unwrap();
    static ref BOLD_ST_RE: Regex = Regex::new(r"(\*\*(.*?)\*\*)").unwrap();
    static ref BOLD_UN_RE: Regex = Regex::new(r"(__(.*?)__)").unwrap();
    static ref ITALIC_RE: Regex = Regex::new(r"(\*(.*?)\*)").unwrap();
    static ref ITALIC_UN_RE: Regex = Regex::new(r"(_(.*?)_)").unwrap();
    static ref INLINE_CODE: Regex = Regex::new(r"(`([a-z].*)`)").unwrap();
}

#[derive(Debug, Clone)]
pub struct TextOption {
    pub word: String,
    pub col: Option<usize>
}

/// TextMetas
#[derive(Debug, Clone)]
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
    let bold_star = get_kind_content(content, pattern::BOLD_STAR, &BOLD_ST_RE);
    let bold_under = get_kind_content(content, pattern::BOLD_UNDER, &BOLD_UN_RE);
    let italic_star = get_kind_content(content, pattern::ITALIC_STAR, &ITALIC_RE);
    let italic_under = get_kind_content(content, pattern::ITALIC_UN, &ITALIC_UN_RE);
    let inline_code = get_kind_content(content, pattern::CODE_PATTERN, &INLINE_CODE);

    let bold = merge_option_vec::<TextOption>(bold_star, bold_under);
    let italic = merge_option_vec(italic_star, italic_under);

    Some(TextMetas {
        images,
        links,
        bold,
        italic,
        strike,
        inline_code
    })

}

/// Sanitize Content
///
/// # Description
/// Clean the content of any markdown style character
///
/// # Arguments
/// * `content` &str
///
/// # Return
/// String
pub fn sanitze_content(content: &str) -> String {
    let wobold = content.replace(pattern::BOLD_STAR, "");
    let woboldun = wobold.replace(pattern::BOLD_UNDER, "");
    let wostrike = woboldun.replace(pattern::STRIKE, "");
    let woitalic = wostrike.replace(pattern::ITALIC_STAR, "");
    let woitalicun = woitalic.replace(pattern::ITALIC_UN, "");
    let wocode = woitalicun.replace(pattern::CODE_PATTERN, "");

    wocode
        .trim()
        .to_string()
}

/// Merge Option Vec
///
/// # Description
/// Merge option vector into one
///
/// # Arguments
/// * `a` Option<Vec<T>>
/// * `b` Option<Vec<T>>
///
/// # Return
/// * `Option<Vec<T<>`
fn merge_option_vec<T>(a: Option<Vec<T>>, b: Option<Vec<T>>) -> Option<Vec<T>> {
    if let Some(mut vec_a) = a {
        if let Some(mut vec_b) = b {
            vec_a.append(vec_b.as_mut());
        }

        return Some(vec_a);
    }

    a
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
        .filter_map(|c| {
            // Index 2 is the content of the capture
            let word = c.get(2).map_or("", |v| v.as_str());
            if word.is_empty() {
                return None;
            }

            Some(TextOption {
                word: word.to_string(),
                col: get_indices(pattern, word, content)    
            })
        })
        .collect();

    if k.is_empty() {
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