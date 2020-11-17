mod ast;
mod parse_error;
mod parser;

pub use parser::Parser;
pub use parse_error::{ParseResult, ParseError};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
