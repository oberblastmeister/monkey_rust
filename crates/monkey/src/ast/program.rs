use std::fmt;

use crate::{
    common::Peekable,
    parser::{Parse, ParseResult, Parser},
};

use super::stmt::Statement;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Program<'a> {
    statements: Vec<Statement<'a>>,
}

impl<'a> Program<'a> {
    pub fn push(&mut self, statement: Statement<'a>) {
        self.statements.push(statement)
    }
}

impl<'a> fmt::Display for Program<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.statements
                .iter()
                .map(|stmt| stmt.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Parse for Program<'_> {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        let mut program = Program::default();

        loop {
            if p.lexer().peek().is_none() {
                return Ok(program);
            }
            program.push(p.parse()?);
        }
    }
}
