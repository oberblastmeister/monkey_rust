use std::fmt;

use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Program<'a> {
    statements: Vec<Statement<'a>>,
}

impl<'a> Program<'a> {
    pub fn push(&mut self, statement: Statement<'a>) {
        self.statements.push(statement)
    }
}

impl<'a> fmt::Display for Program<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for statement in &self.statements {
            s.push_str(&statement.to_string());
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    Let {
        ident: Token<'a>,
        value: Expression<'a>,
    },
    Return(Expression<'a>),
    Expression(Expression<'a>),
}

impl<'a> fmt::Display for Statement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let { ident, value } => write!(f, "let {} = value {};", ident, value),
            Statement::Return(x) => write!(f, "return {};", x),
            Statement::Expression(x) => write!(f, "{};", x),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Infix {
        lhs: Box<Expression<'a>>,
        operator: Token<'a>,
        rhs: Box<Expression<'a>>,
    },
    Prefix {
        prefix: Token<'a>,
        rhs: Box<Expression<'a>>,
    },
    NumberLiteral(i64),
    BooleanLiteral(bool),
}

impl<'a> fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Infix { lhs, operator, rhs } => write!(f, "({} {} {})", lhs, operator, rhs),
            Expression::Prefix { prefix, rhs } => write!(f, "({}{})", prefix, rhs),
            Expression::NumberLiteral(x) => write!(f, "{}", x),
            Expression::BooleanLiteral(x) => write!(f, "{}", x),
        }
    }
}
