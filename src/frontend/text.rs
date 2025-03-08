use nom::{IResult, Parser, error::Error};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::multispace0,
    combinator::{map, map_parser},
};

use super::Markdown;
use super::parse::parse;
use super::utils::fenced;

/// 形式解析
fn text<'a>(
    boundary: &'a str,
) -> impl Parser<&'a str, Output = Vec<Markdown<'a>>, Error = Error<&'a str>> {
    // 将第一个解析器的匹配结果应用于第二个解析器
    map_parser(fenced(boundary, boundary), parse)
}

fn bold_italic(input: &str) -> IResult<&str, Vec<Markdown>> {
    text("***").parse(input)
}

fn bold(input: &str) -> IResult<&str, Vec<Markdown>> {
    text("**").parse(input)
}

fn italic(input: &str) -> IResult<&str, Vec<Markdown>> {
    text("*").parse(input)
}

fn strike(input: &str) -> IResult<&str, Vec<Markdown>> {
    text("~~").parse(input)
}

fn highlight(input: &str) -> IResult<&str, Vec<Markdown>> {
    text("==").parse(input)
}

fn block_quote(input: &str) -> IResult<&str, Vec<Markdown>> {
    map_parser(
        map((multispace0, tag(">"), multispace0, is_not("\n")), |x| x.3),
        parse,
    )
    .parse(input)
}

pub fn parse_text(input: &str) -> IResult<&str, Markdown> {
    alt((
        map(bold_italic, Markdown::BoldItalic),
        map(bold, Markdown::Bold),
        map(italic, Markdown::Italic),
        map(strike, Markdown::Strike),
        map(highlight, Markdown::Highlight),
        map(block_quote, Markdown::BlockQuote),
    ))
    .parse(input)
}
