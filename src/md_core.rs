#[derive(Debug, PartialEq, Eq)]
pub enum Markdown<'a> {
    Config(Option<&'a str>),
    Text(&'a str),
    Head {
        level: usize,
        title: &'a str,
        id: &'a str,
    },
    /// # 粗斜体
    /// ```markdown
    /// ***bold italic***
    /// ```
    BoldItalic(Vec<Markdown<'a>>),
    /// # 粗体
    /// ```markdown
    /// **blod**
    /// ```
    Bold(Vec<Markdown<'a>>),
    /// # 斜体
    /// ```markdown
    /// *italic*
    /// __italic__
    /// ```
    Italic(Vec<Markdown<'a>>),
    /// # 删除线
    /// ```markdown
    /// ~~strike~~
    /// ```
    Strike(Vec<Markdown<'a>>),
    /// # 高亮
    /// ```markdown
    /// ==highlight==
    /// ^^highlight^^
    /// ```
    Highlight(Vec<Markdown<'a>>),
    /// # 引语
    /// ```markdown
    /// > quote1
    /// >> quote2
    /// ```
    BlockQuote(Vec<Markdown<'a>>),
    /// # 有序列表
    /// ```markdown
    /// 1. list
    /// 2. list
    /// ```
    OrderedList(Vec<Markdown<'a>>),
    /// # 无序列表
    /// ```markdown
    /// - list
    /// + list
    /// * list
    /// ```
    UnorderedList(Vec<Markdown<'a>>),
    /// # 任务列表
    /// ``` markdown
    /// -[] task1
    /// -[x] task2
    /// ```
    TaskList {
        content: Vec<Markdown<'a>>,
        finish: bool,
    },
    /// # 连续回车只解析成一个换行
    NewLine,
    /// # 分割线
    /// ```markdown
    /// ---
    /// ```
    HorizontalRule,
    /// # 原始链接
    /// ```markdown
    /// https://example.com/
    /// ```
    RawLink(&'a str),
    /// #外部链接
    /// ```markdown
    /// [title](url)
    /// ```
    UrlLink { title: &'a str, url: &'a str },
    ///# 图片链接
    /// ```markdown
    /// ![alt](url)
    /// ```
    Image { alt: &'a str, url: &'a str },
    /// # 行内代码
    /// ```markdown
    /// `code`
    /// ```
    Code(&'a str),
    /// 代码块
    /// ```markdown
    /// \```text
    /// code
    /// \```
    /// ```
    CodeBlock { lang: &'a str, code: &'a str },
    /// # 表格
    Table(&'a str),
    /// # 直接插入的原始 html 标签
    HTML(&'a str),
    /// # 拓展语法，由双层大括号包裹
    Command(&'a str),
}
