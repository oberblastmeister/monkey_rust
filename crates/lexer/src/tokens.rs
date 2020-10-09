use std::str::FromStr;

pub enum Token {
    Illegal,
    // Eof,

    // identifies + literals
    Ident(String),
    Int(i64),

    // operators
    Assign,
    Plus,

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
}
