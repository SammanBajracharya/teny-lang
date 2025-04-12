use std::{char, collections::HashMap};

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
#[derive(Clone, Debug, Copy)]
pub enum Symbol {
    Semicolon,
    LParen,
    RParen,
    Colon,
}
#[derive(Clone, Debug, Copy)]
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

pub struct Scanner<'a> {
    input: &'a str,
    pos: usize,
    keywords: HashMap<&'static str, Keyword>,
    operators: HashMap<&'static str, Operator>,
    symbols: HashMap<&'static str, Symbol>,
    data_types: HashMap<&'static str, DataTypes>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("let", Keyword::Let);
        keywords.insert("int", Keyword::Int);
        keywords.insert("float", Keyword::Float);
        keywords.insert("bool", Keyword::Bool);
        keywords.insert("char", Keyword::Char);
        keywords.insert("string", Keyword::String);

        let mut operators = HashMap::new();
        operators.insert("+", Operator::Plus);
        operators.insert("-", Operator::Minus);
        operators.insert("*", Operator::Multiply);
        operators.insert("/", Operator::Divide);
        operators.insert("=", Operator::Equal);
        operators.insert("==", Operator::DoubleEqual);
        operators.insert("!=", Operator::NotEqual);
        operators.insert(">", Operator::Greater);
        operators.insert("<", Operator::Less);
        operators.insert(">=", Operator::GreaterEqual);
        operators.insert("<=", Operator::LessEqual);
        operators.insert("++", Operator::Increment);
        operators.insert("--", Operator::Decrement);

        let mut symbols = HashMap::new();
        symbols.insert(";", Symbol::Semicolon);
        symbols.insert(":", Symbol::Colon);
        symbols.insert("(", Symbol::LParen);
        symbols.insert(")", Symbol::RParen);

        let mut data_types = HashMap::new();
        data_types.insert("int", DataTypes::Integer(0));
        data_types.insert("float", DataTypes::Float(0.0));
        data_types.insert("bool", DataTypes::Boolean(false));
        data_types.insert("char", DataTypes::Char('\0'));
        data_types.insert("string", DataTypes::String(String::new()));

        Self { input, pos: 0, keywords, operators, symbols, data_types }
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        self.pos += 1;
        ch
    }

    pub fn scan(&mut self) -> Token {
        // Skip whitespaces
        while let Some(c) = self.peek() {
            if c.is_whitespace() { self.advance(); }
            if c == '\n' { self.advance(); }
            else { break; }
        }

        // Check for End of File
        if self.pos >= self.input.len() { return Token::EOF; }

        let ch = self.advance().unwrap();

        // Identifiers or Keywords
        if ch.is_alphabetic() || ch == '_' {
            let mut ident = String::new();
            ident.push(ch);
            while let Some(c) = self.peek() {
                if c.is_alphanumeric() || c == '_' { ident.push(self.advance().unwrap()); }
                else { break; }
            }

            if let Some(kw) = self.keywords.get(ident.as_str()) {
                return Token::Keyword(kw.clone());
            }

            return Token::Identifiers(ident)
        }

        // Integer or float
        if ch.is_numeric() {
            let mut num_str = ch.to_string();
            let mut is_float = false;
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() { num_str.push(self.advance().unwrap()); }
                else if c == '.' && !is_float {
                    is_float = true;
                    num_str.push(self.advance().unwrap());
                }
                else { break; }
            }

            if is_float {
                match num_str.parse::<f64>() {
                    Ok(f) => return Token::DataTypes(DataTypes::Float(f)),
                    Err(_) => panic!("Invalid integer literal. Expected float, got {}", num_str),
                }
            } else {
                match num_str.parse::<i64>() {
                    Ok(i) => return Token::DataTypes(DataTypes::Integer(i)),
                    Err(_) => panic!("Invalid float literal. Expected int, got {}", num_str),
                }
            }
        }

        // String
        if ch == '"' {
            let mut str_val = String::new();
            while let Some(c) = self.peek() {
                if c != '"' {
                    str_val.push(self.advance().unwrap());
                } else {
                    self.advance();
                    break;
                }
            }
            return Token::DataTypes(DataTypes::String(str_val));
        }

        // Char
        if ch == '\'' {
            let c = self.advance().unwrap();
            self.advance(); // Skip the closing quote
            if c == '\'' && self.peek() == Some('\'') {
                panic!("Invalid char literal");
            }
            return Token::DataTypes(DataTypes::Char(c));
        }

        // Multi-character Operators
        if let Some(c) = self.peek() {
            let op = format!("{}{}", ch, c);
            if self.operators.contains_key(op.as_str()) {
                self.advance();
                let operator = self.operators.get(op.as_str()).unwrap();
                return Token::Operator(operator.clone());
            }
        }

        // Single-character Operators
        if let Some(operator) = self.operators.get(&ch.to_string().as_str()) {
            return Token::Operator(operator.clone());
        }

        // Symbols
        if let Some(symbol) = self.symbols.get(&ch.to_string().as_str()) {
            return Token::Symbol(symbol.clone());
        }

        Token::Unknown(ch)
    }
}
