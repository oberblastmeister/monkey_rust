mod tokens;

use std::str::CharIndices;

use tokens::Token;


pub struct Lexer<'a> {
    input: CharIndices<'a>,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl<'a> Lexer<'a> {
    fn new(input: &str) -> Lexer {
        let mut input = input.char_indices();
        let (position, ch) = input.next().unwrap();
        let position = position as usize;
        let read_position: usize = position + 1;
        Lexer {
            input,
            position,
            read_position,
            ch: Some(ch)
        }
    }

    fn read_char(&mut self) {
        let res = self.input.next();
        match res {
            Some((position, ch)) => {
                self.ch = Some(ch);
                self.position = position as usize;
                self.read_position = (position + 1) as usize;
            }
            None => self.ch = None,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if ch.is_ascii_whitespace() {
                self.read_char()
            } else {
                break
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut res = String::new();
        self.position = self.input.find_map(|(position, ch)| {
            if !is_letter(ch) { Some(position) } else { None }
        }).unwrap();
        res
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        self.skip_whitespace();

        let token = match self.ch {
            Some(ch) => {
                let token = match ch {
                    '=' => Assign,
                    ';' => Semicolon,
                    '(' => Lparen,
                    ')' => Rparen,
                    ',' => Comma,
                    '+' => Plus,
                    '{' => Lbrace,
                    '}' => Rbrace,
                    _ => if is_letter(ch) {
                        match 
                    }
                };
                Some(token)
            }
            None => {
                None
            }
        };
        self.read_char();
        token
    }
}

const fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
