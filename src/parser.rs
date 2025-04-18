use crate::token::{Token, DataTypes, Symbol, Operator, Keyword};
use crate::ast::{Statement, Expr};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl<'a> Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens.get(self.current  - 1).unwrap_or(&Token::EOF).clone()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::EOF)
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Vec::new();
        loop {
            // Check if we have reached the end of the tokens
            if self.is_at_end() {
                break;
            }
            statements.push(self.parse_stmt());
        }
        statements
    }

    fn parse_stmt(&mut self) -> Statement {
        match self.peek() {
            Token::Keyword(Keyword::Let) => self.parse_let_stmt(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_expr_stmt(&mut self) -> Statement {
        let expr = self.parse_expr();
        self.consume_symbol(Symbol::Semicolon);
        Statement::Expr(expr)
    }

    fn parse_let_stmt(&mut self) -> Statement {
        self.advance();
        let name = if let Token::Identifiers(name) = self.advance() {
            name
        } else {
            panic!("Expected identifiers after 'let'");
        };
        self.consume_symbol(Symbol::Colon);
        let var_type = match self.advance() {
            Token::Keyword(Keyword::Int) => DataTypes::Integer(0),
            Token::Keyword(Keyword::Float) => DataTypes::Float(0.0),
            Token::Keyword(Keyword::Bool) => DataTypes::Boolean(false),
            Token::Keyword(Keyword::Char) => DataTypes::Char('\0'),
            Token::Keyword(Keyword::String) => DataTypes::String(String::new()),
            _ => panic!("Expected type after ':'"),
        };

        self.consume_operator(Operator::Equal);
        let value = self.parse_expr();
        self.consume_symbol(Symbol::Semicolon);

        Statement::Let { name, var_type, value }
    }

    fn parse_expr(&mut self) -> Expr {
        let mut expr = self.parse_term();
        while let Some(op) = self.match_operator(&[Operator::Plus, Operator::Minus]) {
            let right = self.parse_term();
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        println!("{:?}", expr);
        expr
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();

        while let Some(op) = self.match_operator(&[Operator::Multiply, Operator::Divide]) {
            let right = self.parse_factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            }
        }

        expr
    }

    fn parse_factor(&mut self) -> Expr {
        match self.advance() {
            Token::DataTypes(data) => Expr::Literal(data),
            Token::Identifiers(name) => Expr::Identifier(name),
            Token::Symbol(Symbol::LParen) => {
                let expr = self.parse_expr();
                self.consume_symbol(Symbol::RParen);
                Expr::Grouping(Box::new(expr))
            },
            Token::Unknown(' ') => { self.advance(); self.parse_factor() }
            tok => panic!("Unexpected token in expression: {:?}", tok),
        }
    }

    fn match_operator(&mut self, ops: &[Operator]) -> Option<Operator> {
        if let Token::Operator(op) = self.peek() {
            if ops.contains(op) {
                return Some(self.advance_operator());
            }
        }
        None
    }

    fn advance_operator(&mut self) -> Operator {
        if let Token::Operator(op) = self.advance() { op }
        else { panic!("Expected Operator") }
    }

    fn consume_symbol(&mut self, expected: Symbol) {
        match self.advance() {
            Token::Symbol(sym) if sym == expected => {},
            t => panic!("Expected symbol {:?}, got {:?}", expected, t),
        }
    }

    fn consume_operator(&mut self, expected: Operator) {
        match self.advance() {
            Token::Operator(op) if op == expected => {},
            t => panic!("Expected operator {:?}, got {:?}", expected, t),
        };
    }
}
