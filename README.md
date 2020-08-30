## Markdown

A library to parse the github markdown syntax.

## Using the library

The library is pretty simple to use. Only 2 methods are exposed.

```rust
xiaomarkdwn::{parse_markdown, parse_code_markdown}
```

- parse_markdown is use to parse any content except code block
- parse_code_markdown is use to parse any code block except the content

Both of this method return a BTreeMap

## Example

```rust
use xiaomarkdwn;

fn main() {
    let content = "
        Hello **my name is Xiao**
    ";

    let parse_result = xiaomarkdwn.parse_markdown(content).unwrap();
    
    // get the first line
    let first_line = parse_result[&0].unwrap().to_owned();
    let text = first_line.text.unwrap();
    let text_metas = text.metas.unwrap();
    
    println!("{:?}", text.content);
    println!("{:?}", text_metas.bold[0].word); // will output 'my name is xiao'
    println!("{:?}", text_metas.bold[0].col); // will output 6
}
```

A set of examples will be available on the `examples` folder

## Tests

Run test by running the commands

```shell
cargo test
```