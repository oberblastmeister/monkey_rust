#[allow(dead_code)]
#[allow(unused_variables)]
mod tokens;
mod advanced_chars;

use std::str;
use std::time::Duration;
use std::sync::mpsc::{Sender, Receiver, channel, RecvTimeoutError, SendError};

use log::debug;
use log::info;
// use crossbeam_utils::thread;

use common::Accept;
pub use tokens::Token;
use tokens::Token::*;
use advanced_chars::AdvancedChars;

/// lexer struct, holds input str, chars iterator, and start position which is the memorized
/// position in case the token is more than one char long
#[derive(Debug, Clone)]
pub struct Lexer<'input> {
    input: &'input str,
    chars: AdvancedChars<'input>,
    start: usize,
}

impl<'input> Lexer<'input> {
    /// crete a new lexer iterator from a string
    pub fn new(input: &str) -> Lexer<'_> {
        let chars = AdvancedChars::new(input);
        Lexer {
            input,
            chars,
            start: 0,
        }
    }

    /// moves start back to peek position
    fn ignore(&mut self) {
        let new_start = self.chars.peek_pos_or_end(); 
        debug!("ignored: start to {}", new_start);
        self.start = new_start;
    }

    /// the main lexer funciton that determines what the token is and weather the state should be
    /// passed on to a new function
    fn lex_main(&mut self) -> Option<Token<'input>> {
        let c = self.chars.next()?;
        // if the match arm returns a token, that means the token can only be one char long. if
        // there is ambiguity about which token should be returned or weather the token is multiple
        // chars long, a new state function is called that will determine the token
        let res = match c {
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
            _ if is_start_of_number(&c) => self.number(),
            _ if is_letter(&c) => self.keyword(),
            _ if is_whitespace(&c) => self.whitespace(),
            _ => token(Illegal),
        };
        debug!("res: {:?}", res);
        self.ignore();
        res
    }

    fn whitespace(&mut self) -> Option<Token<'input>> {
        self.ignore();
        self.lex_main()
    }

    fn slash_or_comment(&mut self) -> Option<Token<'input>> {
        if self.chars.accept('/') {
            self.comment()
        } else {
            token(Slash)
        }
    }

    fn comment(&mut self) -> Option<Token<'input>> {
        info!("In comment state");
        if let None = self.chars.find(is_linebreak) {
            if self.input.lines().count() != 1 {
                panic!("Could not find char that matched function is_linebreak");
            }
        };
        self.ignore();
        self.lex_main()
    }

    fn assign_or_eq(&mut self) -> Option<Token<'input>> {
        if self.chars.accept('=') {
            token(Eq)
        } else {
            token(Assign)
        }
    }

    fn gt(&mut self) -> Option<Token<'input>> {
        if self.chars.accept('=') {
            token(GtEq)
        } else {
            token(Gt)
        }
    }

    fn bang_or_not_eq(&mut self) -> Option<Token<'input>> {
        if self.chars.accept('=') {
            token(NotEq)
        } else {
            token(Bang)
        }
    }

    fn lt(&mut self) -> Option<Token<'input>> {
        if self.chars.accept('=') {
            token(LtEq)
        } else {
            token(Lt)
        }
    }

    fn number(&mut self) -> Option<Token<'input>> {
        info!("in number state");
        self.chars.accept_while(is_digit);
        if self.chars.accept('.') {
            self.chars.accept_while(is_digit);
        }

        token(Number(self.current_slice()))
    }

    fn keyword(&mut self) -> Option<Token<'input>> {
        info!("in keyword state");
        self.chars.accept_while(is_letter);
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
        info!("in ident state");
        token(Ident(self.current_slice()))
    }

    fn current_slice(&mut self) -> &'input str {
        let peek_pos = self.chars.peek_pos_or_end();
        let start = self.start;
        let current_slice = &self.input[start..peek_pos];
        debug!("slice start pos: `{}`", start);
        debug!("slice end pos: `{}`", peek_pos);
        debug!("current slice `{}`", current_slice);
        current_slice
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Token<'input>> {
        let res = self.lex_main();
        debug!("next token: {:?}", res);
        res
    }
} 

fn token(token: Token) -> Option<Token> {
    Some(token)
}

// const fn is_linebreak(c: char) -> bool {
//     c == '\n'
// }
const fn is_linebreak(c: &char) -> bool {
    *c == '\n'
}

const fn is_whitespace(c: &char) -> bool {
    c.is_ascii_whitespace()
}

const fn is_start_of_number(c: &char) -> bool {
    is_digit(c) || *c == '.'
}

/// checks if the char is an ascii digit
const fn is_digit(c: &char) -> bool {
    c.is_ascii_digit()
}

/// checks if the char is a unicode letter
fn is_letter(c: &char) -> bool {
    c.is_alphabetic() || *c == '_'
}

pub struct LexerSender<'input> {
    sender: Sender<Option<Token<'input>>>,
    lexer: Lexer<'input>,
}

impl<'input> LexerSender<'input> {
    fn new(input: &'input str, sender: Sender<Option<Token<'input>>>) -> LexerSender<'input> {
        let lexer = Lexer::new(input);
        LexerSender {
            sender,
            lexer,
        }
    }

    /// Sends all the tokens and block
    fn send(self) {
        for token in self.lexer {
            self.sender.send(Some(token)).expect("Failed to send token");
        }
        self.sender.send(None).expect("Failed to send no tokens");
    }
}

pub struct LexerReceiver<'input> {
    receiver: Receiver<Option<Token<'input>>>,
    no_more: bool,
}

impl<'input> LexerReceiver<'input> {
    fn new(receiver: Receiver<Option<Token<'input>>>) -> LexerReceiver<'input> {
        LexerReceiver {
            receiver,
            no_more: false,
        }
    }
}

impl<'input> Iterator for LexerReceiver<'input> {
    type Item = Result<Token<'input>, RecvTimeoutError>;

    fn next(&mut self) -> Option<Result<Token<'input>, RecvTimeoutError>> {
        if self.no_more {
            return None;
        }

        match self.receiver.recv_timeout(Duration::from_secs(60)).transpose() {
            Some(token) => Some(token),
            None => {
                self.no_more = true;
                None
            }
        }
    }
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
    fn lexer_init() {
        let mut lexer = Lexer::new("hello person");
        assert_eq!(lexer.current_slice(), "");
    }

    #[test]
    fn accept() {
        let _ = env_logger::builder().is_test(true).try_init();
        let mut lexer = Lexer::new("hi");
        assert!(lexer.chars.accept('h'));
        assert_eq!(lexer.current_slice(), "h");
    }

    #[test]
    fn accept_fail() {
        let mut lexer = Lexer::new("wow");
        assert!(!lexer.chars.accept('h'));
        assert_eq!(lexer.current_slice(), "");
    }

    #[test]
    fn accept_while_none() {
        let mut lexer = Lexer::new("this");
        lexer.chars.accept_while(|c| *c == 'x');
        assert_eq!(lexer.current_slice(), "");
    }

    #[test]
    fn accept_while_everything() {
        let mut lexer = Lexer::new("this should be the same");
        lexer.chars.accept_while(|_| true);
        assert_eq!(lexer.current_slice(), "this should be the same");
    }

    #[test]
    fn accept_while_in_the_middle_test() {
        let mut lexer = Lexer::new("middle7098");
        lexer.chars.accept_while(is_letter);
        assert_eq!(lexer.current_slice(), "middle");
    }

    #[test]
    fn find() {
        let mut lexer = Lexer::new("first line\nnext line");
        assert!(lexer.chars.find(is_linebreak).is_some());
        assert_eq!(lexer.current_slice(), "first line\n");
    }

    #[test]
    fn find_none_test() {
        let mut lexer = Lexer::new("first line the same first line");
        assert!(lexer.chars.find(is_linebreak).is_none());
        assert_eq!(lexer.current_slice(), "first line the same first line")
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
    fn let_only_test() {
        let input = "let";
        let expected_tokens = &[Let];
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
