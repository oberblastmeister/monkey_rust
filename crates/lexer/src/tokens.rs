#[derive(Debug, PartialEq)]
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
