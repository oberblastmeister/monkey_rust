use super::ast;
use super::Parse;
use super::ParseError;
use crate::common::{Peekable, Accept};
use crate::lexer::AdvancedLexer;
use crate::lexer::Token::{self, *};

pub struct Parser<'input> {
    pub lexer: AdvancedLexer<'input>,
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

    pub fn next_or_err(&mut self) -> Result<Token<'input>, ParseError> {
        self.lexer.next().ok_or(ParseError::UnexpectedEof)
    }

    pub fn curr_token_or_err(&self) -> Result<Token<'input>, ParseError> {
        self.lexer.curr_token().ok_or(ParseError::UnexpectedEof)
    } 
}
