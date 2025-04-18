mod scanner;
mod token;
mod parser;
mod ast;

use token::{Token, tokenize};
use parser::Parser;

fn main() {
    let source = r#"
        let x: int = 5 + 3 * 2;
    "#;

    let tokens: Vec<Token> = tokenize(source.trim());
    println!("{:?}", tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    println!("{:#?}", ast);
}
