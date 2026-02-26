# Hello, World!

> 📖 对应书中章节：Chapter 1.2 - Hello, World!

安装好 Rust 后，让我们编写第一个程序——打印 "Hello, world!"。这是学习新语言的传统第一步！

---

## 📁 创建项目目录

首先，创建一个存放代码的目录：

```bash
# Linux / macOS / PowerShell
mkdir ~/projects
cd ~/projects
mkdir hello_world
cd hello_world
```

```cmd
# Windows CMD
mkdir "%USERPROFILE%\projects"
cd /d "%USERPROFILE%\projects"
mkdir hello_world
cd hello_world
```

---

## 📝 编写代码

在 `hello_world` 目录下创建文件 `main.rs`（注意扩展名是 `.rs`）：

```rust
fn main() {
    println!("Hello, world!");
}
```

> ⚠️ Rust 文件命名惯例：使用**下划线**分隔单词，如 `hello_world.rs`，而不是 `helloworld.rs` 或 `hello-world.rs`

---

## 🔍 代码解析

让我们逐行分析这个程序：

```rust
fn main() {
    println!("Hello, world!");
}
```

### `fn main()`

```rust
fn main() {
    // 函数体
}
```

- `fn` 是 **function** 的缩写，用于声明函数
- `main` 是**特殊函数名**——每个 Rust 可执行程序都从 `main` 函数开始执行
- `()` 表示这个函数没有参数
- `{}` 包裹函数体（Rust 要求所有函数体都用花括号）

### `println!("Hello, world!");`

```rust
println!("Hello, world!");
```

这里有三个关键点：

| 部分 | 说明 |
|------|------|
| `println!` | 这是一个**宏**（macro），不是函数。`!` 是宏的标志 |
| `"Hello, world!"` | 字符串参数，会被打印到终端 |
| `;` | 分号表示语句结束（大多数 Rust 语句以分号结尾） |

> 💡 **宏 vs 函数**：如果 `println` 是函数，应该写成 `println(...)`。加了 `!` 表示它是宏。宏可以生成代码，比函数更灵活。我们会在第20章深入学习宏。

---

## 🔨 编译和运行

Rust 是**预编译语言**(ahead-of-time compiled)，需要先编译再运行：

### 步骤 1：编译

```bash
rustc main.rs
```

### 步骤 2：运行

```bash
# Linux / macOS
./main

# Windows PowerShell
.\main.exe

# Windows CMD
main.exe
```

输出：

```
Hello, world!
```

### 查看生成的文件

```bash
# Linux / macOS
ls
main  main.rs

# Windows
dir
main.exe  main.pdb  main.rs
```

编译后会生成：
- **Linux/macOS**：`main`（可执行文件）
- **Windows**：`main.exe`（可执行文件）+ `main.pdb`（调试信息）

---

## 🔄 编译 vs 解释

| 语言类型 | 代表语言 | 执行方式 |
|----------|----------|----------|
| 编译型 | C, C++, Rust, Go | 先编译成二进制，再运行 |
| 解释型 | Python, Ruby, JavaScript | 逐行解释执行 |
| 混合型 | Java, C# | 编译成字节码，再由虚拟机执行 |

Rust 编译的优势：
- 生成独立的可执行文件
- 接收者**不需要**安装 Rust 就能运行
- 编译时进行大量优化和检查

---

## 🆚 rustc vs Cargo

刚才我们用 `rustc` 直接编译，这适合单文件的简单程序。但实际开发中，我们使用 **Cargo**——Rust 的构建系统和包管理器。

| 特性 | rustc | Cargo |
|------|-------|-------|
| 适用场景 | 单文件简单程序 | 实际项目开发 |
| 依赖管理 | 无 | 自动下载和管理依赖 |
| 构建配置 | 手动 | 使用 `Cargo.toml` |
| 测试支持 | 无 | 内置 `cargo test` |

---

## 📋 小结

我们学到了：

1. Rust 文件使用 `.rs` 扩展名，命名用下划线分隔
2. `main` 函数是程序的入口点
3. `println!` 是宏（注意 `!`），用于打印输出
4. 用 `rustc` 编译，生成可执行文件
5. Rust 是预编译语言，编译后可独立运行

---

## 🎯 下一步

`rustc` 适合学习基础，但实际开发使用 **Cargo** 更方便 → [03_hello_cargo.md](./03_hello_cargo.md)
