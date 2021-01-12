use crate::lexer::Token;
use thiserror::Error;

pub type ParseResult<T, E = ParseError> = Result<T, E>;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParseError {
    #[error("{0}")]
    Custom(&'static str),

    #[error("Bad number")]
    BadNumber,

    #[error("Unexpected end of file")]
    UnexpectedEof,

    #[error("No semicolon after statement was found")]
    NoSemicolon,

    #[error("Bad prefix operator `{op}`")]
    BadPrefixOperator {
        op: String,
    },

    #[error("Bad postfix operator `{op}`")]
    BadPostfixOperator {
        op: String
    },

    #[error("Expected semicolon, got `{got}`")]
    ExpectedSemicolon {
        got: String,
    },

    #[error("Expected token `{token}`, got token `{got}`")]
    Expected {
        token: &'static str,
        got: &'static str,
    }
}
