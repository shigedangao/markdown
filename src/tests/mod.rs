mod token;
mod wrapper;

#[cfg(test)]
mod test_lib {
    use crate::{parse_code_markdown, parse_markdown};
    use crate::token::tag::TagOperator;
    use crate::parser::heading::HeadingLevel;
    use crate::parser::code::CodeOperator;

    #[test]
    fn expect_error_content_empty() {
        let res = parse_markdown("");
        assert!(res.is_err());

        let code_res = parse_code_markdown("");
        assert!(code_res.is_err());
    }

    #[test]
    fn expect_text_tag_tokens() {
        let content = "
            # Title

            A little *turtle* is walking down the **road**
            
            [A link](https://woxihuanchibaozi.com)

            > A baozi is so cute and tasty
        ";
        
        let res = parse_markdown(content).unwrap();

        // Title
        let title_tag = res.get(&1)
            .unwrap()
            .tag
            .as_ref()
            .unwrap();

        let title_metas = title_tag
            .metas
            .as_ref()
            .unwrap();

        assert_eq!(title_tag.content, "Title");
        assert_eq!(title_tag.line, 0);
        assert_eq!(title_tag.operator, TagOperator::Heading);
        assert_eq!(title_metas.heading_kind, HeadingLevel::H1);

        // Text
        let text = res.get(&3)
            .unwrap()
            .text
            .as_ref()
            .unwrap();

        assert_eq!(text.sanitize_content, "A little turtle is walking down the road");

        let text_bold_meta = text
            .metas
            .as_ref()
            .unwrap()
            .bold
            .as_ref()
            .unwrap();

        assert_eq!(text_bold_meta[0].word, "road");
        assert_eq!(text_bold_meta[0].col.unwrap(), 38);
        
        let text_italic_meta = text
            .metas
            .as_ref()
            .unwrap()
            .italic
            .as_ref()
            .unwrap();

        assert_eq!(text_italic_meta[0].word, "turtle");
        assert_eq!(text_italic_meta[0].col.unwrap(), 9);

        // Link
        let link_line = res.get(&5).unwrap();
        let link = link_line
            .text
            .as_ref()
            .unwrap();

        let link_metas = link
            .metas
            .as_ref()
            .unwrap()
            .links
            .as_ref()
            .unwrap();

        assert_eq!(link_metas[0].title, "A link");
        assert_eq!(link_metas[0].url, "https://woxihuanchibaozi.com");

        // Blockquote
        let blockquote_line = res.get(&7).unwrap();
        let blockquote = blockquote_line
            .tag
            .as_ref()
            .unwrap();

        let blockquote_metas = blockquote.metas.as_ref();

        assert_eq!(blockquote.operator, TagOperator::BlockQuote);
        assert_eq!(blockquote.content, "A baozi is so cute and tasty");
        assert!(blockquote_metas.is_none());
    }

    #[test]
    fn expect_code_tokens() {
        let content = "
            ```js
            let hello = 'foo';
            console.log(hello);
            ```

            ```rust
            fn main() {
                println!('hello')
            }
            ```
        ";

        let tokens = parse_code_markdown(content).unwrap();
        
        //// First block of code markdown
        let first_line = tokens.get(&1)
            .unwrap()
            .to_owned();

        assert_eq!(first_line.operator, CodeOperator::BlockCodeStart);
        assert_eq!(first_line.content, "```js");

        let second_line = tokens.get(&2)
            .unwrap()
            .to_owned();

        assert_eq!(second_line.operator, CodeOperator::BlockCodeContent);
        assert_eq!(second_line.content, "let hello = 'foo';");

        let third_line = tokens.get(&3)
            .unwrap()
            .to_owned();

        assert_eq!(third_line.operator, CodeOperator::BlockCodeContent);
        assert_eq!(third_line.content, "console.log(hello);");

        let end_line = tokens.get(&4)
            .unwrap()
            .to_owned();

        assert_eq!(end_line.operator, CodeOperator::BlockCodeEnd);
        assert_eq!(end_line.content, "```");

        //// Second block of code markdown
        let second_block_first_line = tokens.get(&6)
            .unwrap()
            .to_owned();

        assert_eq!(second_block_first_line.operator, CodeOperator::BlockCodeStart);
        assert_eq!(second_block_first_line.content, "```rust");

        let second_block_second_line = tokens.get(&7)
            .unwrap()
            .to_owned();

        assert_eq!(second_block_second_line.operator, CodeOperator::BlockCodeContent);
        assert_eq!(second_block_second_line.content, "fn main() {");

        let second_block_third_line = tokens.get(&8)
            .unwrap()
            .to_owned();

        assert_eq!(second_block_third_line.operator, CodeOperator::BlockCodeContent);
        assert_eq!(second_block_third_line.content, "println!('hello')");

        let second_block_fourth_line = tokens.get(&9)
            .unwrap()
            .to_owned();

        assert_eq!(second_block_fourth_line.operator, CodeOperator::BlockCodeContent);
        assert_eq!(second_block_fourth_line.content, "}");

        let second_block_end_line = tokens.get(&10)
            .unwrap()
            .to_owned();

        assert_eq!(second_block_end_line.operator, CodeOperator::BlockCodeEnd);
    }
}