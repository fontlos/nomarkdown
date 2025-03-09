use nom::{IResult, Parser};
use nom::{branch::alt, bytes::complete::take_till};

use super::Markdown;
use super::{code, math, text};

pub fn line_element_parser(input: &str) -> IResult<&str, Vec<Markdown>> {
    let mut markdown = Vec::<Markdown>::with_capacity(4);
    let mut current_input = &input[..];
    while !current_input.is_empty() {
        // 一直匹配到可能含有语法结构才进行解析
        // 按照常用顺序排列, 对应: 换行, 粗体斜体, 链接, 行内代码, 图片, 转义字符, 行内数学公式, 删除线, 高亮
        // 被匹配到的是文字, 没被匹配到的才含有语法
        let (syntax, text) = take_till(|c| {
            c == '\n'
                || c == '*'
                || c == '['
                || c == '`'
                || c == '!'
                || c == '\\'
                || c == '$'
                || c == '~'
                || c == '='
        })
        .parse(current_input)?;
        if !text.is_empty() {
            markdown.push(Markdown::Text(text));
        }
        match line_element_syntax(syntax) {
            Ok((remain, parsed)) => {
                markdown.push(parsed);
                current_input = remain;
            }
            Err(_) => {
                break;
            }
        }
    }
    Ok(("", markdown))
}

fn line_element_syntax(input: &str) -> IResult<&str, Markdown> {
    alt((text::text_parser, code::code, math::math)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "文字***加粗斜体***文字**粗体**文字*斜体*文字~~删除线~~文字==高亮==文字";
        let (_, md) = line_element_parser(input).unwrap();
        println!("{:?}", md);
    }
}
