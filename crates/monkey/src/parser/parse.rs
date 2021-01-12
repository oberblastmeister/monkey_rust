use super::{ParseResult, Parser};

pub trait Parse
where
    Self: Sized,
{
    fn parse(p: &mut Parser) -> ParseResult<Self>;
}
