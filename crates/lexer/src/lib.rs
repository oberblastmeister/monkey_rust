#[allow(dead_code)]
#[allow(unused_variables)]
mod tokens;
mod advanced_chars;

use std::str;

use log::debug;
use log::info;
use log::trace;
use tokens::Token::{self, *};

use advanced_chars::AdvancedChars;

/// macro to make return state functions easier
pub struct Lexer<'input> {
    input: &'input str,
    chars: AdvancedChars<'input>,
    start: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &str) -> Lexer<'_> {
        let chars = AdvancedChars::new(input);
        Lexer {
            input,
            chars,
            start: 0,
        }
    }

    fn ignore(&mut self) {
        self.start = self.chars.current_pos_or_end();
    }

    fn accept(&mut self, valid: char) -> bool {
        match self.chars.peek() {
            Some(n) if n == valid => {
                self.chars.next();
                true
            },
            _ => false,
        }
    }

    fn accept_multiple(&mut self, valid: &str) -> bool {
        match self.chars.peek() {
            Some(n) if valid.contains(n) => {
                self.chars.next();
                true
            },
            _ => false,
            
        }
    }

    fn accept_run(&mut self, valid: &str) {
        while let Some(n) = self.chars.peek() {
            if !valid.contains(n) {
                break;
            } else {
                self.chars.next();
            }
        }
    }

    fn accept_while(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(n) = self.chars.peek() {
            debug!("accept while char: {:?}", n);
            if !predicate(n) {
                break;
            } else {
                self.chars.next();
            }
        }
    }

    fn lex_main(&mut self) -> Option<Token<'input>> {
        loop {
            let c = self.chars.next()?;
            break match c {
                '=' => self.assign_or_eq(),
                ';' => token(Semicolon),
                '(' => token(Lparen),
                ')' => token(Rparen),
                ',' => token(Comma),
                '+' => token(Plus),
                '*' => token(Asterisk),
                '/' => self.slash_or_comment(),
                '-' => token(Minus),
                '{' => token(Lbrace),
                '}' => token(Rbrace),
                '>' => self.gt(),
                '<' => self.lt(),
                '!' => self.bang_or_not_eq(),
                _ if is_start_of_number(c) => self.number(),
                _ if is_letter(c) => self.keyword(),
                _ if is_whitespace(c) => self.whitespace(),
                _ => continue,
                // _ => Illegal,
            }
        }
    }

    fn whitespace(&mut self) -> Option<Token<'input>> {
        self.ignore();
        self.lex_main()
    }

    fn slash_or_comment(&mut self) -> Option<Token<'input>> {
        if self.accept('/') {
            self.comment()
        } else {
            token(Slash)
        }
    }

    fn comment(&mut self) -> Option<Token<'input>> {
        while let Some(c) = self.chars.next() {
            if c == '\n' {
                break;
            }
        }
        self.lex_main()
    }

    fn assign_or_eq(&mut self) -> Option<Token<'input>> {
        if self.accept('=') {
            token(Eq)
        } else {
            token(Assign)
        }
    }

    fn gt(&mut self) -> Option<Token<'input>> {
        if self.accept('=') {
            token(GtEq)
        } else {
            token(Gt)
        }
    }

    fn bang_or_not_eq(&mut self) -> Option<Token<'input>> {
        if self.accept('=') {
            token(NotEq)
        } else {
            token(Bang)
        }
    }

    fn lt(&mut self) -> Option<Token<'input>> {
        if self.accept('=') {
            token(LtEq)
        } else {
            token(Lt)
        }
    }

    fn number(&mut self) -> Option<Token<'input>> {
        self.accept_while(is_digit);
        if self.accept('.') {
            self.accept_while(is_digit);
        }

        token(Number(self.current_slice()))
    }

    fn keyword(&mut self) -> Option<Token<'input>> {
        self.accept_while(is_letter);
        match self.current_slice() {
            "fn" => token(Function),
            "return" => token(Return),
            "let" => token(Let),
            "if" => token(If),
            "else" => token(Else),
            "true" => token(True),
            "false" => token(False),
            _ => self.ident(),
        }
    }

    fn ident(&mut self) -> Option<Token<'input>> {
        token(Ident(self.current_slice()))
    }

    fn current_slice(&mut self) -> &'input str {
        &self.input[self.start..self.chars.current_pos_or_end()]
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Token<'input>> {
        self.lex_main()
    }
} 

fn token(token: Token) -> Option<Token> {
    Some(token)
}

const fn is_linebreak(c: char) -> bool {
    c == '\n'
}

const fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

const fn is_start_of_number(c: char) -> bool {
    is_digit(c) || c == '.'
}

const fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

const fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lexer(input: &'static str, expected_tokens: &[Token]) {
        let _ = env_logger::builder().is_test(true).try_init();
        let res: Vec<_> = Lexer::new(input).collect();
        assert_eq!(res, expected_tokens);
    }

    #[test]
    fn lex1_test() {
        let input = "let five = 5;";
        let expected_tokens = &[Let, Ident("five"), Assign, Number("5"), Semicolon];
        test_lexer(input, expected_tokens);
    }

    #[test]
    fn lex_function_test() {
        let input = "let add = fn(x, y) {
    x + y;
}";
        let expected_tokens = &[
            Let,
            Ident("add"),
            Assign,
            Function,
            Lparen,
            Ident("x"),
            Comma,
            Ident("y"),
            Rparen,
            Lbrace,
            Ident("x"),
            Plus,
            Ident("y"),
            Semicolon,
            Rbrace,
        ];
        test_lexer(input, expected_tokens);
    }

    #[test]
    fn assign_or_eq_test() {
        let input = "let add = 20;
20 == 20;";

        let expected_tokens = &[
            Let,
            Ident("add"),
            Assign,
            Number("20"),
            Semicolon,
            Number("20"),
            Eq,
            Number("20"),
            Semicolon,
        ];

        test_lexer(input, expected_tokens);
    }

    #[test]
    fn comment_and_other_test() {
        let input = "// this is a comment
20 / 2;";

        let expected_tokens = &[Number("20"), Slash, Number("2"), Semicolon];

        test_lexer(input, expected_tokens);
    }

    #[test]
    fn comment_only_test() {
        let input = "// this is another comment";
        let expected_tokens = &[];
        test_lexer(input, expected_tokens);
    }

    #[test]
    fn operators_test() {
        let input = "!-/*5;";
        let expected_tokens = &[Bang, Minus, Slash, Asterisk, Number("5"), Semicolon];
        test_lexer(input, expected_tokens);
    }

    #[test]
    fn if_statement_test() {
        let input = "if (5 < 10) {
return true;
} else {
    return false;
}
";
        let expected_tokens = &[
            If,
            Lparen,
            Number("5"),
            Lt,
            Number("10"),
            Rparen,
            Lbrace,
            Return,
            True,
            Semicolon,
            Rbrace,
            Else,
            Lbrace,
            Return,
            False,
            Semicolon,
            Rbrace,
        ];
        test_lexer(input, expected_tokens);
    }

    #[test]
    fn single_line_test() {
        let input = "let number = 50;";
        let expected_tokens = &[
            Let,
            Ident("number"),
            Assign,
            Number("50"),
            Semicolon,
        ];
        test_lexer(input, expected_tokens);
    }
    
    #[test]
    fn none_test() {
        let input = "";
        let expected_tokens = &[];
        test_lexer(input, expected_tokens);
    }

    #[test]
    fn only_newlines_test() {
        let input = "\n\n\n";
        let expected_tokens = &[];
        test_lexer(input, expected_tokens);
    }

    #[test]
    fn unicode_test() {
        let input = "let Здравствуйте = 100;";
        let expected_tokens = &[
            Let,
            Ident("Здравствуйте"),
            Assign,
            Number("100"),
            Semicolon
        ];
        test_lexer(input, expected_tokens);
    }
}
