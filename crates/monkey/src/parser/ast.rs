use crate::lexer::Token::{self, *};
use crate::common::Accept;
use super::{Parser, Parse, ParseError};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Program<'a> {
    statements: Vec<Statement<'a>>,
}

impl<'a> Program<'a> {
    pub fn push(&mut self, stmt: Statement<'a>) {
        self.statements.push(stmt);
    }
}

impl<'a> Parse for Program<'a> {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        let mut program = Program::default();

        loop {
            let stmt = Statement::parse(p)?;
            program.push(stmt);
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

#[derive(Debug, Clone, PartialEq)]
pub struct LitNum(u64);

impl Parse for LitNum {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        let token
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LitBool(bool);

impl Parse for LitBool {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        todo!()
    }
}

impl<'a> Parse for Expression<'a> {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrefixExpression<'a> {
    prefix: Token<'a>,
    rhs: Expression<'a>,
}

impl<'a> Parse for PrefixExpression<'a> {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InfixExpression<'a> {
    lhs: Expression<'a>,
    operator: Token<'a>,
    rhs: Expression<'a>,
}

impl<'a> Parse for InfixExpression<'a> {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStmt<'a>(Expression<'a>);

impl<'a> Parse for ExpressionStmt<'a> {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStmt<'a> {
    name: Token<'a>,
    value: Expression<'a>,
}

impl<'a> Parse for LetStmt<'a> {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt<'a> {
    token: Token<'a>,
    return_value: Expression<'a>,
}

impl<'a> Parse for ReturnStmt<'a> {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        p.lexer().accept_or(Return, ParseError::Custom("Failed to find return token"))?;
        let expr = p.parse::<Expression>()?;
        Ok(
            ReturnStmt {
                token: Return,
                return_value: expr,
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    Let(LetStmt<'a>),
    Return(ReturnStmt<'a>),
    Expression(ExpressionStmt<'a>),
}

impl<'a> Parse for Statement<'a> {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        let token = p.lexer().next().ok_or_else(|| ParseError::Custom("no next token"))?;
        let stmt = match token {
            Let => Statement::Let(LetStmt::parse(p)?),
            Return => Statement::Return(ReturnStmt::parse(p)?),
            _ => Statement::Expression(ExpressionStmt::parse(p)?),
        };
        Ok(stmt)
    }
}
