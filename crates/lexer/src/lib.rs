#[allow(dead_code)]
#[allow(unused_variables)]
mod tokens;

use std::str;

use log::debug;
use log::info;
use log::trace;
use tokens::Token::{self, *};

/// macro to make return state functions easier
macro_rules! change_state {
    ($name:ident) => {
        {
            info!("Next state function: {}::{}", "Lexer", stringify!($name));
            return Some(StateFunction(Lexer::$name));
        }
    };
}

/// use macro instead of function to bypass lifetimes
macro_rules! current_slice {
    ($lexer:ident) => {
        {
            let bytes = &$lexer.input[$lexer.start..$lexer.pos];
            let s = str::from_utf8(bytes).unwrap();
            s
        }
    }
}

struct StateFunction(fn(&mut Lexer) -> Option<StateFunction>);

impl StateFunction {
    fn start_state() -> Option<StateFunction> {
        change_state!(lex_main);
    }
}

impl StateFunction {
    fn f(&self, lexer: &mut Lexer) -> Option<StateFunction> {
        self.0(lexer)
    }
}

pub struct Lexer<'input> {
    input: &'input [u8],
    start: usize,
    pos: usize,
    current_line: usize,
    token: Option<Token<'input>>
}

impl<'input> Lexer<'input> {
    pub fn new(input: &str) -> Lexer<'_> {
        Lexer {
            input: input.as_bytes(),
            start: 0,
            pos: 0,
            token: None,
            current_line: 0,
        }
    }

    // fn next_byte(&mut self) -> Option<char> {
    //     let res = self.input[self.pos..].chars().next();

    //     if let Some(c) = res {
    //         if is_linebreak(c) {
    //             self.current_line += 1;
    //             debug!("Next char: {:?}", c);
    //         }
    //     } else {
    //         debug!("No next char");
    //     }
    //     debug!("Pos: {}", self.pos);
    //     self.pos += 1;
    //     res
    // }

    fn next_byte(&mut self) -> Option<u8> {
        debug!("current slice: `{:?}`", &self.input[self.pos..]);
        debug!("current range: {:?}", self.pos..);
        trace!("input len: {}", self.input.len());
        debug!("Pos: {}", self.pos);
        // let res = if self.pos >= self.input.len()
        let res = if self.pos >= self.input.len() {
            None
        } else {
            let b = self.input[self.pos];
            Some(b)
        };
        // let res = self.input[self.pos..].chars().next();
        debug!("res: {:?}", res);
        // let res = self.input.get(self.pos..).and_then(|s| s.chars().next());

        if let Some(c) = res {
            if is_linebreak(c) {
                self.current_line += 1;
            }
        } else {
        }
        self.pos += 1;
        res
    }

    fn backup(&mut self) {
        self.pos -= 1;
    }

    fn ignore(&mut self) {
        self.start = self.pos;
    }

    fn emit(&mut self, token: Token<'input>) {
        match self.token {
            Some(ref token) => panic!(format!("Attempting to override token `{:?}` using emit", token)),
            None => self.token = Some(token),
        }
        self.ignore();
    }

    fn accept(&mut self, valid: u8) -> bool {
        match self.next_byte() {
            Some(n) if n == valid => true,
            _ => {
                self.backup();
                false
            }
        }
    }

    fn accept_multiple(&mut self, valid: &[u8]) -> bool {
        match self.next_byte() {
            Some(n) if valid.contains(&n) => true,
            _ => {
                self.backup();
                false
            }
        }
    }

    fn accept_run(&mut self, valid: &[u8]) {
        while let Some(n) = self.next_byte() {
            if !valid.contains(&n) {
                break;
            }
        }
        self.backup();
    }

    fn accept_while(&mut self, predicate: impl Fn(u8) -> bool) {
        while let Some(n) = self.next_byte() {
            debug!("accept while char: {:?}", n);
            if !predicate(n) {
                break;
            }
        }
        self.backup();
    }

    fn lex_main(l: &mut Lexer) -> Option<StateFunction> {
        let token = loop {
            let b = l.next_byte()?;
            break match b {
                b'=' => change_state!(assign_or_eq),
                b';' => Semicolon,
                b'(' => Lparen,
                b')' => Rparen,
                b',' => Comma,
                b'+' => Plus,
                b'*' => Asterisk,
                b'/' => change_state!(slash_or_comment),
                b'-' => Minus,
                b'{' => Lbrace,
                b'}' => Rbrace,
                b'>' => change_state!(gt),
                b'<' => change_state!(lt),
                b'!' => change_state!(bang_or_not_eq),
                _ if is_start_of_number(b) => change_state!(number),
                _ if is_letter(b) => change_state!(keyword),
                _ if is_whitespace(b) => change_state!(whitespace),
                _ => continue,
                // _ => Illegal,
            }
        };
        l.emit(token);
        None
    }

    fn whitespace(l: &mut Lexer) -> Option<StateFunction> {
        l.ignore();
        change_state!(lex_main)
    }

    fn slash_or_comment(l: &mut Lexer) -> Option<StateFunction> {
        if l.accept(b'/') {
            change_state!(comment);
        } else {
            l.emit(Token::Slash)
        }
        None
    }

    fn comment(l: &mut Lexer) -> Option<StateFunction> {
        while let Some(c) = l.next_byte() {
            if c == b'\n' {
                break;
            }
        }
        l.ignore();
        change_state!(lex_main)
    }

    fn assign_or_eq(l: &mut Lexer) -> Option<StateFunction> {
        if l.accept(b'=') {
            l.emit(Token::Eq)
        } else {
            l.emit(Token::Assign)
        }
        None
    }

    fn gt(l: &mut Lexer) -> Option<StateFunction> {
        if l.accept(b'=') {
            l.emit(Token::GtEq);
        } else {
            l.emit(Token::Gt);
        }
        None
    }

    fn bang_or_not_eq(l: &mut Lexer) -> Option<StateFunction> {
        if l.accept(b'=') {
            l.emit(Token::NotEq);
        } else {
            l.emit(Token::Bang);
        }
        None
    }

    fn lt(l: &mut Lexer) -> Option<StateFunction> {
        if l.accept(b'=') {
            l.emit(Token::LtEq);
        } else {
            l.emit(Token::Lt);
        }
        None
    }

    fn number(l: &mut Lexer) -> Option<StateFunction> {
        l.accept_while(is_digit);
        if l.accept(b'.') {
            l.accept_while(is_digit);
        }

        l.emit(Token::Number(current_slice!(l)));
        None
    }

    fn keyword(l: &mut Lexer) -> Option<StateFunction> {
        l.accept_while(is_letter);
        let token = match str::from_bytes(&l.input[l.start..l.pos]) {
            "fn" => Function,
            "return" => Return,
            "let" => Let,
            "if" => If,
            "else" => Else,
            "true" => True,
            "false" => False,
            _ => change_state!(ident),
        };
        l.emit(token);
        None
    }

    fn ident(l: &mut Lexer) -> Option<StateFunction> {
        l.emit(Ident(current_slice!(l)));
        None
    }
}

trait FromBytes {
    fn from_bytes<'a>(bytes: &'a [u8]) -> &'a str;
}

impl FromBytes for str {
    fn from_bytes<'a>(bytes: &'a [u8]) -> &'a str {
        str::from_utf8(bytes).unwrap()
    }
}

// impl From<&[u8]> for &str {
//     fn from(bytes: &[u8]) -> Self {
//         str::from_utf8(bytes).unwrap()
//     }
// }

impl<'input> Iterator for Lexer<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut state = StateFunction::start_state();
        while let Some(next_state) = state {
            state = next_state.f(self);
        }
        self.token.take()
    }
}

fn current_slice<'a>(l: &'a Lexer) -> &'a [u8] {
    &l.input[l.start..l.pos]
}

const fn is_linebreak(c: u8) -> bool {
    c == b'\n'
}

const fn is_whitespace(c: u8) -> bool {
    c.is_ascii_whitespace()
}

const fn is_start_of_number(c: u8) -> bool {
    is_digit(c) || c == b'.'
}

const fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}

const fn is_letter(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
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
