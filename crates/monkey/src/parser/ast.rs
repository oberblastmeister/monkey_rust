use std::fmt;

use super::{ParseError, Parser};
use crate::common::{Accept, Peekable};
use crate::lexer::Token::{self, *};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Program<'a> {
    statements: Vec<Statement<'a>>,
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
        name: Token<'a>,
        value: Expression<'a>,
    },
    Return(Option<Expression<'a>>),
    Expression(ExpressionStmt<'a>),
}

impl<'a> fmt::Display for Statement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(x) => write!(f, "{}", x),
            Statement::Return(x) => write!(f, "{}", x),
            Statement::Expression(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Infix(Box<InfixExpression<'a>>),
    Prefix(Box<PrefixExpression<'a>>),
    LitNum(LitNum),
    LitBool(LitBool),
}

impl<'a> fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Infix(x) => write!(f, "{}", x),
            Expression::Prefix(x) => write!(f, "{}", x),
            Expression::LitNum (x)=> write!(f, "{}", x),
            Expression::LitBool(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LitNum(pub u64);

impl fmt::Display for LitNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LitBool(pub bool);

impl fmt::Display for LitBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrefixExpression<'a> {
    pub prefix: Token<'a>,
    pub rhs: Expression<'a>,
}

impl<'a> fmt::Display for PrefixExpression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.prefix, self.rhs)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InfixExpression<'a> {
    pub lhs: Expression<'a>,
    pub operator: Token<'a>,
    pub rhs: Expression<'a>,
}

impl<'a> fmt::Display for InfixExpression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.lhs, self.operator, self.rhs)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStmt<'a>(pub Expression<'a>);

impl<'a> fmt::Display for ExpressionStmt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStmt<'a> {
    pub name: Token<'a>,
    pub value: Expression<'a>,
}

impl<'a> fmt::Display for LetStmt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "let {} = {}", self.name, self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt<'a> {
    pub value: Expression<'a>,
}

impl<'a> fmt::Display for ReturnStmt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return {}", self.value)
    }
}
