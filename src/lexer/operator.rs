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
    // Operator -> `
    pub const CODE: u8 = 96;
    // Operator -> >
    pub const BLOCKQUOTE: u8 = 62;
}

pub mod pattern {
    // TextOperator -> ~~
    pub const BOLD: &str = "~~";
    // TextOperator -> *
    pub const ITALIC: &str = "*";
    // TextOperator -> ~~
    pub const STRIKE: &str = "~~";
}