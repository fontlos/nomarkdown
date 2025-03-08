use nom::{IResult, Parser};
use nom::{
    branch::alt,
    combinator::{all_consuming, map},
};

use super::Markdown;
use super::{
    code::{code, code_block},
    head::head,
    link::parse_link,
    text::parse_text,
    utils::{command, escapable, horizontal_rule, new_line},
};

impl<'a> Markdown<'a> {
    pub fn from(input: &'a str) -> Result<Vec<Markdown<'a>>, nom::Err<nom::error::Error<&'a str>>> {
        alt((all_consuming(parse),))
            .parse(input)
            .map(|(_, result)| result)
    }
}

pub fn parse(input: &str) -> IResult<&str, Vec<Markdown>> {
    let mut markdown = Vec::with_capacity(4);
    // 此刻输入
    let mut current_input = input;
    // 逐字节解析字符串，直至字符串为空
    while !current_input.is_empty() {
        // 当前字符是否为纯文本
        let mut find_syntax = false;
        // 从此刻输入的第一个字符开始遍历，每次记录遍历起始索引
        for (current_index, _) in current_input.char_indices() {
            // 开始匹配语法，匹配当前索引一直到结尾的所有字符
            match syntax(&current_input[current_index..]) {
                // 剩余部分和成功匹配的部分
                Ok((remaining, parsed)) => {
                    // 如果匹配到语法，从第一个字符到当前字符应该被匹配掉，所以为空
                    let remainder = &current_input[0..current_index];
                    // 如果不为空则存在多余文本，按普通字符处理压入输出
                    if !remainder.is_empty() {
                        markdown.push(Markdown::Text(remainder));
                    }
                    // 将匹配到的结构压入输出
                    markdown.push(parsed);
                    // 将剩余部分作为当前输入
                    current_input = remaining;
                    // 成功匹配到语法，无需下面的处理
                    find_syntax = true;
                    break;
                }
                Err(nom::Err::Error(_)) => {
                    // 未发现任何语法，则该字符串为普通字符，无需特殊处理
                }
                Err(e) => {
                    // 遇到其他错误直接返回
                    return Err(e);
                }
            }
        }
        // 如果从始至终未发现语法按照普通字符处理
        if !find_syntax {
            markdown.push(Markdown::Text(current_input));
            break;
        }
    }
    Ok(("", markdown))
}

fn syntax(input: &str) -> IResult<&str, Markdown> {
    alt((
        map(escapable, Markdown::Text),
        map(head, |(level, title, id)| Markdown::Head {
            level,
            title,
            id,
        }),
        parse_text,
        map(horizontal_rule, |_| Markdown::HorizontalRule),
        parse_link,
        // 为保证能匹配到code block，须放在 code 上面
        map(code_block, |(lang, code)| Markdown::CodeBlock {
            lang,
            code,
        }),
        map(code, Markdown::Code),
        map(command, Markdown::Command),
        map(new_line, |_| Markdown::NewLine),
    ))
    .parse(input)
}
