use std::default::Default;

/// Heading LEvel
#[derive(Debug)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6
}

impl Default for HeadingLevel {
    fn default() -> Self { HeadingLevel::H1 }
}

/// Get Heading Depth
///
/// # Description
/// Get the heading depth by counting the number of #
pub fn get_heading_depth(line: &str) -> HeadingLevel {
    let res = line.rfind("#");
    if let Some(idx) = res {
        return match idx {
            0 => HeadingLevel::H1,
            1 => HeadingLevel::H2,
            2 => HeadingLevel::H3,
            3 => HeadingLevel::H4,
            4 => HeadingLevel::H5,
            5 => HeadingLevel::H6,
            _ => HeadingLevel::H2
        };
    }

    HeadingLevel::H1
}