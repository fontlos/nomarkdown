use nom::combinator::map;
use nom::{IResult, Parser};

use super::Markdown;
use super::utils::fenced;

pub fn code(input: &str) -> IResult<&str, Markdown> {
    map(fenced("`", "`"), Markdown::Code).parse(input)
}
