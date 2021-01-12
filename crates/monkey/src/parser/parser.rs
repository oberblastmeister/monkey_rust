use super::{Parse, ParseError, ParseResult};
use crate::ast;
use crate::common::{Accept, Peekable};
use crate::lexer::AdvancedLexer;
use crate::lexer::Token::{self, *};

pub struct Parser<'input> {
    pub lexer: AdvancedLexer<'input>,
    pub errors: Vec<ParseError>,
}

pub fn parse(s: &str) -> ParseResult<ast::Program<'_>> {
    Parser::new(s).parse()
}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Parser<'input> {
        let lexer = AdvancedLexer::new(input);
        Parser {
            lexer,
            errors: Vec::new(),
        }
    }

    pub fn lexer(&mut self) -> &mut AdvancedLexer<'input> {
        &mut self.lexer
    }

    pub fn expect_or(&mut self, expected: Token<'_>, err: ParseError) -> ParseResult<()> {
        if *self.lexer().peek().ok_or(err.clone())? == expected {
            Err(err)
        } else {
            Ok(())
        }
    }

    pub fn next_or_err(&mut self) -> Result<Token<'_>, ParseError> {
        self.lexer.next().ok_or(ParseError::UnexpectedEof)
    }

    pub fn peek_or_err(&mut self) -> Result<&Token<'_>, ParseError> {
        self.lexer.peek().ok_or(ParseError::UnexpectedEof)
    }

    pub fn curr_token_or_err(&self) -> Result<Token<'input>, ParseError> {
        self.lexer.curr_token().ok_or(ParseError::UnexpectedEof)
    }

    pub fn parse<T: Parse>(&mut self) -> ParseResult<T> {
        T::parse(self)
    }

    pub fn expect<'a>(&mut self, token: Token<'a>) -> ParseResult<Token<'a>> {
        let next = self.next_or_err()?;
        if next == token {
            Ok(token)
        } else {
            Err(ParseError::Expected {
                token: token.as_static_str(),
                got: next.as_static_str(),
            })
        }
    }
}

// impl<'a> Peekable for Parser<'a> {
//     fn peek(&self) -> Option<&Self::Item> {
//         self.lexer.peek()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        let s = "1234;";
        let program = parse(s).unwrap();
        assert_eq!(program.to_string(), "1234;")
    }

    #[test]
    fn parse_bool() {
        let s = "true;";
        let program = parse(s).unwrap();
        assert_eq!(program.to_string(), "true;")
    }

    #[should_panic]
    #[test]
    fn parse_statement_no_end() {
        let s = "1234123";
        let program = parse(s).unwrap();
    }
}
