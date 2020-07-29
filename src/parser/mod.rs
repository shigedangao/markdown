/// Parser
///
/// # Description
/// There are multiple markdown format. For the sake of simplicity we're going to follow the markdown cheatset
/// from github https://github.com/adam-p/markdown-here/wiki/Markdown-Cheatsheet
///
/// # Parser
/// First we need to define the main token that we need to use in order to get the basic match
/// Matchers that we'll do for each line:
/// - # -> heading 
/// - --- / *** / ___ -> horizontal rules
/// - * / - / + -> unordered list
/// - <number>. -> ordered list
/// - ``` -> code
/// - > -> blockquotes
///
/// Other token are gonna be treat as a content marker and will later be process by a content lexer
/// The content parser will have to match the following regex for each line
/// - **...** | __...__-> bold
/// - _..._ | *...* -> italic
/// - ~~...~~ -> strikethrought
/// - []() -> link
/// - ![]() -> image
pub mod token;
pub mod external;
pub mod heading;
mod code;
mod list;
mod operator;
mod text;
