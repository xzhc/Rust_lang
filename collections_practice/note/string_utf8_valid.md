# 为什么所有这些 String 值都是"有效"的？

## Code

```rust
// 所有这些都是有效的 String 值！
let hello = String::from("السلام عليكم");  // 阿拉伯语
let hello = String::from("Dobrý den");       // 捷克语
let hello = String::from("Hello");           // 英语
let hello = String::from("שלום");            // 希伯来语
let hello = String::from("नमस्ते");           // 印地语
let hello = String::from("こんにちは");       // 日语
let hello = String::from("안녕하세요");       // 韩语
let hello = String::from("你好");            // 中文
```

## Problem

注释里的"有效"是什么含义？

## Answer

这里的**"有效"（valid）**指的是：Rust 的 `String` 类型内部使用 **UTF-8 编码**，而 UTF-8 是一种支持**所有 Unicode 字符**的编码方式。

所以这段代码想表达的意思是：

> 不管你用的是哪种语言——阿拉伯语、中文、日语、韩语等等——只要是合法的 Unicode 文本，都可以存储在 Rust 的 `String` 中，全部都是**有效的**（不会报错、不会乱码）。

### 对比其他语言

在一些编程语言中，字符串可能默认使用 ASCII 或其他单字节编码，对非英语字符支持不好，可能出现：
- 乱码
- 截断
- 运行时错误

而 Rust 的 `String` **从设计上就强制要求 UTF-8 编码**，所以天然支持全世界所有语言的文字。这就是"有效"的含义——**这些多语言字符串在 Rust 中都是合法的、被正确支持的 `String` 值**。

### 简单总结

| 概念 | 含义 |
|---|---|
| `String` 编码 | UTF-8 |
| UTF-8 支持 | 所有 Unicode 字符 |
| "有效" | 这些不同语言的字符串都能被正确创建、存储和使用，不会出错 |
