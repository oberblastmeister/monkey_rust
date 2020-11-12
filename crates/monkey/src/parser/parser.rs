use super::ast;
use super::Parse;
use super::{ParseError, ParseResult};
use crate::common::{Accept, Peekable};
use crate::lexer::AdvancedLexer;
use crate::lexer::Token::{self, *};

const LOWEST: u8 = 0;

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

    pub fn expect_or(&mut self, expected: Token<'_>, err: ParseError) -> ParseResult<()> {
        if *self.lexer().peek().ok_or(err)? == expected {
            Err(err)
        } else {
            Ok(())
        }
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

    fn parse_program(&mut self) -> ParseResult<ast::Program<'input>> {
        let mut program = ast::Program::default();

        loop {
            if self.lexer().next().is_none() {
                return Ok(program);
            }
            let stmt = self.parse_statement()?;
            program.push(stmt);
        }
    }

    fn parse_statement(&mut self) -> ParseResult<ast::Statement<'input>> {
        let stmt = match self.curr_token_or_err()? {
            Let => ast::Statement::Let(self.parse_let_statement()?),
            Return => ast::Statement::Return(self.parse_return_statement()?),
            _ => ast::Statement::Expression(self.parse_expression_statement()?),
        };
        self.lexer
            .accept_or(Token::Semicolon, ParseError::NoSemicolon)?;
        Ok(stmt)
    }

    fn parse_let_statement(&mut self) -> ParseResult<ast::LetStmt<'input>> {
        let name = self.next_or_err()?;
        let value = self.parse_expression(LOWEST)?;
        Ok(ast::LetStmt { name, value })
    }

    fn parse_return_statement(&mut self) -> ParseResult<ast::ReturnStmt<'input>> {
        let value = self.parse_expression(LOWEST)?;
        Ok(ast::ReturnStmt {
            token: Token::Return,
            value,
        })
    }

    fn parse_expression_statement(&mut self) -> ParseResult<ast::ExpressionStmt<'input>> {
        let expression = self.parse_expression(LOWEST)?;
        Ok(ast::ExpressionStmt(expression))
    }

    fn parse_litnum(&mut self) -> ParseResult<ast::LitNum> {
        let token = self.lexer().next().ok_or(ParseError::UnexpectedEof)?;
        let token = token.as_str();
        let num = token.parse::<u64>().map_err(|e| ParseError::IntLit {
            int: token.to_string(),
            source: e,
        })?;
        Ok((ast::LitNum(num)))
    }

    fn parse_litbool(&mut self) -> ParseResult<ast::LitBool> {
        let token = self.lexer().next().ok_or(ParseError::UnexpectedEof)?;
        let token = token.as_str();
        let bool = token.parse::<bool>().map_err(|e| ParseError::BoolLit {
            bool: token.to_string(),
            source: e,
        })?;
        Ok(ast::LitBool(bool))
    }

    fn parse_expression(&mut self, min_bp: u8) -> ParseResult<ast::Expression<'input>> {
        let token = self.curr_token_or_err()?;
        let lhs = match token {
            Token::Number(_) => ast::Expression::LitNum(self.parse_litnum()?),
            Token::True | Token::False => ast::Expression::LitBool(self.parse_litbool()?),
            _ => {
                let ((), r_bp) = prefix_binding_power(token)?;
                let rhs = self.parse_expression(r_bp)?;
                ast::Expression::Prefix(Box::new(ast::PrefixExpression { prefix: token, rhs }))
            }
        };

        loop {
            let op = match self.lexer.peek() {
                Some(op) => op,
                None => return Ok(lhs),
            };

            if let Some((l_bp, r_bp)) = infix_binding_power(*op) {
                if l_bp < min_bp {
                    break;
                }
                let token = self.lexer.next().unwrap();
                let rhs = self.parse_expression(r_bp)?;
                lhs = ast::Expression::Infix(Box::new(ast::InfixExpression { lhs, operator: token, rhs }));
                continue;
            };
            break;
        }

        Ok(lhs)
    }

    fn parse_prefix_expression(&mut self) -> ParseResult<ast::PrefixExpression<'input>> {
        todo!()
    }

    fn parse_infix_expression(&mut self) -> ParseResult<ast::InfixExpression<'input>> {
        todo!()
    }
}

fn prefix_binding_power(op: Token<'_>) -> ParseResult<((), u8)> {
    match op {
        Plus | Minus | Bang => Ok(((), 9)),
        _ => Err(ParseError::BadPrefixOperator { op: op.to_string() }),
    }
}

fn infix_binding_power(op: Token<'_>) -> Option<(u8, u8)> {
    let op = match op {
        Assign => (2, 1),
        Plus | Minus => (5, 6),
        Asterisk | Slash => (7, 8),
        _ => return None,
    };
    Some(op)
}
