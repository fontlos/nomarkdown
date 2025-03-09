use nom::{Parser, error::Error};
use nom::{
    bytes::complete::{tag, take_until},
    sequence::delimited,
};

/// 生成处理被特定模式包裹的字符串的函数
pub fn fenced<'a>(
    boundary: &'a str,
) -> impl Parser<&'a str, Output = &'a str, Error = Error<&'a str>> {
    delimited(tag(boundary), take_until(boundary), tag(boundary))
}
