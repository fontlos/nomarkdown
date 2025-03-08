mod code;
mod head;
mod link;
mod list;
mod parse;
mod text;
mod utils;

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
    /// # 分割线
    /// ```Markdown
    /// ---
    /// ```
    HorizontalRule,
    /// # 原始链接
    /// ```Markdown
    /// https://example.com/
    /// ```
    RawLink(&'a str),
    /// #外部链接
    /// ```Markdown
    /// [title](url)
    /// ```
    UrlLink {
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
    /// # 表格
    Table(&'a str),
    /// # 直接插入的原始 html 标签
    HTML(&'a str),
    /// # 拓展语法，由双层大括号包裹
    Command(&'a str),
}
