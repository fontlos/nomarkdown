use nom::{IResult, Parser};
use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::char,
    combinator::map,
    sequence::{delimited, pair, preceded},
};
use urlocator::{UrlLocation, UrlLocator};

use super::Markdown;
use super::utils::fenced;

fn raw_link(input: &str) -> IResult<&str, &str> {
    let mut locator = UrlLocator::new();
    let mut end = 0;
    for c in input.chars() {
        match locator.advance(c) {
            UrlLocation::Url(s, _e) => {
                end = s as usize;
            }
            UrlLocation::Reset => break,
            UrlLocation::Scheme => {}
        }
    }
    if end > 0 {
        Ok((&input[end..], &input[0..end]))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::RegexpFind,
        )))
    }
}

fn url_link(input: &str) -> IResult<&str, (&str, &str)> {
    pair(
        fenced("[", "]"),
        //匹配始于 ’(‘，匹配非')'的字符，匹配止于')'
        delimited(char('('), is_not(")"), char(')')),
    )
    .parse(input)
}

fn image(input: &str) -> IResult<&str, (&str, &str)> {
    // 先用第一个解析器匹配并丢弃该值，紧接着用第二个解析器匹配并返回该值
    preceded(char('!'), url_link).parse(input)
}

pub fn parse_link(input: &str) -> IResult<&str, Markdown> {
    alt((
        map(raw_link, Markdown::RawLink),
        map(url_link, |(title, url)| Markdown::UrlLink { title, url }),
        map(image, |(alt, url)| Markdown::Image { alt, url }),
    ))
    .parse(input)
}
