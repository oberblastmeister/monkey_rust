use std::fmt;

use crate::parser::{Parse, ParseError, ParseResult, Parser};
use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    // Infix {
    //     lhs: Box<Expression<'a>>,
    //     operator: Token<'a>,
    //     rhs: Box<Expression<'a>>,
    // },
    // Prefix {
    //     prefix: Token<'a>,
    //     rhs: Box<Expression<'a>>,
    // },
    NumberLiteral(i64),
    BooleanLiteral(bool),
    Phantom(std::marker::PhantomData<&'a str>),
}

impl<'a> fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Expression::Infix { lhs, operator, rhs } => write!(f, "({} {} {})", lhs, operator, rhs),
            // Expression::Prefix { prefix, rhs } => write!(f, "({}{})", prefix, rhs),
            Expression::NumberLiteral(x) => write!(f, "{}", x),
            Expression::BooleanLiteral(x) => write!(f, "{}", x),
            _ => todo!(),
        }
    }
}

impl Parse for Expression<'_> {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        let next = p.next_or_err()?;
        println!("Next: {:?}", next);

        Ok(match next {
            Token::Number(n) => Expression::NumberLiteral(n.parse::<i64>().map_err(|e| ParseError::BadNumber)?),
            Token::True => Expression::BooleanLiteral(true),
            Token::False => Expression::BooleanLiteral(false),
            _ => panic!(),
        })
    }
}
