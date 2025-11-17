# Comprehensive Markdown Features Test

This document tests all GitHub-flavored markdown features.

## Task Lists

- [x] Set up project structure
- [x] Implement live reload
- [x] Add React frontend
- [x] Add syntax highlighting library
- [ ] Add copy button for code blocks
- [ ] Add line numbers toggle

### Nested Task Lists

- [x] Backend features
  - [x] File watcher
  - [x] WebSocket support
  - [x] Image path rewriting
  - [ ] PDF support
- [x] Frontend features
  - [x] File tree navigation
  - [x] Theme switching
  - [x] Resizable sidebar
  - [ ] Search functionality

## Text Formatting

**Bold text** and *italic text* and ***bold italic text***

__Bold with underscores__ and _italic with underscores_

~~Strikethrough text~~

`Inline code` with backticks

## Links and References

[External link](https://github.com)

[Link with title](https://rust-lang.org "The Rust Programming Language")

Autolink: https://www.example.com

Email autolink: test@example.com

[Reference link][ref1]

[Another reference][ref2]

[ref1]: https://developer.mozilla.org
[ref2]: https://www.typescriptlang.org

## Nested Lists

1. First ordered item
2. Second ordered item
   - Nested unordered item
   - Another nested item
     1. Deeply nested ordered
     2. Another deep item
        - Even deeper unordered
        - More depth
3. Third ordered item
   1. Nested ordered item
   2. Another nested ordered
      - Mix unordered here
      - And another

## Mixed Lists

- Unordered parent
  1. Ordered child
  2. Another ordered child
     - Unordered grandchild
     - Another grandchild
       1. Ordered great-grandchild
       2. Another great-grandchild

## Blockquotes

> This is a simple blockquote.

> This is a multi-line blockquote.
> It spans multiple lines.
> And continues here.

> Nested blockquotes:
> > This is nested once
> > > This is nested twice
> > > > And this is nested three times

> **Note:** You can use other markdown inside blockquotes
> - Like lists
> - And `inline code`
>
> ```javascript
> // And even code blocks
> console.log("Hello from blockquote");
> ```

## Horizontal Rules

Three ways to create horizontal rules:

---

***

___

## Emphasis and Escaping

This \*text\* is not italic because asterisks are escaped.

This \\*text\\* shows the backslashes because they're escaped too.

## Inline HTML

<div style="background-color: #f0f0f0; padding: 10px; border-radius: 5px;">
  This is <strong>inline HTML</strong> inside markdown.
  <br>
  It can contain <em>any HTML tags</em>.
</div>

<details>
<summary>Click to expand</summary>

This content is hidden by default.

You can put any markdown here:

- List items
- More items

```javascript
// Even code blocks
console.log("Hidden code");
```

</details>

## Emoji (if supported)

:rocket: :heart: :smile: :+1: :fire: :sparkles:

## Footnotes

Here's a sentence with a footnote[^1].

Another reference to a footnote[^2].

[^1]: This is the first footnote.
[^2]: This is the second footnote with more content.
    It can span multiple lines with proper indentation.

## Definition Lists

Term 1
: Definition 1

Term 2
: Definition 2a
: Definition 2b

## Math Expressions (if supported)

Inline math: $E = mc^2$

Block math:

$$
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
$$

## Complex Table

| Feature | Status | Priority | Assignee | Notes |
|---------|--------|----------|----------|-------|
| Live Reload | ✅ Done | High | @dev1 | Working perfectly |
| Syntax Highlighting | ⏳ In Progress | High | @dev2 | Using Prism.js |
| Mermaid Diagrams | ✅ Done | Medium | @dev1 | All diagram types |
| Dark Mode | ✅ Done | Medium | @dev3 | 5 themes available |
| Search | 📝 Planned | Low | - | Coming soon |

### Table with Code

| Language | Example | Output |
|----------|---------|--------|
| JavaScript | `console.log("Hi")` | `Hi` |
| Python | `print("Hello")` | `Hello` |
| Rust | `println!("Hi")` | `Hi` |

### Aligned Columns

| Left Aligned | Center Aligned | Right Aligned |
|:-------------|:--------------:|--------------:|
| Left | Center | Right |
| Text | More Text | Numbers |
| A | B | 123 |

## Comments

<!-- This is an HTML comment and won't be visible -->

[//]: # (This is a markdown comment)

## Line Breaks

This line ends with two spaces
So this starts on a new line.

This line ends with a backslash\
And this also starts on a new line.

## Combining Everything

> **Project Status: Active** 🚀
>
> ### Current Sprint Goals
>
> - [x] Complete React migration
> - [ ] Add syntax highlighting
>   - [x] Choose library
>   - [ ] Integrate with backend
>   - [ ] Add copy button
>
> ---
>
> **Important Note:**[^3] The code highlighting feature will use [Prism.js](https://prismjs.com/) for its comprehensive language support.
>
> Sample configuration:
>
> ```javascript
> const config = {
>   theme: "tomorrow-night",
>   languages: ["javascript", "python", "rust", "typescript"],
>   plugins: ["line-numbers", "copy-to-clipboard"]
> };
> ```
>
> | Component | Coverage | Tests |
> |-----------|----------|-------|
> | FileTree | 95% | ✅ |
> | Sidebar | 87% | ✅ |
> | App | 92% | ✅ |

[^3]: See issue #42 for more details on the implementation plan.
