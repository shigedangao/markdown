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

            - I love eating baozi
        ";

        let res = lexer::tokens::get_tokens(content);
        print!("{:?}", res);
        assert!(!res.is_err());
    }
}