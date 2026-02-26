# Rust 学习笔记生成指南

## 🎯 核心工作流程

AI 阅读《The Rust Programming Language》的每一章节，然后生成**概念讲解 + 代码演示交替穿插**的 Markdown 笔记。用户根据笔记自行实操学习。

### 每个章节的工作步骤

1. **新建 Cargo 项目**
   - 每开始一个新章节，新建一个对应的 Cargo 项目
   - 项目名格式：`ch{章节号}_{英文简称}`（如 `ch11_testing`、`ch12_io_project`）
   - ⚠ **注意**：不同章节可能使用不同的 `cargo new` 方式（`--lib` / `--bin` / workspace等），**必须在笔记中说明创建方式和原因**

2. **创建 `note/` 文件夹**
   - 位置：与项目的 `src/` 文件夹**同级**
   - 结构示例：
     ```
     ch11_testing/
     ├── Cargo.toml
     ├── src/
     │   └── lib.rs (或 main.rs)
     ├── note/               ← 笔记在这里
     │   ├── 01_xxx.md
     │   ├── 02_xxx.md
     │   └── ...
     └── tests/              ← 如果需要
     ```

3. **编写笔记**（核心输出）
   - 每个小节一个 `.md` 文件，按序号命名：`01_xxx.md`、`02_xxx.md`
   - 笔记格式：**概念讲解 → 代码演示 → 概念讲解 → 代码演示**，交替穿插
   - **所有代码演示都写在笔记的 Markdown 代码块中**，不写到 `src/` 里
   - 代码块中使用 Rust 语法高亮，代码应完整、可运行
   - 笔记开头说明该小节对应书中的哪个章节
   - 用通俗易懂的语言，面向初学者

4. **`src/` 留给用户**
   - AI **不写** `src/` 里的代码
   - 用户阅读笔记后，自己在 `src/` 中编写代码来实操学习
   - `src/` 中只保留 `cargo new` 生成的默认内容

## 📚 学习资料

- **主要教材**：《The Rust Programming Language》官方书籍
- **本地路径**：`file:///Users/xzh/.rustup/toolchains/stable-aarch64-apple-darwin/share/doc/rust/html/book/`
- **命令行查看**：`rustup doc --book`
- **在线版本**：https://doc.rust-lang.org/stable/book/
- **Rust 版本**：1.90.0+ (2024 Edition)

## 📝 笔记编写规范

### 格式要求
- 使用 emoji 作为小节标题前缀，增强可读性
- 概念部分用文字讲解，紧接着用代码块演示
- 用表格做速查/对比总结
- 用 `> 💡` 引用块标注小贴士
- 用 `> ⚠` 引用块标注易错点/注意事项
- 列出关键命令（如 `cargo test`、`cargo run` 等的变体用法）

### 内容要求
- 解释"为什么"而不仅仅是"怎么做"
- 用类比帮助理解抽象概念
- 指出常见陷阱和易混淆的地方
- 代码示例要完整、可运行
- **不同章节的项目创建方式如果不同，务必记录差异**

## 💡 常用命令

```bash
# 查看官方书籍
rustup doc --book

# 创建二进制项目（默认，有 main.rs）
cargo new project_name

# 创建库项目（有 lib.rs，无 main.rs）
cargo new project_name --lib

# 构建 / 运行 / 测试 / 检查
cargo build
cargo run
cargo test
cargo check
```

---

**最后更新**：2026-02-26
