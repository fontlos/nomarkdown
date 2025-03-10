//! 用于匹配各种文字变体
//! 包括粗斜体, 粗体, 斜体, 删除线, 高亮

use nom::{
    IResult, Parser,
    error::{Error, ErrorKind},
};
use nom::{branch::alt, bytes::complete::tag, sequence::preceded};

use super::Markdown;
use super::parse::line_element_parser;

// 类似 take_until, 但是消耗定界符,
// 有这个函数是为了保证严格模式解析器形式上与解析粗体和斜体时一致
// 并且考虑到我们可能需要处理转义字符, 需要更细粒度的控制
fn take_until_boundary<'a>(
    boundary: &'a str,
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, Error<&'a str>> {
    move |input: &'a str| {
        // 严格模式下, 针对粗体和斜体定界符, 考虑嵌套问题
        #[cfg(feature = "strict")]
        match boundary {
            "**" => {
                // take_until 内部差不多就是这样实现的, 我们只针对字符串还能更简单一点
                // 尝试寻找 `**`, 如果找到了, 那么看下一个字符是不是 `*`,
                // 如果是, 那么向后移动一格并完成匹配, 如果不是, 直接完成匹配
                // 如果没找到, 则匹配失败
                if let Some(pos) = input.find("**") {
                    // 如果实际上找到了 `***``, 向后移动一格
                    if input[pos + 2..].starts_with('*') {
                        return Ok((&input[pos + 3..], &input[..pos + 1]));
                    } else {
                        return Ok((&input[pos + 2..], &input[..pos]));
                    }
                }
                return Err(nom::Err::Error(Error::new(input, ErrorKind::TakeUntil)));
            }
            "*" => {
                // 对于斜体更复杂一点, 需要记录一下当前位置
                let mut start = 0;
                while let Some(pos) = input[start..].find('*') {
                    let pos = start + pos;
                    if input[pos + 1..].starts_with("*") {
                        start = pos + 2; // 跳过 `**`
                    } else {
                        return Ok((&input[pos + 1..], &input[..pos]));
                    }
                }
                return Err(nom::Err::Error(Error::new(input, ErrorKind::TakeUntil)));
            }
            _ => {}
        };
        if let Some(pos) = input.find(boundary) {
            return Ok((&input[pos + boundary.len()..], &input[..pos]));
        }
        Err(nom::Err::Error(Error::new(input, ErrorKind::TakeUntil)))
    }
}

fn text_parser_gen<'a, F>(
    boundary: &'a str,
    map: F,
) -> impl Parser<&'a str, Output = Markdown<'a>, Error = Error<&'a str>>
where
    F: Fn(Vec<Markdown<'a>>) -> Markdown<'a> + 'a,
{
    // 两种不同模式的切换
    #[cfg(not(feature = "strict"))]
    return preceded(tag(boundary), take_until_boundary(boundary))
        .and_then(line_element_parser)
        .map(map);
    // 文字变体解析器生成器严格模式, 这里处理的是不再匹配定界符不紧挨文字的情况
    #[cfg(feature = "strict")]
    move |input: &'a str| {
        // 首先我们解析文字变体, 保留剩余部分
        let (remaining, parsed) =
            preceded(tag(boundary), take_until_boundary(boundary)).parse(input)?;
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
    fn test_take_until_boundary() {
        let input = "粗斜体***其他内容";
        let (remaining, parsed) = take_until_boundary("***").parse(input).unwrap();
        assert_eq!(remaining, "其他内容");
        assert_eq!(parsed, "粗斜体");

        let input = "粗体**其他内容";
        let (remaining, parsed) = take_until_boundary("**").parse(input).unwrap();
        assert_eq!(remaining, "其他内容");
        assert_eq!(parsed, "粗体");

        let input = "斜体*其他内容";
        let (remaining, parsed) = take_until_boundary("*").parse(input).unwrap();
        assert_eq!(remaining, "其他内容");
        assert_eq!(parsed, "斜体");

        let input = "删除线~~其他内容";
        let (remaining, parsed) = take_until_boundary("~~").parse(input).unwrap();
        assert_eq!(remaining, "其他内容");
        assert_eq!(parsed, "删除线");

        let input = "高亮==其他内容";
        let (remaining, parsed) = take_until_boundary("==").parse(input).unwrap();
        assert_eq!(remaining, "其他内容");
        assert_eq!(parsed, "高亮");
    }

    #[test]
    #[cfg(feature = "strict")]
    fn test_take_until_boundary_nested() {
        // 粗体套斜体
        // 嵌套并且后面有其他内容
        let input = "粗体*嵌套的斜体***其他内容";
        let (remaining, parsed) = take_until_boundary("**").parse(input).unwrap();
        assert_eq!(remaining, "其他内容");
        assert_eq!(parsed, "粗体*嵌套的斜体*");
        // 嵌套但是后面没有其他内容
        let input = "粗体*嵌套的斜体***";
        let (remaining, parsed) = take_until_boundary("**").parse(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(parsed, "粗体*嵌套的斜体*");

        // 斜体套粗体
        // 嵌套并且后面有其他内容
        let input = "斜体**内嵌的粗体1****内嵌的粗体2***其他内容";
        let (remaining, parsed) = take_until_boundary("*").parse(input).unwrap();
        assert_eq!(remaining, "其他内容");
        assert_eq!(parsed, "斜体**内嵌的粗体1****内嵌的粗体2**");
        // 嵌套但是后面没有其他内容
        let input = "斜体**内嵌的粗体1****内嵌的粗体2***";
        let (remaining, parsed) = take_until_boundary("*").parse(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(parsed, "斜体**内嵌的粗体1****内嵌的粗体2**");
    }

    #[test]
    #[should_panic] // 无效
    #[cfg(feature = "strict")]
    fn test_take_until_boundary_invalid_bold() {
        // 抑制 panic 输出
        std::panic::set_hook(Box::new(|_| {}));
        // 粗体匹配到末尾也没发现对应的定界符
        let _ = take_until_boundary("**").parse("粗体*嵌套的斜体*").unwrap();
    }

    #[test]
    #[should_panic]
    #[cfg(feature = "strict")]
    fn test_take_until_boundary_invalid_italic() {
        std::panic::set_hook(Box::new(|_| {}));
        let _ = take_until_boundary("*")
            .parse("斜体**内嵌的粗体****内嵌的粗体2**")
            .unwrap();
    }

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
                Markdown::Strike(vec![Markdown::Text("合法删除线"),]),
                Markdown::Text("  "),
                Markdown::Text("***")
            ])
        );
    }
}
