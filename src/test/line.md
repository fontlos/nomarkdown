对于文字变体可以通过空格破坏语法, 比如 * 这不是斜体, **但内嵌粗体** *, 可以内嵌所有行内元素
***粗斜体*** ___另一种粗斜体___
**粗体** __另一种粗体__
*斜体* _另一种斜体_
~~删除线~~
==高亮== ^^另一种高亮^^
**文字变体可以嵌套, 例如这是粗体中的 *斜体*,~~删除线~~**

` code, 无法通过空格破坏语法结构, 内部只能是字符, 但可以被嵌入到文字变体中 `

$math\displaystyle\frac{1}{2}$

https://www.原始链接.com
![图片](http)
[链接](http)
[![链接可以内嵌图片]()]()
[[但图片](不可以内嵌链接)]()
[**可以内嵌文字变体**, `code`, $math$]()
[但理论上原始链接的优先级应该高于文字变体, 比如 VSCode无法正确解析这个 http://hello.com/__123__](http://hello.com/__123__)
**[可以被嵌入到文字变体中]()**
[链接文字
可以多行
只要不空行]()

单个斜杠换行\
标签换行<br>



我在实际使用组合子的时候

pub fn text_parser(input: &str) -> IResult<&str, Markdown> {
    alt((bold_italic, bold, italic, strike, highlight)).parse(input)
}

已经按照这种优先级排序了, 问题就出在逆优先级的情况, 我们可以默认***这种粗斜体内不可能嵌套任何粗体斜体, 即使嵌套我们也拒绝按照嵌套来解析. 所以问题主要出在粗体和斜体的嵌套

vscode也没有好的方案, 那就

总之匹配*就跳过**, 找不到就匹配失败, 找到***就匹配后面的*
匹配**就跳过*, 找不到就匹配失败, 找到***就匹配后面的**