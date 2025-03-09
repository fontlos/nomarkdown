mod code;
mod math;
mod parse;
mod text;
mod utils;

pub use parse::line_element_parser;

#[derive(Debug, PartialEq, Eq)]
pub enum Markdown<'a> {
    Config(Option<&'a str>),
    /// # 普通文字
    /// 注: HTML 标签默认作为普通文字不做处理,
    /// 如果其内部文字干扰解析会导致解析出奇怪的东西
    Text(&'a str),
    Head {
        level: usize,
        title: &'a str,
        id: &'a str,
    },
    /// # 粗斜体
    /// ```Markdown
    /// ***bold italic***
    /// ```
    BoldItalic(Vec<Markdown<'a>>),
    /// # 粗体
    /// ```Markdown
    /// **blod**
    /// ```
    Bold(Vec<Markdown<'a>>),
    /// # 斜体
    /// ```Markdown
    /// *italic*
    /// __italic__
    /// ```
    Italic(Vec<Markdown<'a>>),
    /// # 删除线
    /// ```Markdown
    /// ~~strike~~
    /// ```
    Strike(Vec<Markdown<'a>>),
    /// # 高亮
    /// ```Markdown
    /// ==highlight==
    /// ^^highlight^^
    /// ```
    Highlight(Vec<Markdown<'a>>),
    /// # 引语
    /// ```Markdown
    /// > quote1
    /// >> quote2
    /// ```
    BlockQuote(Vec<Markdown<'a>>),
    /// # 有序列表
    /// ```Markdown
    /// 1. list
    /// 2. list
    /// ```
    OrderedList(Vec<Markdown<'a>>),
    /// # 无序列表
    /// ```Markdown
    /// - list
    /// + list
    /// * list
    /// ```
    UnorderedList(Vec<Markdown<'a>>),
    /// # 任务列表
    /// ``` Markdown
    /// -[] task1
    /// -[x] task2
    /// ```
    TaskList {
        content: Vec<Markdown<'a>>,
        finish: bool,
    },
    /// # 连续回车只解析成一个换行
    NewLine,
    /// # 分割线, 三个以上
    /// ```Markdown
    /// ---
    /// ```
    DividingLine,
    /// # 原始链接, 链接自己作为标题
    /// ```Markdown
    /// https://example.com/
    /// ```
    ///
    /// #外部链接
    /// ```Markdown
    /// [title](url)
    /// ```
    Link {
        title: &'a str,
        url: &'a str,
    },
    ///# 图片链接
    /// ```Markdown
    /// ![alt](url)
    /// ```
    Image {
        alt: &'a str,
        url: &'a str,
    },
    /// # 行内代码
    /// ```Markdown
    /// `code`
    /// ```
    Code(&'a str),
    /// 代码块
    /// ```Markdown
    /// \```text
    /// code
    /// \```
    /// ```
    CodeBlock {
        lang: &'a str,
        code: &'a str,
    },
    /// # 行内数学公式
    /// ```Markdown
    /// $math$
    /// ```
    Math(&'a str),
    /// # 数学块
    /// ```Markdown
    /// $$math$$
    /// ```
    MathBlock(&'a str),
    /// # 表格
    Table(&'a str),
    /// # 拓展语法，由双层大括号包裹
    Command(&'a str),
}
