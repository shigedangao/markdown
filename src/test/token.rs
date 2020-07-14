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

#[test]
fn parse_markdown_heading_all() {
    let content = "
        # Heading
        ## Heading
        ### Heading
        #### Heading
        ##### Heading
        ###### Heading
    ";

    let res = lexer::token::get_tokens(content).unwrap();
    let meta_1: &Meta = res.get(0).unwrap().metas.as_ref().unwrap();
    let meta_2: &Meta = res.get(1).unwrap().metas.as_ref().unwrap();
    let meta_3: &Meta = res.get(2).unwrap().metas.as_ref().unwrap();
    let meta_4: &Meta = res.get(3).unwrap().metas.as_ref().unwrap();
    let meta_5: &Meta = res.get(4).unwrap().metas.as_ref().unwrap();
    let meta_6: &Meta = res.get(5).unwrap().metas.as_ref().unwrap();

    assert_eq!(meta_1.heading.as_ref().unwrap(), &heading::HeadingLevel::H1);
    assert_eq!(meta_2.heading.as_ref().unwrap(), &heading::HeadingLevel::H2);
    assert_eq!(meta_3.heading.as_ref().unwrap(), &heading::HeadingLevel::H3);
    assert_eq!(meta_4.heading.as_ref().unwrap(), &heading::HeadingLevel::H4);
    assert_eq!(meta_5.heading.as_ref().unwrap(), &heading::HeadingLevel::H5);
    assert_eq!(meta_6.heading.as_ref().unwrap(), &heading::HeadingLevel::H6);
}

#[test]
fn parse_markdown_ordered_text() {
    let content = "
        1. Hello
        2. Foo bar
    ";

    let res = lexer::token::get_tokens(content).unwrap();
    let first = res.get(0).unwrap();
    let second = res.get(1).unwrap();

    assert_eq!(first.operator, BaseOperator::OrderedList);
    assert_eq!(second.operator, BaseOperator::OrderedList);
}

#[test]
fn parse_markdown_unordered_text() {
    let content = "
        +. Hello
        -. Panda
        *. Tiger
    ";

    let res = lexer::token::get_tokens(content).unwrap();
    let first = res.get(0).unwrap();
    let second = res.get(1).unwrap();
    let third = res.get(2).unwrap();

    assert_eq!(first.operator, BaseOperator::UnorderedList);
    assert_eq!(second.operator, BaseOperator::UnorderedList);
    assert_eq!(third.operator, BaseOperator::UnorderedList);

    assert_eq!(first.content, "Hello");
    assert_eq!(second.content, "Panda");
    assert_eq!(third.content, "Tiger");
}