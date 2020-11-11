use std::num::ParseIntError;

use crate::lexer::Token;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("{0}")]
    Custom(&'static str),

    #[error("Failed to parse token {int} into an integer: {source}")]
    ParseInt {
        int: String,
        #[source]
        source: ParseIntError,
    }
}
