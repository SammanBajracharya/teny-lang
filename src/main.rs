mod scanner;

use scanner::{Token, Scanner};

fn main() {
    let mut scanner = Scanner::new("a + b;");

    loop {
        let token = scanner.scan();
        if let Token::EOF = token {
            break;
        }
        println!("{:?}", token);
    }
}
