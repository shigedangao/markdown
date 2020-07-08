mod lexer;
mod parser;
mod error;
mod test;

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

pub fn parse_markdown(content: &str) {
    lexer::token::get_tokens(content);
}