use std::num::ParseIntError;
use std::str::ParseBoolError;

use crate::lexer::Token;
use thiserror::Error;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParseError {
    #[error("{0}")]
    Custom(&'static str),

    #[error("Failed to parse token {int} into an integer: {source}")]
    IntLit {
        int: String,
        #[source]
        source: ParseIntError,
    },

    #[error("Failed to parse token {bool} into a bool: {source}")]
    BoolLit {
        bool: String,
        #[source]
        source: ParseBoolError,
    },

    #[error("Unexpected Eof")]
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
    }
}
