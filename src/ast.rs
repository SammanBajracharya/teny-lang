use crate::token::{DataTypes, Operator};

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        name: String,
        var_type: DataTypes,
        value: Expr,
    },
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    Literal(DataTypes),
    Identifier(String),
    Grouping(Box<Expr>),
}


