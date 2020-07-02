mod lexer;
mod parser;
mod error;

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
        ";

        let res = lexer::token::get_tokens(content);
        print!("{:?}", res);
        assert!(!res.is_err());
    }
}