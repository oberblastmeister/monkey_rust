use super::parse_error::ParseError;
use super::Parser;

pub trait Parse
where
    Self: Sized,
{
    fn parse(p: &mut Parser) -> Result<Self, ParseError>;
}

impl<T> Parse for Box<T>
where
    T: Parse,
{
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        Ok(Box::new(parser.parse()?))
    }
}
