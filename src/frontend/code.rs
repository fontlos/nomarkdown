use nom::{IResult, Parser};

use super::Markdown;
use super::utils::fenced;

// 行内代码
// 优先级很高, 只要能检测到定界符就匹配掉
pub fn code(input: &str) -> IResult<&str, Markdown> {
    fenced("`").map(Markdown::Code).parse(input)
}
