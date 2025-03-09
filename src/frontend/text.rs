//! 用于匹配各种文字变体
//! 包括粗斜体, 粗体, 斜体, 删除线, 高亮

use nom::{IResult, Parser, error::Error};
use nom::{
    branch::alt,
    combinator::{map, map_parser},
    bytes::complete::{tag, take_until},
};

use super::Markdown;
use super::parse::line_element_parser;

/// 生成处理被特定模式包裹的字符串的函数
pub fn fenced<'a>(
    start: &'a str,
    end: &'a str,
) -> impl Parser<&'a str, Output = &'a str, Error = Error<&'a str>> {
    map((tag(start), take_until(end), tag(end)), |x| x.1)
}

/// 文字变体解析器生成器
fn text_parser_gen<'a>(
    boundary: &'a str,
) -> impl Parser<&'a str, Output = Vec<Markdown<'a>>, Error = Error<&'a str>> {
    // 将第一个解析器的匹配结果应用于第二个解析器
    map_parser(fenced(boundary, boundary), line_element_parser)
}

fn bold_italic(input: &str) -> IResult<&str, Markdown> {
    map(text_parser_gen("***"), Markdown::BoldItalic).parse(input)
}

fn bold(input: &str) -> IResult<&str, Markdown> {
    map(text_parser_gen("**"), Markdown::Bold).parse(input)
}

fn italic(input: &str) -> IResult<&str, Markdown> {
    map(text_parser_gen("*"), Markdown::Italic).parse(input)
}

fn strike(input: &str) -> IResult<&str, Markdown> {
    map(text_parser_gen("~~"), Markdown::Strike).parse(input)
}

fn highlight(input: &str) -> IResult<&str, Markdown> {
    map(text_parser_gen("=="), Markdown::Highlight).parse(input)
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
