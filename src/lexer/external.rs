use regex::Regex;
use lazy_static::lazy_static;
use super::token;

// Creating static variable to match regex for external resource
// such as link and images
lazy_static!{
    static ref LINK_RE: Regex = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
    static ref IMG_RE: Regex = Regex::new(r"!\[(.*?)\]\((.*?)\)").unwrap();
}

#[derive(Debug)]
pub struct LinkMeta {
    pub title: String,
    pub url: String
}

#[derive(Debug)]
pub struct ImageMeta {
    pub alt_text: String,
    pub url: String
}


/// Get Link Object
///
/// # Description
/// Get link object from line
///
/// # Arguments
/// * `line` &String
///
/// # Return
/// * `Vec<LinkMeta>`
pub fn get_link_metas(content: &String) -> Option<Vec<LinkMeta>> {
    let captures = LINK_RE.captures_iter(content);

    let links: Vec<LinkMeta> = captures
        .map(|link| {
            let label = link.get(1);
            let url = link.get(2);

            LinkMeta {
                title: label.unwrap().as_str().to_string(),
                url: url.unwrap().as_str().to_string()
            }
        })
        .collect();

    if links.len() == 0 {
        return None;
    }

    Some(links)
}

/// Get Image Metas
///
/// # Description
/// Get Image from the text field either inline or block images
///
/// # Arguments
/// * `content` &String
///
/// # Return
/// Option<Vec<ImageMeta>>
pub fn get_image_metas(content: &String) -> Option<Vec<ImageMeta>> {
    let captures = IMG_RE.captures_iter(content);

    let images: Vec<ImageMeta> = captures
        .map(|img| {
            let alt_text = img.get(1).unwrap().as_str().to_string();
            let url = img.get(2).unwrap().as_str().to_string();

            ImageMeta {
                alt_text,
                url
            }
        })
        .collect();

    if images.len() == 0 {
        return None;
    }

    Some(images)
}

/// Resolve Image Link Conflict
///
/// # Description
/// Resolve the issue when a link is generate instead of being an image
/// this can happened due to the fact that Rust regex doesn't support the lookaround feature
fn resolve_image_link_conflict(images: &Vec<ImageMeta>, links: &Vec<LinkMeta>) {

}