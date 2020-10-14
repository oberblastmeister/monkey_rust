#[allow(dead_code)]
#[allow(unused_variables)]
mod tokens;

use std::sync::mpsc::{channel, Sender};

use tokens::Token;

macro_rules! change_state {
    ($name:ident) => {
        return Some(StateFunction(Lexer::$name));
    };
}

struct StateFunction(fn(&mut Lexer) -> Option<StateFunction>);

enum State<'a> {
    Function(fn(&mut Lexer) -> State<'a>),
    Token(Token<'a>),
}

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

pub struct Lexer<'a> {
    input: &'a str,
    start: usize,
    pos: usize,
    token_sender: Sender<Token<'a>>,
    current_line: usize,
}

impl<'a> Lexer<'a> {
    pub fn begin_lexing(input: &'a str, sender: Sender<Token<'a>>) {
        let mut lexer = Lexer {
            input,
            start: 0,
            pos: 0,
            token_sender: sender,
            current_line: 0,
        };
        lexer.run()
    }

    fn run(&mut self) {
        let mut state = StateFunction::start_state();
        while let Some(next_state) = state {
            state = next_state.f(self)
        }
    }

    fn next(&mut self) -> Option<char> {
        match self.input[self.pos..].chars().next() {
            Some(c) => {
                if is_linebreak(c) {
                    self.current_line += 1;
                }
                self.pos += 1;
                Some(c)
            }
            None => None,
        }
    }

    fn current_slice(&self) -> &str {
        &self.input[self.start..self.pos]
    }

    fn backup(&mut self) {
        self.pos -= 1;
    }

    fn ignore(&mut self) {
        self.start = self.pos;
    }

    fn emit(&mut self, token: Token<'a>) {
        self.token_sender
            .send(token)
            .expect("Unable to send token on channel");
        self.ignore();
    }

    fn accept(&mut self, valid: &str) -> bool {
        match self.next() {
            Some(n) if valid.contains(n) => true,
            _ => {
                self.backup();
                false
            }
        }
    }

    fn accept_run(&mut self, valid: &str) {
        while let Some(n) = self.next() {
            if !valid.contains(n) {
                break;
            }
        }
        self.backup();
    }

    fn accept_while(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(n) = self.next() {
            if !predicate(n) {
                break;
            }
        }
        self.backup();
    }

    fn lex_main(l: &mut Lexer) -> Option<StateFunction> {
        use Token::*;

        while let Some(c) = l.next() {
            let token = match c {
                '=' => change_state!(assign_or_eq),
                ';' => Semicolon,
                '(' => Lparen,
                ')' => Rparen,
                ',' => Comma,
                '+' => Plus,
                '*' => Asterisk,
                '/' => change_state!(slash_or_comment),
                '-' => Minus,
                '{' => Lbrace,
                '}' => Rbrace,
                '>' => change_state!(lex_gt),
                '<' => change_state!(lex_lt),
                '!' => change_state!(lex_bang_or_not_eq),
                _ if is_start_of_number(c) => change_state!(lex_number),
                _ if is_letter(c) => change_state!(keyword),
                _ if is_whitespace(c) => change_state!(whitespace),
                _ => Illegal,
            };
            l.emit(token);
        }
        None
    }

    fn whitespace(l: &mut Lexer) -> Option<StateFunction> {
        l.ignore();
        change_state!(lex_main);
    }

    fn slash_or_comment(l: &mut Lexer) -> Option<StateFunction> {
        if l.accept("/") {
            change_state!(comment);
        } else {
            l.emit(Token::Slash)
        }
        change_state!(lex_main);
    }

    fn comment(l: &mut Lexer) -> Option<StateFunction> {
        while let Some(c) = l.next() {
            if c == '\n' {
                break;
            }
        }
        l.ignore();
        change_state!(lex_main);
    }

    fn assign_or_eq(l: &mut Lexer) -> Option<StateFunction> {
        if l.accept("=") {
            l.emit(Token::Eq)
        } else {
            l.emit(Token::Assign)
        }
        change_state!(lex_main);
    }

    fn lex_gt(l: &mut Lexer) -> Option<StateFunction> {
        if l.accept("=") {
            l.emit(Token::GtEq);
        } else {
            l.emit(Token::Gt);
        }
        change_state!(lex_main);
    }

    fn lex_bang_or_not_eq(l: &mut Lexer) -> Option<StateFunction> {
        if l.accept("=") {
            l.emit(Token::NotEq);
        } else {
            l.emit(Token::Bang);
        }
        change_state!(lex_main)
    }

    fn lex_lt(l: &mut Lexer) -> Option<StateFunction> {
        if l.accept("=") {
            l.emit(Token::LtEq);
        } else {
            l.emit(Token::Lt);
        }
        change_state!(lex_main);
    }

    fn lex_number(l: &mut Lexer) -> Option<StateFunction> {
        l.accept_while(is_digit);
        if l.accept(".") {
            l.accept_while(is_digit);
        }

        l.emit(Token::Number(&l.input[l.start..l.pos]));
        change_state!(lex_main);
    }

    fn keyword(l: &mut Lexer) -> Option<StateFunction> {
        l.accept_while(is_letter);
        let current_slice = &l.input[l.start..l.pos];
        let token = match current_slice {
            "fn" => Token::Function,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "true" => Token::True,
            "false" => Token::False,
            _ => change_state!(ident),
        };
        l.emit(token);
        None
    }

    fn ident(l: &mut Lexer) -> Option<StateFunction> {
        let current_slice = &l.input[l.start..l.pos];
        let token = Token::Ident(current_slice);
        l.emit(token);
        change_state!(lex_main);
    }
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
    use std::thread;
    use Token::*;

    fn test_lexer(input: &'static str, expected_tokens: &[Token]) {
        let (sender, receiver) = channel();
        let handle = thread::spawn(move || {
            Lexer::begin_lexing(input, sender);
        });
        let mut idx = 0;
        while let Ok(token) = receiver.recv() {
            if expected_tokens[idx] != token {
                panic!(
                    "Expected token {:?} does not equal actual token {:?} (idx {})",
                    expected_tokens[idx], token, idx
                )
            }
            idx += 1;
        }
        handle.join().expect("Failed to join thread");
    }

    #[test]
    fn lex1_test() {
        let input = "let five = 5;";
        let expected_tokens = &[Let, Ident("five"), Assign, Number("5"), Semicolon];
        test_lexer(input, expected_tokens);
    }

    #[test]
    fn lex_funtion_test() {
        let input = "let add = fn(x, y) {
    x + y;
}";
        let expected_tokens = [
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
            Rparen,
        ];
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
    fn comment_test() {
        let input = "// this is a comment
20 / 2;";

        let expected_tokens = &[Number("20"), Slash, Number("2"), Semicolon];

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
            Rbrace,
            Return,
            True,
            Semicolon,
            Comma,
            Lbrace,
            Else,
            Rbrace,
            Return,
            False,
            Semicolon,
            Rparen,
        ];
        test_lexer(input, expected_tokens);
    }
}
