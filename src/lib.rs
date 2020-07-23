use std::collections::BTreeMap;
mod lexer;
mod error;
mod tests;

#[cfg(test)]
mod markdown {
    use crate::lexer;

    #[test]
    fn parse_markdown() {
        let content = "### Hello
            My name is Xiao xiao

            > I'm a little panda

            1. I'm living in Sichuan
            2. I'm living in Seoul in a 12 floor building

            - I love eating baozi

            [hello](lol)

            i really like lao food [foo](bar)

            ![alt text](url)

            Today is very ~~sunny~~ gloomy ~~hot~~ actually cold

            I'm a **little** sick a *little*
        ";

        let res = lexer::token::get_tokens(content);
        assert!(!res.is_err());
    }
}

/// Parse Markdown
///
/// # Description
/// Wrapper around the lexer::token::get_tokens methods
///
/// # Arguments
/// * `content` &str
///
/// # Return
/// Result<BTreeMap<usize, lexer::token::Token, error::LexerError>>
pub fn parse_markdown(content: &str) -> Result<BTreeMap<usize, lexer::token::Token>, error::LexerError> {
    lexer::token::get_tokens(content)
}