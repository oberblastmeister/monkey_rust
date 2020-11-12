use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Token<'a> {
    Illegal,

    // identifies + literals
    Ident(&'a str),
    Number(&'a str),

    // operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    LtEq,
    GtEq,

    // delimiters
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,

    Eq,
    NotEq,
}

impl<'a> Token<'a> {
    pub fn as_str(&self) -> &str {
        use Token::*;

        match self {
            Illegal => "ILLEGAL",

            Ident(s) => s,
            Number(s) => s,

            Assign => "=",
            Plus => "+",
            Asterisk => "*",
            Slash => "/",
            Lt => "<",
            Gt => ">",
            LtEq => "<=",
            GtEq => ">=",

            Comma => ",",
            Semicolon => ";",

            Lparen => "(",
            Rparen => ")",
            Lbrace => "{",
            Rbrace => "}",

            // keywords
            Function => "fn",
            Let => "let",
            True => "true",
            False => "false",
            If => "if",
            Else => "else",
            Return => "return",

            Eq => "==",
            NotEq => "!=",
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
