/// Operator
///
/// # Description
/// List of operator available in Markdown

pub mod bytes {
    // Operator -> #
    pub const HEADING: u8 = 35;
    // Operator -> *
    pub const UNORDERED_MUL: u8 = 42;
    // Operator -> -
    pub const UNORDERED_MINUS: u8 = 45;
    // Operator -> +
    pub const UNORDERED_PLUS: u8 = 43;
    // Operator -> >
    pub const BLOCKQUOTE: u8 = 62;

    /// Trim Matches Content
    ///
    /// # Description
    /// Trim the matches content
    ///
    /// # Arguments
    /// * `content` &str
    ///
    /// # Return
    /// String
    pub fn trim_matches_content(content: &str) -> String {
        let matches: &[char] = &[
            HEADING as char,
            UNORDERED_MUL as char,
            UNORDERED_MINUS as char,
            UNORDERED_PLUS as char,
            BLOCKQUOTE as char
        ];


        content.trim_matches(matches).trim().to_string()
    }

}

pub mod pattern {
    // TextOperator -> **
    pub const BOLD_STAR: &str = "**";
    // TextOperator -> __
    pub const BOLD_UNDER: &str = "__";
    // TextOperator -> *
    pub const ITALIC_STAR: &str = "*";
    // TextOeprator -> _
    pub const ITALIC_UN: &str = "_";
    // TextOperator -> ~~
    pub const STRIKE: &str = "~~";
    // CODE_PATTERN -> `
    pub const CODE_PATTERN: &str = "`";
}