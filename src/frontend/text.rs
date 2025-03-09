//! 用于匹配各种文字变体
//! 包括粗斜体, 粗体, 斜体, 删除线, 高亮

use nom::{IResult, Parser, error::Error};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    sequence::delimited,
};

use super::Markdown;
use super::parse::line_element_parser;

/// 文字变体解析器生成器
fn text_parser_gen<'a>(
    boundary: &'a str,
) -> impl Parser<&'a str, Output = Vec<Markdown<'a>>, Error = Error<&'a str>> {
    delimited(tag(boundary), take_until(boundary), tag(boundary)).and_then(line_element_parser)
}

fn bold_italic(input: &str) -> IResult<&str, Markdown> {
    text_parser_gen("***")
        .map(Markdown::BoldItalic)
        .parse(input)
}

fn bold(input: &str) -> IResult<&str, Markdown> {
    text_parser_gen("**").map(Markdown::Bold).parse(input)
}

fn italic(input: &str) -> IResult<&str, Markdown> {
    text_parser_gen("*").map(Markdown::Italic).parse(input)
}

fn strike(input: &str) -> IResult<&str, Markdown> {
    text_parser_gen("~~").map(Markdown::Strike).parse(input)
}

fn highlight(input: &str) -> IResult<&str, Markdown> {
    text_parser_gen("==").map(Markdown::Highlight).parse(input)
}

pub fn text_parser(input: &str) -> IResult<&str, Markdown> {
    alt((bold_italic, bold, italic, strike, highlight)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_parser() {
        let (_, md) = text_parser("***加粗斜体***").unwrap();
        assert_eq!(md, Markdown::BoldItalic(vec![Markdown::Text("加粗斜体")]));
        let (_, md) = text_parser("***加粗斜体 *嵌套斜体* 剩余加粗斜体***").unwrap();
        assert_eq!(
            md,
            Markdown::BoldItalic(vec![
                Markdown::Text("加粗斜体 "),
                Markdown::Italic(vec![Markdown::Text("嵌套斜体")]),
                Markdown::Text(" 剩余加粗斜体")
            ])
        );
        println!("{:?}", md);
    }
}
