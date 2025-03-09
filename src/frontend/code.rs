use nom::combinator::map;
use nom::{IResult, Parser};

use super::Markdown;
use super::utils::fenced;

pub fn code(input: &str) -> IResult<&str, Markdown> {
    map(fenced("`", "`"), Markdown::Code).parse(input)
}

// pub fn code_block(input: &str) -> IResult<&str, (&str, &str)> {
//     map(
//         (tag("```"), take_until("\n"), take_until("```"), tag("```")),
//         |x| (x.1, x.2),
//     )
//     .parse(input)
// }
