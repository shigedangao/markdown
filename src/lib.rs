mod lexer;
mod parser;
mod error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[cfg(test)]
mod markdown {
    fn parse_markdown() -> Result<(), ()> {

    }
}