# Nomarkdown: Markdown parser powered by nom

By default, normal Markdown is parsed, but if the 'strict' feature is enabled, the following changes will be made:

- `* Variants of text that are not immediately adjacent to the delimiter will not be parsed, including italics, bold, etc *`
- `$ Inline math formulas that are not immediately adjacent to the delimiter will not be parsed $`
