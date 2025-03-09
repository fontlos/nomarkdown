use nom::{IResult, Parser};

use super::Markdown;
use super::utils::fenced;

// 行内数学公式
// 只要能检测到定界符就匹配掉, 但如果匹配中心的文字两边有空白字符,
// 那么不作为数学公式被匹配, 而是作为普通文字
pub fn math(input: &str) -> IResult<&str, Markdown> {
    let (remaining, output) = fenced("$", "$").parse(input)?;
    if output.starts_with(|c: char| c.is_whitespace())
        || output.ends_with(|c: char| c.is_whitespace())
    {
        // 加 2 是为了带上开头结尾的定界符
        return Ok((remaining, Markdown::Text(&input[..output.len() + 2])));
    }
    Ok((remaining, Markdown::Math(output)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (_, md) = math("$x^2$").unwrap();
        assert_eq!(md, Markdown::Math("x^2"));
        let (_, md) = math("$ x^2 $").unwrap();
        assert_eq!(md, Markdown::Text("$ x^2 $"));
    }
}
