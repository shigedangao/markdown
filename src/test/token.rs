use crate::lexer;
use crate::lexer::token::{Token, BaseOperator, Meta};
use crate::lexer::heading;

#[test]
fn parse_markdown_blockquote() {
    let content = "
        > Hello my name is xiao
        I love eating baozi
    ";

    let res = lexer::token::get_tokens(content);
    let token = res.unwrap();

    let blockquote: &Token = token.get(0).unwrap();
    
    assert_eq!(blockquote.line, 1);
    assert_eq!(blockquote.operator, BaseOperator::BlockQuotes);
    assert_eq!(blockquote.content, "Hello my name is xiao");
    assert!(blockquote.metas.is_none());
}

#[test]
fn parse_markdown_heading_1() {
    let content = "
        # Heading
    ";

    let res: Vec<Token> = lexer::token::get_tokens(content).unwrap();
    let heading: &Token = res.get(0).unwrap();

    let metas = heading.metas.as_ref().unwrap();
    
    assert_eq!(heading.operator, BaseOperator::Heading);
    assert_eq!(metas.heading.as_ref().unwrap(), &heading::HeadingLevel::H1);
}