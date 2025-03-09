use nom::{IResult, Parser};

use super::Markdown;
use super::utils::fenced;

// 行内数学公式
// 只要能检测到定界符就匹配掉, 但如果匹配中心的文字两边有空白字符,
// 那么不作为数学公式被匹配, 而是作为普通文字
pub fn math(input: &str) -> IResult<&str, Markdown> {
    fenced("$").map(Markdown::Math).parse(input)
}

#[cfg(feature = "strict")]
// 行内数学公式严格模式
// 只要能检测到定界符就匹配掉, 但如果匹配中心的文字两边有空白字符,
// 那么不作为数学公式被匹配, 而是作为普通文字
pub fn math(input: &str) -> IResult<&str, Markdown> {
    let (remaining, parsed) = fenced("$").parse(input)?;
    if parsed.starts_with(|c: char| c.is_whitespace())
        || parsed.ends_with(|c: char| c.is_whitespace())
    {
        // 加 2 是为了带上开头结尾的定界符
        Ok((remaining, Markdown::Text(&input[..parsed.len() + 2])))
    } else {
        Ok((remaining, Markdown::Math(parsed)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math() {
        let (_, md) = math("$x^2$").unwrap();
        assert_eq!(md, Markdown::Math("x^2"));
        let (_, md) = math("$ x^2 $").unwrap();
        assert_eq!(md, Markdown::Math(" x^2 "));
    }

    #[test]
    #[cfg(feature = "strict")]
    fn test_math_strict() {
        let (_, md) = math("$x^2$").unwrap();
        assert_eq!(md, Markdown::Math("x^2"));
        let (_, md) = math("$ x^2 $").unwrap();
        assert_eq!(md, Markdown::Text("$ x^2 $"));
    }
}
