use crate::parser::token::{Token, BaseOperator, Meta};
use crate::parser::external::{LinkMeta};
use crate::parser::{heading, token};

#[test]
fn parse_markdown_blockquote() {
    let content = "
        > Hello my name is xiao
        I love eating baozi
    ";

    let res = token::get_tokens(content);
    let token = res.unwrap();

    let blockquote: &Token = token.get(&1).unwrap();
    
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

    let res = token::get_tokens(content).unwrap();
    let heading: &Token = res.get(&1).unwrap();

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

    let res = token::get_tokens(content).unwrap();
    let meta_1: &Meta = res.get(&1).unwrap().metas.as_ref().unwrap();
    let meta_2: &Meta = res.get(&2).unwrap().metas.as_ref().unwrap();
    let meta_3: &Meta = res.get(&3).unwrap().metas.as_ref().unwrap();
    let meta_4: &Meta = res.get(&4).unwrap().metas.as_ref().unwrap();
    let meta_5: &Meta = res.get(&5).unwrap().metas.as_ref().unwrap();
    let meta_6: &Meta = res.get(&6).unwrap().metas.as_ref().unwrap();

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

    let res = token::get_tokens(content).unwrap();
    let first = res.get(&1).unwrap();
    let second = res.get(&2).unwrap();

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

    let res = token::get_tokens(content).unwrap();
    let first = res.get(&1).unwrap();
    let second = res.get(&2).unwrap();
    let third = res.get(&3).unwrap();

    assert_eq!(first.operator, BaseOperator::UnorderedList);
    assert_eq!(second.operator, BaseOperator::UnorderedList);
    assert_eq!(third.operator, BaseOperator::UnorderedList);

    assert_eq!(first.content, "Hello");
    assert_eq!(second.content, "Panda");
    assert_eq!(third.content, "Tiger");
}

#[test]
fn parse_links() {
    let content = "
        [hello](from laos)
        [hello](you)
    ";

    let res = token::get_tokens(content).unwrap();
    let first_link = res.get(&1).unwrap();
    let second_link = res.get(&2).unwrap();

    assert_eq!(first_link.operator, BaseOperator::Text);
    assert_eq!(second_link.operator, BaseOperator::Text);

    assert!(first_link.metas.as_ref().unwrap().text_metas.is_some());
    assert!(second_link.metas.as_ref().unwrap().text_metas.is_some());

    let first_link_metas = first_link
        .metas
        .as_ref()
        .unwrap()
        .text_metas
        .as_ref()
        .unwrap();

    assert_eq!(first_link_metas.links.as_ref().unwrap()[0].title, "hello");
    assert_eq!(first_link_metas.links.as_ref().unwrap()[0].url, "from laos");

    let second_link_metas = second_link
        .metas
        .as_ref()
        .unwrap()
        .text_metas
        .as_ref()
        .unwrap();

    assert_eq!(second_link_metas.links.as_ref().unwrap()[0].title, "hello");
    assert_eq!(second_link_metas.links.as_ref().unwrap()[0].url, "you");
}

#[test]
fn parse_links_in_text() {
    let content = "
        Hello from Taiwan here is the link of my [trip](https://link.foo)
    ";

    let res = token::get_tokens(content).unwrap();
    let line = res.get(&1).unwrap();

    assert_eq!(line.operator, BaseOperator::Text);
    assert_eq!(line.content, "Hello from Taiwan here is the link of my [trip](https://link.foo)");
    
    let link = line.metas
        .as_ref()
        .unwrap()
        .text_metas
        .as_ref()
        .unwrap();

    assert_eq!(link.links.as_ref().unwrap()[0].title, "trip");
    assert_eq!(link.links.as_ref().unwrap()[0].url, "https://link.foo");
}

#[test]
fn parse_images() {
    let content = "
        ![a chinese temple](https://chinese-temple.com)
    ";

    let res = token::get_tokens(content).unwrap();
    let image = res.get(&1).unwrap();

    assert_eq!(image.operator, BaseOperator::Text);
    
    let image_metas = image
        .metas
        .as_ref()
        .unwrap()
        .text_metas
        .as_ref()
        .unwrap();

    assert_eq!(image_metas.images.as_ref().unwrap()[0].url, "https://chinese-temple.com");
    assert_eq!(image_metas.images.as_ref().unwrap()[0].alt_text, "a chinese temple");
}

#[test]
fn parse_images_within_content() {
    let content = "
        Hello this is a sample text
        here is an image within this text ![bugcat capoo](貓貓)
    ";

    let res = token::get_tokens(content).unwrap();
    let image = res.get(&2).unwrap();

    assert_eq!(image.operator, BaseOperator::Text);
    let image_metas = image
        .metas
        .as_ref()
        .unwrap()
        .text_metas
        .as_ref()
        .unwrap();

    assert_eq!(image_metas.images.as_ref().unwrap()[0].url, "貓貓");
    assert_eq!(image_metas.images.as_ref().unwrap()[0].alt_text, "bugcat capoo");
}

#[test]
fn parse_text_style() {
    let content = "
        This is a ~~strike~~ *text* lol 
        ha **End** ha
    ";

    let res = token::get_tokens(content).unwrap();
    let first_line = res.get(&1).unwrap();

    let first_line_text_metas = first_line
        .metas
        .as_ref()
        .unwrap()
        .text_metas
        .as_ref()
        .unwrap();

    let strike_vec = first_line_text_metas
        .strike
        .as_ref()
        .unwrap();

    assert_eq!(strike_vec[0].word, "strike");
    assert_eq!(strike_vec[0].col.unwrap(), 10);

    let italic_vec = first_line_text_metas
        .italic
        .as_ref()
        .unwrap();

    assert_eq!(italic_vec[0].word, "text");
    assert_eq!(italic_vec[0].col.unwrap(), 21);

    let second_line = res.get(&2).unwrap();

    let bold_vec = second_line
        .metas
        .as_ref()
        .unwrap()
        .text_metas
        .as_ref()
        .unwrap()
        .bold
        .as_ref()
        .unwrap();

    assert_eq!(bold_vec[0].word, "End");
    assert_eq!(bold_vec[0].col.unwrap(), 3);
}

#[test]
fn parse_block_code_content() {
    let content = "
        ```rust
        fn main() {
            println!('hello')
        } 
        ```
    ";

    let res = token::get_tokens(content).unwrap();
    println!("{:?}", res);


    let code_header = res.get(&1).unwrap();
    assert_eq!(code_header.operator, BaseOperator::BlockCodeStart);
    assert_eq!(code_header.content, "```rust");

    let code_content_fn = res.get(&2).unwrap();
    assert_eq!(code_content_fn.operator, BaseOperator::BlockCodeContent);
    assert_eq!(code_content_fn.content, "fn main() {");

    let code_content_body = res.get(&3).unwrap();
    assert_eq!(code_content_body.operator, BaseOperator::BlockCodeContent);
    assert_eq!(code_content_body.content, "println!('hello')");

    let code_footer = res.get(&5).unwrap();
    assert_eq!(code_footer.operator, BaseOperator::BlockCodeEnd);
    assert_eq!(code_footer.content, "```");
}