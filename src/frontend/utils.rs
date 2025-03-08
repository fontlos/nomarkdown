use nom::{IResult, Parser, error::Error};
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    combinator::{map, map_parser},
    multi::many1,
};

/// 生成处理被特定模式包裹的字符串的函数
pub fn fenced<'a>(
    start: &'a str,
    end: &'a str,
) -> impl Parser<&'a str, Output = &'a str, Error = Error<&'a str>> {
    map((tag(start), take_until(end), tag(end)), |x| x.1)
}

/// 转义字符表<br>
/// 出于技术问题以及为了使 markdown 语法使用时更加规范方便兼容<br>
/// 以下字符如需出现在正文中必须进行转义处理<br>
/// \ ` * > # + - . |<br>
/// 以下字符如需出现在正文中建议进行转义处理<br>
/// <<br>
/// 以下字符很少遇到需要转义的情况，故未实现相关功能，但仍可以通过插入 \，破坏语法来实现类似功能<br>
/// _ { } [ ] ( ) ! ~<br>
/// 如 \\\[\\\[]] 可以避免被识别为 [[]]，!\\\[\]\(\) 可以避免被识别为图像，但这种情况极其少见<br>
pub fn escapable(input: &str) -> IResult<&str, &str> {
    map_parser(
        map((tag("\\"), take(1u8)), |x| x.1),
        alt((
            tag("\\"),
            tag("`"),
            tag("*"),
            tag("<"),
            tag(">"),
            tag("#"),
            tag("+"),
            tag("-"),
            tag("."),
            tag("|"),
        )),
    )
    .parse(input)
}

pub fn new_line(input: &str) -> IResult<&str, Vec<&str>> {
    alt((many1(tag("\r\n")), many1(tag("\n")))).parse(input)
}

pub fn horizontal_rule(input: &str) -> IResult<&str, &str> {
    tag("\n---\n").parse(input)
}

pub fn command(input: &str) -> IResult<&str, &str> {
    fenced("{{", "}}").parse(input)
}
