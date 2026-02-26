# Hello, Cargo!

> 📖 对应书中章节：Chapter 1.3 - Hello, Cargo!

`cargo` 是 Rust 的**构建系统**和**包管理器**，绝大多数 Rust 项目都用它来管理。

---

## 🎯 为什么用 Cargo？

Cargo 能帮你：

- 📦 **管理依赖**：自动下载、编译第三方库（Rust 中称为 **crate**）
- 🔨 **构建项目**：处理复杂的编译选项
- 🧪 **运行测试**：`cargo test`
- 📤 **发布代码**：发布到 crates.io
- 📚 **生成文档**：`cargo doc`

> 💡 类比理解：Cargo ≈ npm (Node.js) + pip (Python) + make (C/C++) 的结合体

---

## ✅ 检查 Cargo 是否安装

```bash
cargo --version
```

如果显示版本号（如 `cargo 1.90.0`），说明已安装。Cargo 随 Rust 一起安装，无需单独安装。

---

## 🆕 创建项目

使用 `cargo new` 创建新项目：

```bash
cargo new hello_cargo
cd hello_cargo
```

这会创建以下目录结构：

```
hello_cargo/
├── Cargo.toml      # 配置文件
├── src/
│   └── main.rs     # 源代码
└── .gitignore      # Git 忽略文件（自动初始化 Git 仓库）
```

### Cargo.toml 详解

```toml
[package]
name = "hello_cargo"    # 项目名称
version = "0.1.0"       # 版本号
edition = "2024"        # Rust 版本（Edition）

[dependencies]          # 依赖列表（目前为空）
```

- **TOML** 是 Cargo 使用的配置格式（Tom's Obvious, Minimal Language）
- `edition` 指定 Rust 版本（2015/2018/2021/2024），不同版本可能有语法差异

### src/main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo 会自动生成一个 "Hello, world!" 程序！

---

## 📁 Cargo 项目结构约定

```
项目根目录/
├── Cargo.toml          # 配置文件（必须）
├── Cargo.lock          # 依赖版本锁定（自动生成）
├── src/                # 源代码目录
│   ├── main.rs         # 二进制程序入口
│   └── lib.rs          # 库代码入口（可选）
├── tests/              # 集成测试
├── examples/           # 示例代码
└── benches/            # 性能测试
```

> ⚠️ **重要**：Cargo 期望源代码放在 `src/` 目录下，项目根目录放 README、许可证等非代码文件

---

## 🔨 常用 Cargo 命令

### cargo build — 编译项目

```bash
cargo build
```

输出：

```
   Compiling hello_cargo v0.1.0 (/path/to/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.23s
```

生成的可执行文件位于：`target/debug/hello_cargo`

### cargo run — 编译并运行

```bash
cargo run
```

```
   Compiling hello_cargo v0.1.0 (/path/to/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/hello_cargo`
Hello, world!
```

> 💡 如果代码没有改动，`cargo run` 会直接运行已有二进制文件，不会重新编译

### cargo check — 快速检查

```bash
cargo check
```

```
   Checking hello_cargo v0.1.0 (/path/to/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
```

- 只检查代码能否编译，**不生成可执行文件**
- 速度比 `cargo build` 快很多
- 适合在编码过程中频繁使用

### cargo build --release — 发布构建

```bash
cargo build --release
```

- 启用优化，生成的程序运行更快
- 编译时间更长
- 输出位置：`target/release/`（而不是 `target/debug/`）

---

## 📊 命令对比表

| 命令 | 说明 | 输出位置 |
|------|------|----------|
| `cargo build` | 编译（调试模式） | `target/debug/` |
| `cargo build --release` | 编译（发布模式，优化开启） | `target/release/` |
| `cargo run` | 编译并运行 | `target/debug/` |
| `cargo run --release` | 发布模式编译并运行 | `target/release/` |
| `cargo check` | 仅检查，不生成二进制 | 无 |

---

## 🔍 Debug vs Release

| 模式 | 编译时间 | 运行速度 | 二进制大小 | 用途 |
|------|----------|----------|------------|------|
| debug | 快 | 较慢 | 较大 | 开发、调试 |
| release | 慢 | 快 | 较小 | 发布、性能测试 |

> ⚠️ 做性能基准测试时，一定要用 `cargo build --release`！

---

## 📦 Cargo.lock 文件

首次 `cargo build` 会生成 `Cargo.lock`：

- 记录所有依赖的**精确版本号**
- 确保团队所有人使用相同的依赖版本
- **不要手动编辑**，Cargo 会自动管理
- 对于**应用程序**：应该把 `Cargo.lock` 加入版本控制
- 对于**库**：通常不加入版本控制

---

## 🔄 将现有项目转为 Cargo 项目

如果你已经有一个简单的 Rust 文件（如 `main.rs`），想转为 Cargo 项目：

```bash
# 在项目目录下运行
cargo init
```

这会自动创建 `Cargo.toml` 和 `src/` 目录结构。

---

## 🌐 处理现有项目

克隆别人的 Rust 项目后，只需：

```bash
git clone https://example.com/someproject
cd someproject
cargo build
```

Cargo 会自动下载并编译所有依赖。

---

## 📋 本章小结

我们学习了：

| 知识点 | 说明 |
|--------|------|
| `cargo new` | 创建新项目 |
| `cargo build` | 编译项目 |
| `cargo run` | 编译并运行 |
| `cargo check` | 快速检查（不生成二进制） |
| `cargo build --release` | 发布构建（优化） |
| `Cargo.toml` | 项目配置文件 |
| `Cargo.lock` | 依赖版本锁定 |
| `src/` | 源代码目录 |

---

## 🎯 接下来...

第1章完成！你已经：
- ✅ 安装了 Rust
- ✅ 编写了第一个程序
- ✅ 学会了使用 Cargo

现在可以选择：
- **第2章**：编写一个猜数字游戏（综合实践）
- **第3章**：学习 Rust 的基础语法（变量、函数、控制流）

> 💡 建议：先看第3章学基础，再看第2章实践
