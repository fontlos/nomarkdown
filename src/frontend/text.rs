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

#[cfg(not(feature = "strict"))]
/// 文字变体解析器生成器
fn text_parser_gen<'a, F>(
    boundary: &'a str,
    map: F,
) -> impl Parser<&'a str, Output = Markdown<'a>, Error = Error<&'a str>>
where
    F: Fn(Vec<Markdown<'a>>) -> Markdown<'a> + 'a,
{
    delimited(tag(boundary), take_until(boundary), tag(boundary))
        .and_then(line_element_parser)
        .map(map)
}

#[cfg(feature = "strict")]
/// 文字变体解析器生成器严格模式
fn text_parser_gen<'a>(
    boundary: &'a str,
    map: fn(Vec<Markdown<'a>>) -> Markdown<'a>,
) -> impl Parser<&'a str, Output = Markdown<'a>, Error = Error<&'a str>> {
    move |input: &'a str| {
        // 首先我们解析文字变体, 保留剩余部分
        let (remaining, parsed) =
            delimited(tag(boundary), take_until(boundary), tag(boundary)).parse(input)?;
        // 对于解析部分看是否符合标准, 如果两边有空白字符, 那么不作为文字变体, 定界符也将作为普通字符, 但内部被匹配的部分将继续解析
        if parsed.starts_with(|c: char| c.is_whitespace())
            || parsed.ends_with(|c: char| c.is_whitespace())
        {
            let (_, elements) = line_element_parser(parsed)?;
            let mut res = Vec::with_capacity(3);
            // 定位第一个定界符, 从当前输入开头一直到定界符长度
            res.push(Markdown::Text(&input[..boundary.len()]));
            res.extend(elements);
            // 定位第二个定界符, 从当前输入被匹配的内容的结尾, 到这个位置再加上一个定界符的长度
            res.push(Markdown::Text(
                &input[boundary.len() + parsed.len()..2 * boundary.len() + parsed.len()],
            ));
            Ok((remaining, Markdown::Vanilla(res)))
        } else {
            let (_, elements) = line_element_parser(parsed)?;
            Ok((remaining, map(elements)))
        }
    }
}

fn bold_italic(input: &str) -> IResult<&str, Markdown> {
    text_parser_gen("***", Markdown::BoldItalic).parse(input)
}

fn bold(input: &str) -> IResult<&str, Markdown> {
    text_parser_gen("**", Markdown::Bold).parse(input)
}

fn italic(input: &str) -> IResult<&str, Markdown> {
    text_parser_gen("*", Markdown::Italic).parse(input)
}

fn strike(input: &str) -> IResult<&str, Markdown> {
    text_parser_gen("~~", Markdown::Strike).parse(input)
}

fn highlight(input: &str) -> IResult<&str, Markdown> {
    text_parser_gen("==", Markdown::Highlight).parse(input)
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
    }

    #[test]
    #[cfg(feature = "strict")]
    fn test_text_parser_strict() {
        let (_, md) = text_parser("*** 不合法加粗斜体 ~~合法删除线~~  ***").unwrap();
        assert_eq!(
            md,
            Markdown::Vanilla(vec![
                Markdown::Text("***"),
                Markdown::Text(" 不合法加粗斜体 "),
                Markdown::Strike(vec![
                    Markdown::Text("合法删除线"),
                ]),
                Markdown::Text("  "),
                Markdown::Text("***")
            ])
        );
    }
}
