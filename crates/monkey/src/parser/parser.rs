use super::ast;
use super::Parse;
use super::ParseError;
use crate::common::{Peekable, Accept};
use crate::lexer::AdvancedLexer;
use crate::lexer::Token::{self, *};

pub struct Parser<'a> {
    lexer: AdvancedLexer<'a>,
}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Parser<'input> {
        let lexer = AdvancedLexer::new(input);
        Parser { lexer }
    }

    pub fn lexer(&mut self) -> &mut AdvancedLexer<'input> {
        &mut self.lexer
    }

    pub fn parse<T>(&mut self) -> Result<T, ParseError>
    where
        T: Parse,
    {
        T::parse(self)
    }
}
