mod ast;
mod parse_error;
mod parse;
mod parser;

pub use parse::Parse;
pub use parser::Parser;
pub use parse_error::ParseError;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
