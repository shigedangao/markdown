use crate::token;
use crate::token::text::{TextToken};
use crate::token::tag::{TagToken, TagOperator, TagMeta};
use crate::parser::heading;
use crate::parser::code::CodeOperator;

#[test]
fn parse_markdown_blockquote() {
    let content = "
        > Hello my name is xiao
        I love eating baozi
    ";

    let res = token::get_textual_tokens(content);
    let token = res.unwrap();

    let blockquote: &TagToken = token.get(&1).unwrap().tag.as_ref().unwrap();
    
    assert_eq!(blockquote.line, 0);
    assert_eq!(blockquote.operator, TagOperator::BlockQuotes);
    assert_eq!(blockquote.content, "Hello my name is xiao");
    assert!(blockquote.metas.is_none());
}

#[test]
fn parse_markdown_heading_1() {
    let content = "
        # Heading
    ";

    let res = token::get_textual_tokens(content).unwrap();
    let heading: &TagToken = res.get(&1).unwrap().tag.as_ref().unwrap();

    let metas = heading.metas.as_ref().unwrap();
    
    assert_eq!(heading.operator, TagOperator::Heading);
    assert_eq!(metas.heading_kind, heading::HeadingLevel::H1);
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

    let res = token::get_textual_tokens(content).unwrap();
    let meta_1: &TagMeta = res.get(&1).unwrap().tag.as_ref().unwrap().metas.as_ref().unwrap();
    let meta_2: &TagMeta = res.get(&2).unwrap().tag.as_ref().unwrap().metas.as_ref().unwrap();
    let meta_3: &TagMeta = res.get(&3).unwrap().tag.as_ref().unwrap().metas.as_ref().unwrap();
    let meta_4: &TagMeta = res.get(&4).unwrap().tag.as_ref().unwrap().metas.as_ref().unwrap();
    let meta_5: &TagMeta = res.get(&5).unwrap().tag.as_ref().unwrap().metas.as_ref().unwrap();
    let meta_6: &TagMeta = res.get(&6).unwrap().tag.as_ref().unwrap().metas.as_ref().unwrap();

    assert_eq!(meta_1.heading_kind, heading::HeadingLevel::H1);
    assert_eq!(meta_2.heading_kind, heading::HeadingLevel::H2);
    assert_eq!(meta_3.heading_kind, heading::HeadingLevel::H3);
    assert_eq!(meta_4.heading_kind, heading::HeadingLevel::H4);
    assert_eq!(meta_5.heading_kind, heading::HeadingLevel::H5);
    assert_eq!(meta_6.heading_kind, heading::HeadingLevel::H6);
}

#[test]
fn parse_markdown_ordered_text() {
    let content = "
        1. Hello
        2. Foo bar
    ";

    let res = token::get_textual_tokens(content).unwrap();
    let first = res.get(&1).unwrap().tag.as_ref().unwrap();
    let second = res.get(&2).unwrap().tag.as_ref().unwrap();

    assert_eq!(first.operator, TagOperator::OrderedList);
    assert_eq!(second.operator, TagOperator::OrderedList);
}

#[test]
fn parse_markdown_unordered_text() {
    let content = "
        +. Hello
        -. Panda
        *. Tiger
    ";

    let res = token::get_textual_tokens(content).unwrap();
    let first = res.get(&1).unwrap().tag.as_ref().unwrap();
    let second = res.get(&2).unwrap().tag.as_ref().unwrap();
    let third = res.get(&3).unwrap().tag.as_ref().unwrap();

    assert_eq!(first.operator, TagOperator::UnorderedList);
    assert_eq!(second.operator, TagOperator::UnorderedList);
    assert_eq!(third.operator, TagOperator::UnorderedList);

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

    let res = token::get_textual_tokens(content).unwrap();
    let first_link = res.get(&1).unwrap().text.as_ref().unwrap();
    let second_link = res.get(&2).unwrap().text.as_ref().unwrap();

    assert!(first_link.metas.is_some());
    assert!(second_link.metas.is_some());

    let first_link_metas = first_link
        .metas
        .as_ref()
        .unwrap();

    assert_eq!(first_link_metas.links.as_ref().unwrap()[0].title, "hello");
    assert_eq!(first_link_metas.links.as_ref().unwrap()[0].url, "from laos");

    let second_link_metas = second_link
        .metas
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

    let res = token::get_textual_tokens(content).unwrap();
    let line = res.get(&1).unwrap().text.as_ref().unwrap();

    assert_eq!(line.content, "Hello from Taiwan here is the link of my [trip](https://link.foo)");
    
    let link = line
        .metas
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

    let res = token::get_textual_tokens(content).unwrap();
    let image = res.get(&1).unwrap().text.as_ref().unwrap();

    
    let image_metas = image
        .metas
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

    let res = token::get_textual_tokens(content).unwrap();
    let image = res.get(&2).unwrap().text.as_ref().unwrap();

    let image_metas = image
        .metas
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

    let res = token::get_textual_tokens(content).unwrap();
    let first_line = res.get(&1).unwrap().text.as_ref().unwrap();

    let first_line_text_metas = first_line
        .metas
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

    let second_line = res.get(&2).unwrap().text.as_ref().unwrap();

    let bold_vec = second_line
        .metas
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
            println!('hello');
        } 
        ```
    ";

    let res = token::get_code_tokens(content).unwrap();

    let code_header = res.get(&1).unwrap();
    assert_eq!(code_header.operator, CodeOperator::BlockCodeStart);
    assert_eq!(code_header.content, "```rust");

    let code_content_fn = res.get(&2).unwrap();
    assert_eq!(code_content_fn.operator, CodeOperator::BlockCodeContent);
    assert_eq!(code_content_fn.content, "fn main() {");

    let code_content_body = res.get(&3).unwrap();
    assert_eq!(code_content_body.operator, CodeOperator::BlockCodeContent);
    assert_eq!(code_content_body.content, "println!('hello');");

    let code_footer = res.get(&5).unwrap();
    assert_eq!(code_footer.operator, CodeOperator::BlockCodeEnd);
    assert_eq!(code_footer.content, "```");
}

#[test]
fn parse_block_code_wrap_text() {
    let content = "
        this is some code in JS

        ```js
        console.log('hello')
        ```

        lol
    ";

    let res = token::get_code_tokens(content).unwrap();

    let code_header = res.get(&3).unwrap();
    assert_eq!(code_header.operator, CodeOperator::BlockCodeStart);

    let code_content = res.get(&4).unwrap();
    assert_eq!(code_content.operator, CodeOperator::BlockCodeContent);

    let code_footer = res.get(&5).unwrap();
    assert_eq!(code_footer.operator, CodeOperator::BlockCodeEnd);
}

#[test]
fn parse_inline_code_token() {
    let content = "
        this is some `text`
        and here `too`
    ";

    let res = token::get_textual_tokens(content).unwrap();
    let first_line = res.get(&1).unwrap().text.as_ref().unwrap();
    let inline_code_content = first_line.metas
        .as_ref()
        .unwrap()
        .inline_code
        .as_ref()
        .unwrap();

    let second_line = res.get(&2).unwrap().text.as_ref().unwrap();
    let sec_inline_code_content = second_line.metas
        .as_ref()
        .unwrap()
        .inline_code
        .as_ref()
        .unwrap();

    assert_eq!(inline_code_content[0].word, "text");
    assert_eq!(inline_code_content[0].col.unwrap(), 13);

    assert_eq!(sec_inline_code_content[0].word, "too");
    assert_eq!(sec_inline_code_content[0].col.unwrap(), 9);
}