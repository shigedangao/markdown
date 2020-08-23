## Markdown

A library to parse the github markdown syntax.

## Example

Below is an example of how to use the library

```rust
use xiaomarkdwn;

fn main() {
    let content = "
        Hello **my name is Xiao**
        
        ```js
            // some code
        ```
    ";

    let value = xiaomarkdwn.parse_markdown(content);
    let text = value.0.unwrap().get(&1).unwrap().text.unwrap();

    println!("{:?}", text.content);

    let text_metas = text.as_ref().unwrap().bold.unwrap().as_ref();
    println!("w: {:?} c: {:?}", text_metas.word, text_metas.col);
}
```

## Tests

Run test by running the commands

```shell
cargo test
```