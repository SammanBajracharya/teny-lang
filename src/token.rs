use crate::scanner::Scanner;

#[derive(Clone, Debug)]
pub enum Token {
    Symbol(Symbol),
    Operator(Operator),
    Keyword(Keyword),
    Identifiers(String),
    DataTypes(DataTypes),
    Unknown(char),
    EOF,
}
#[derive(Clone, Debug)]
pub enum DataTypes {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Char(char),
    String(String),
}
#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Symbol {
    Semicolon,
    LParen,
    RParen,
    Colon,
}
#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    DoubleEqual,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Increment,
    Decrement,
}
#[derive(Clone, Debug, Copy)]
pub enum Keyword {
    Let,
    Int,
    Float,
    Bool,
    Char,
    String,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut scanner = Scanner::new(input);
    let mut tokens = Vec::new();

    loop {
        let token = scanner.scan();
        if let Token::EOF = token {
            break;
        }
        tokens.push(token);
    }

    tokens
}
