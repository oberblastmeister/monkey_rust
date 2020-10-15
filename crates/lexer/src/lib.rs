#[allow(dead_code)]
#[allow(unused_variables)]
mod tokens;

use tokens::Token::{self, *};

macro_rules! change_state {
    ($name:ident) => {
        return Some(StateFunction(Lexer::$name));
    };
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
    input: &'input str,
    start: usize,
    pos: usize,
    current_line: usize,
    token: Option<Token<'input>>
}

impl<'input> Lexer<'input> {
    pub fn new(input: &str) -> Lexer<'_> {
        Lexer {
            input,
            start: 0,
            pos: 0,
            token: None,
            current_line: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
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

    fn accept(&mut self, valid: &str) -> bool {
        match self.next_char() {
            Some(n) if valid.contains(n) => true,
            _ => {
                self.backup();
                false
            }
        }
    }

    fn accept_run(&mut self, valid: &str) {
        while let Some(n) = self.next_char() {
            if !valid.contains(n) {
                break;
            }
        }
        self.backup();
    }

    fn accept_while(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(n) = self.next_char() {
            if !predicate(n) {
                break;
            }
        }
        self.backup();
    }

    fn lex_main(l: &mut Lexer) -> Option<StateFunction> {
        let c = l.next_char()?;
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
            '>' => change_state!(gt),
            '<' => change_state!(lt),
            '!' => change_state!(bang_or_not_eq),
            _ if is_start_of_number(c) => change_state!(number),
            _ if is_letter(c) => change_state!(keyword),
            _ if is_whitespace(c) => change_state!(whitespace),
            _ => Illegal,
        };
        l.emit(token);
        None
    }

    fn whitespace(l: &mut Lexer) -> Option<StateFunction> {
        l.ignore();
        change_state!(lex_main)
    }

    fn slash_or_comment(l: &mut Lexer) -> Option<StateFunction> {
        if l.accept("/") {
            change_state!(comment);
        } else {
            l.emit(Token::Slash)
        }
        None
    }

    fn comment(l: &mut Lexer) -> Option<StateFunction> {
        while let Some(c) = l.next_char() {
            if c == '\n' {
                break;
            }
        }
        l.ignore();
        change_state!(lex_main)
    }

    fn assign_or_eq(l: &mut Lexer) -> Option<StateFunction> {
        if l.accept("=") {
            l.emit(Token::Eq)
        } else {
            l.emit(Token::Assign)
        }
        None
    }

    fn gt(l: &mut Lexer) -> Option<StateFunction> {
        if l.accept("=") {
            l.emit(Token::GtEq);
        } else {
            l.emit(Token::Gt);
        }
        None
    }

    fn bang_or_not_eq(l: &mut Lexer) -> Option<StateFunction> {
        if l.accept("=") {
            l.emit(Token::NotEq);
        } else {
            l.emit(Token::Bang);
        }
        None
    }

    fn lt(l: &mut Lexer) -> Option<StateFunction> {
        if l.accept("=") {
            l.emit(Token::LtEq);
        } else {
            l.emit(Token::Lt);
        }
        None
    }

    fn number(l: &mut Lexer) -> Option<StateFunction> {
        l.accept_while(is_digit);
        if l.accept(".") {
            l.accept_while(is_digit);
        }

        l.emit(Token::Number(&l.input[l.start..l.pos]));
        None
    }

    fn keyword(l: &mut Lexer) -> Option<StateFunction> {
        l.accept_while(is_letter);
        let token = match &l.input[l.start..l.pos] {
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
        let token = Ident(&l.input[l.start..l.pos]);
        l.emit(token);
        None
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut state = StateFunction::start_state();
        while let Some(next_state) = state {
            state = next_state.f(self)
        }
        self.token.take()
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

    fn test_lexer(input: &'static str, expected_tokens: &[Token]) {
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
}
