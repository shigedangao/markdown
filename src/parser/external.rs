use std::clone::Clone;
use regex::Regex;
use lazy_static::lazy_static;

// Creating static variable to match regex for external resource
// such as link and images
lazy_static!{
    static ref LINK_RE: Regex = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
    static ref IMG_RE: Regex = Regex::new(r"!\[(.*?)\]\((.*?)\)").unwrap();
}

#[derive(Debug, Clone)]
pub struct LinkMeta {
    pub title: String,
    pub url: String
}

#[derive(Debug, Clone)]
pub struct ImageMeta {
    pub alt_text: String,
    pub url: String
}


/// Get Link Object
///
/// # Description
/// Get link object from content of a line
///
/// # Arguments
/// * `content` &String
/// * `images` &Vec<ImageMeta>
///
/// # Return
/// * `Option<Vec<LinkMeta>>`
pub fn get_link_metas(content: &String, images: &Option<Vec<ImageMeta>>) -> Option<Vec<LinkMeta>> {
    let captures = LINK_RE.captures_iter(content);
    let mut imgs = &Vec::new();
    
    if let Some(img) = images {
        imgs = img;
    } 

    let links: Vec<LinkMeta> = captures
        .filter_map(|link| {
            let title = link.get(1).unwrap().as_str();
            let url = link.get(2).unwrap().as_str();

            if is_not_image(&imgs, title, url) {
                return Some(LinkMeta {
                    title: title.to_string(),
                    url: url.to_string()
                });
            }

            None
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

/// Is Not Image
///
/// # Description
/// Check that the link that is match isn't an image. Rust regex doesn't support lookaround feature.
/// As a result we're not able to make a distinction between images and links itself. This walkaround
/// goal is for each link to check if it already exist as an image. If so then skip the linkitem during
/// the generation of the links tree
///
/// # Arguments
/// * `images` &Vec<ImageMeta>
/// * `link` LinkMeta
///
/// # Return
/// bool
fn is_not_image(images: &Vec<ImageMeta>, link_title: &str, link_url: &str) -> bool {
    let images: Vec<&ImageMeta> = images
        .into_iter()
        .filter(|img| img.url == link_url && img.alt_text == link_title)
        .collect();

    if images.len() == 0 {
        return true;
    }

    false
}