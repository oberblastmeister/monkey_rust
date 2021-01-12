use std::fmt;

use crate::lexer::Token;
use crate::parser::{Parse, Parser, ParseResult};

use super::Expression;

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

impl Parse for Statement<'_> {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        let res = match p.next_or_err()? {
            Token::Let => {
                panic!()
            }
            Token::Return => {
                panic!()
            }
            _ => Statement::Expression(p.parse()?),
        };
        Ok(res)
    }
}
