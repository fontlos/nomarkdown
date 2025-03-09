use nom::combinator::map;
use nom::{IResult, Parser};

use super::Markdown;
use super::utils::fenced;

pub fn math(input: &str) -> IResult<&str, Markdown> {
    map(fenced("`", "`"), Markdown::Math).parse(input)
}
