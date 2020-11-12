use super::{Parse, ParseError, Parser};
use crate::common::{Accept, Peekable};
use crate::lexer::Token::{self, *};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Program<'a> {
    statements: Vec<Statement<'a>>,
}

impl<'a> Program<'a> {
    pub fn push(&mut self, stmt: Statement<'a>) {
        self.statements.push(stmt);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    Let(LetStmt<'a>),
    Return(ReturnStmt<'a>),
    Expression(ExpressionStmt<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Infix(Box<InfixExpression<'a>>),
    Prefix(Box<PrefixExpression<'a>>),
    LitNum(LitNum),
    LitBool(LitBool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LitNum(pub u64);

#[derive(Debug, Clone, PartialEq)]
pub struct LitBool(pub bool);

#[derive(Debug, Clone, PartialEq)]
pub struct PrefixExpression<'a> {
    prefix: Token<'a>,
    rhs: Expression<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InfixExpression<'a> {
    lhs: Expression<'a>,
    operator: Token<'a>,
    rhs: Expression<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStmt<'a>(pub Expression<'a>);

#[derive(Debug, Clone, PartialEq)]
pub struct LetStmt<'a> {
    name: Token<'a>,
    value: Expression<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt<'a> {
    token: Token<'a>,
    value: Expression<'a>,
}
