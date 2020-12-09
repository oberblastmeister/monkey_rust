use log::debug;
use log::info;

use super::ast;
use super::{ParseError, ParseResult};
use crate::common::{Accept, Peekable};
use crate::lexer::AdvancedLexer;
use crate::lexer::Token::{self, *};

const LOWEST: u8 = 0;

// #[macro_export]
// macro_rules! expect {
//     ($parser:expr, $token:expr) => {
//         {
//             let parser = $parser;
//             let token = $token;
//             let res = parser.lexer().accept_return(token);
//             match res {
//                 Ok(token) => Ok(token),
//                 Err(op_token) => {
//                     if let Some(token) = op_token {
//                         ParseError::Expected$token { got: String::from(token) }
//                     } else {
//                         ParseError::UnexpectedEof
//                     }
//                 }
//             }
//         }
//     }
// }

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
        if *self.lexer().peek().ok_or(err.clone())? == expected {
            Err(err)
        } else {
            Ok(())
        }
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
            Let => {
                info!("Parsing let statement");
                self.parse_let_statement()?
            }
            Return => {
                info!("Parsing return statement");
                self.parse_return_statement()?
            }
            _ => {
                info!("Parsing expression statement");
                self.parse_expression_statement()?
            }
        };
        self.lexer
            .accept_or(Token::Semicolon, ParseError::NoSemicolon)?;
        Ok(stmt)
    }

    fn parse_let_statement(&mut self) -> ParseResult<ast::Statement<'input>> {
        // curr token let
        let ident = self.next_or_err()?;
        let value = self.parse_expression(LOWEST)?;
        Ok(ast::Statement::Let { ident, value })
    }

    fn parse_return_statement(&mut self) -> ParseResult<ast::Statement<'input>> {
        // curr token return
        let expr = self.parse_expression(LOWEST)?;
        self.lexer().accept_return(Semicolon).map_err(|op_token| ParseError::ExpectedSemicolon { got: String::from(op_token.as_str()) })?;
        self.lexer().accept_or(Semicolon, ParseError::NoSemicolon);
        Ok(ast::Statement::Return(expr))
    }

    fn parse_expression_statement(&mut self) -> ParseResult<ast::Statement<'input>> {
        let expression = self.parse_expression(LOWEST)?;
        self.lexer().accept_or(Semicolon, ParseError::NoSemicolon)?;
        Ok(ast::Statement::Expression(expression))
    }

    fn parse_litnum(&mut self) -> ParseResult<ast::Expression<'input>> {
        let token = self.curr_token_or_err()?;
        let token = token.as_str();
        let num = token.parse().map_err(|e| ParseError::IntLit {
            int: token.to_string(),
            source: e,
        })?;
        Ok(ast::Expression::NumberLiteral(num))
    }

    fn parse_litbool(&mut self) -> ParseResult<ast::Expression<'input>> {
        let token = self.curr_token_or_err()?;
        let token = token.as_str();
        let bool = token.parse::<bool>().map_err(|e| ParseError::BoolLit {
            bool: token.to_string(),
            source: e,
        })?;
        Ok(ast::Expression::BooleanLiteral(bool))
    }

    fn peek_semicolon(&mut self) -> bool {
        match self.lexer.peek() {
            Some(op) if *op == Semicolon => true,
            _ => false
        }
    }
    fn parse_expression(&mut self, min_bp: u8) -> ParseResult<ast::Expression<'input>> {
        let token = self.curr_token_or_err()?;
        debug!("Token: {:?}", token);
        let mut lhs = self.parse_lhs()?;

        loop {
            let op = match self.lexer.peek() {
                Some(op) if *op == Semicolon => break,
                Some(op) => op,
                None => return Ok(lhs),
            };

            if let Some((l_bp, r_bp)) = infix_binding_power(*op) {
                if l_bp < min_bp {
                    info!("l_bp: {} was less than min_bp: {}", l_bp, min_bp);
                    break;
                }
                info!("l_bp: {} was not less than min_bp: {}", l_bp, min_bp);
                info!("Parsing infix expression");
                let token = self.lexer.next().unwrap();
                let rhs = self.parse_expression(r_bp)?;
                lhs = ast::Expression::Infix(Box::new(ast::InfixExpression {
                    lhs,
                    operator: token,
                    rhs,
                }));
                continue;
            };
            break;
        }

        Ok(lhs)
    }

    fn parse_lhs(&mut self) -> ParseResult<ast::Expression<'input>> {
        let curr_token = self.curr_token_or_err()?;
        let lhs = match curr_token {
            Token::Number(_) => {
                info!("Parsing litnum");
                ast::Expression::LitNum(self.parse_litnum()?)
            }
            Token::True | Token::False => {
                info!("Parsing litbool");
                ast::Expression::LitBool(self.parse_litbool()?)
            }
            _ => ast::Expression::Prefix(Box::new(self.parse_prefix_expression()?)),
        };
        Ok(lhs)
    }

    fn parse_prefix_expression(&mut self) -> ParseResult<ast::PrefixExpression<'input>> {
        let curr_token = self.curr_token_or_err()?;
        info!("Parsing prefix expression");
        let ((), r_bp) = prefix_binding_power(curr_token)?;
        let rhs = self.parse_expression(r_bp)?;
        let prefix_expr = ast::PrefixExpression {
            prefix: curr_token,
            rhs,
        };
        Ok(prefix_expr)
    }

    fn parse_infix_expression(&mut self) -> ParseResult<ast::InfixExpression<'input>> {
        todo!()
    }
}

pub fn parse_program(input: &str) -> ParseResult<ast::Program<'_>> {
    let mut parser = Parser::new(input);
    Ok(parser.parse_program()?)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let _ = env_logger::builder().is_test(true).try_init();
        let program = parse_program("1 + 2 * 3;").unwrap();
        assert_eq!(program.to_string(), "(1 + (2 * 3));");
    }
}
