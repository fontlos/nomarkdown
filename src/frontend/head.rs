use nom::{IResult, Parser};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::multispace0,
    combinator::{map, opt},
    multi::many1,
};

use super::utils::fenced;

/// 匹配任意等级标题，但大于六级会出现不存在的 html 标签
/// 这不应该属于 bug 而是使用习惯的问题
/// 暂不支持在普通文字下方输入多个 = 或 - 识别成标题的功能
pub fn head(input: &str) -> IResult<&str, (usize, &str, &str)> {
    // 一个最普通的标题，其上一定有一定数量的换行符，随后是一定数量的 #，随后是一定数量的空格
    // 随后是标题内容以及一个换行符，换行符不匹配掉，以免紧跟其后的新标题 # 前无换行符
    alt((
        map((many1(tag("#")), multispace0, is_not("\r\n")), |x| {
            let level = x.0.len();
            let (title, id) = match head_id(x.2).unwrap().1 {
                (t, Some(i)) => (t, i),
                (t, None) => (t, t),
            };
            (level, title, id)
        }),
        map((many1(tag("#")), multispace0, is_not("\n")), |x| {
            let level = x.0.len();
            let (title, id) = match head_id(x.2).unwrap().1 {
                (t, Some(i)) => (t, i),
                (t, None) => (t, t),
            };
            (level, title, id)
        }),
    ))
    .parse(input)
}

/// 用于从标题内容中分离出自定义 ID
fn head_id(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    map((is_not("{"), opt(fenced("{#", "}"))), |x| (x.0, x.1)).parse(input)
}
