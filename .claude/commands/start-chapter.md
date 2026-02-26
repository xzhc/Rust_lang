# 开启新章节学习

用户想要开启 Rust 学习的新章节。

## 工作流程

### 1. 确认章节信息

首先确认以下信息：
- 章节号：$ARGUMENTS（用户传入的参数）
- 从 CLAUDE.md 中查找对应的章节名称

如果用户没有提供章节号，或者章节号无效，询问用户要学习哪一章。

### 2. 创建 Cargo 项目

根据章节特性选择项目类型：
- **默认使用 `--lib`**（大多数章节适合库项目）
- 如果章节需要可执行程序（如猜数字游戏、I/O 项目），使用默认的 `--bin`

项目命名格式：`ch{章节号}_{英文简称}`

```bash
cargo new ch{章节号}_{英文简称} --lib  # 或 --bin
```

### 3. 创建笔记目录结构

```
ch{章节号}_{英文简称}/
├── Cargo.toml
├── src/
│   └── lib.rs (或 main.rs)
└── note/               ← 创建这个文件夹
```

### 4. 读取书籍章节并生成笔记

1. 使用 `rustup doc --book` 打开本地书籍，或访问在线版本
2. 阅读对应章节内容
3. 为每个小节生成一个笔记文件（`note/01_xxx.md`、`note/02_xxx.md`...）

笔记格式要求：
- 开头说明对应书中的哪个章节
- **概念讲解 → 代码演示 → 概念讲解 → 代码演示**，交替穿插
- 所有代码写在 Markdown 代码块中，不写到 `src/`
- 使用 emoji 作为小节标题前缀
- 用 `> 💡` 标注小贴士，用 `> ⚠` 标注易错点
- 面向初学者，通俗易懂

### 5. 更新 CLAUDE.md 进度

在 CLAUDE.md 的"已完成章节"或"待学习章节"中标记当前状态。

## 章节信息速查表

| 章节 | 名称 | 项目类型建议 | 英文简称 |
|------|------|--------------|----------|
| 1 | Getting Started | --bin | getting_started |
| 2 | Programming a Guessing Game | --bin | guessing_game |
| 3 | Common Programming Concepts | --lib | common_concepts |
| 4 | Understanding Ownership | --lib | ownership |
| 5 | Structs | --lib | structs |
| 6 | Enums and Pattern Matching | --lib | enums_match |
| 7 | Packages, Crates, and Modules | --lib | modules |
| 8 | Common Collections | --lib | collections |
| 9 | Error Handling | --lib | error_handling |
| 10 | Generic Types, Traits, and Lifetimes | --lib | generics_traits_lifetimes |
| 11 | Testing | --lib | testing |
| 12 | An I/O Project | --bin | io_project |
| 13 | Functional Language Features | --lib | functional_features |
| 14 | More about Cargo and Crates.io | --bin | cargo_crates |
| 15 | Smart Pointers | --lib | smart_pointers |
| 16 | Fearless Concurrency | --lib | concurrency |
| 17 | Async, Await, Futures, and Streams | --lib | async_await |
| 18 | Object Oriented Programming Features | --lib | oop |
| 19 | Patterns and Matching | --lib | patterns |
| 20 | Advanced Features | --lib | advanced |
| 21 | Final Project | --bin | web_server |

## 注意事项

- 第4章（所有权）是重中之重，需要详细讲解
- 第10章（泛型、Trait、生命周期）是中级重要概念
- 第12章是综合项目，需要 --bin
- 确保代码示例完整可运行
