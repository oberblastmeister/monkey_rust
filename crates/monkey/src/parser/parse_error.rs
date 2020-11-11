use std::num::ParseIntError;
use std::str::ParseBoolError;

use crate::lexer::Token;
use thiserror::Error;

#[derive(Error, Debug)]
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
}
