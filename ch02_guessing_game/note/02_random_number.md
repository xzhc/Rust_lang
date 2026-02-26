# 生成随机数

> 📖 对应书中章节：Chapter 2 - Generating a Secret Number

本节学习如何使用外部 crate 生成随机数。

---

## 📦 什么是 Crate？

| 类型 | 说明 | 示例 |
|------|------|------|
| **二进制 crate** | 可执行程序 | 我们的项目 `ch02_guessing_game` |
| **库 crate** | 代码库，供其他项目使用 | `rand` crate |

Rust 标准库没有随机数功能，需要使用社区提供的 `rand` crate。

---

## 🔧 添加依赖

编辑 `Cargo.toml`：

```toml
[package]
name = "ch02_guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.5"
```

> 💡 版本号 `"0.8.5"` 实际上是 `^0.8.5` 的简写，表示「>= 0.8.5 且 < 0.9.0」。

---

## 🏗️ Cargo 构建过程

```bash
cargo build
```

第一次运行时会看到：

```
  Updating crates.io index        # 更新 crates.io 索引
   Locking 15 packages...         # 锁定依赖版本
    Adding rand v0.8.5            # 添加 rand
 Compiling proc-macro2 v1.0.93    # 编译依赖的依赖...
 Compiling rand v0.8.5            # 编译 rand
 Compiling ch02_guessing_game...  # 编译我们的项目
  Finished ...
```

### Cargo 做了什么？

```
crates.io (Rust 包仓库)
     │
     ▼
  下载 rand 及其所有依赖
     │
     ▼
  编译所有依赖
     │
     ▼
  编译我们的项目
```

---

## 🔒 Cargo.lock：可重复构建

Cargo 第一次构建时会创建 `Cargo.lock` 文件：

```toml
# 这个文件由 Cargo 自动管理
# 记录了所有依赖的精确版本

[[package]]
name = "rand"
version = "0.8.5"
...
```

### 为什么需要它？

假设下周 `rand` 发布了 0.8.6：
- **没有 Cargo.lock**：每个人可能用不同版本
- **有 Cargo.lock**：所有人都用相同的 0.8.5

> 💡 这确保了「可重复构建」——在任何时间、任何机器上构建结果一致。

---

## 📤 更新依赖

```bash
# 更新到 Cargo.toml 允许的最新版本
cargo update
```

如果 0.8.6 可用：

```
    Updating rand v0.8.5 -> v0.8.6
```

> ⚠ `cargo update` 只会更新到 `^0.8.5` 允许的版本。想用 0.9.x？需要手动修改 `Cargo.toml`。

---

## 🎲 生成随机数

### 代码

```rust
use std::io;
use rand::Rng;  // 引入 Rng trait

fn main() {
    println!("Guess the number!");

    // 生成 1-100 的随机数
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // 开发时打印答案，方便测试
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

---

## 🔍 代码解析

### 1️⃣ 引入 Rng trait

```rust
use rand::Rng;
```

- `Rng` 是一个 **trait**（特征），定义了随机数生成器的方法
- 必须引入才能使用 `gen_range` 等方法

> 💡 trait 类似其他语言的「接口」，第10章会详细讲解。

---

### 2️⃣ 生成随机数

```rust
let secret_number = rand::thread_rng().gen_range(1..=100);
```

拆解：

| 部分 | 作用 |
|------|------|
| `rand::thread_rng()` | 获取当前线程的随机数生成器 |
| `.gen_range(1..=100)` | 生成 [1, 100] 范围内的随机数 |
| `1..=100` | 闭区间范围（包含 1 和 100） |

#### Range 语法

```rust
1..=100   // 1 到 100，闭区间 [1, 100]
1..100    // 1 到 99，左闭右开 [1, 100)
```

---

## 📚 查看文档

```bash
# 打开所有依赖的本地文档
cargo doc --open
```

这会在浏览器中打开文档，你可以：
- 点击 `rand` 查看其 API
- 了解所有可用的函数和方法

---

## 🏃 运行测试

```bash
cargo run
```

多次运行，验证随机数不同：

```
$ cargo run
Guess the number!
The secret number is: 7
...

$ cargo run
Guess the number!
The secret number is: 83
...
```

---

## 📝 小结

| 概念 | 说明 |
|------|------|
| `Cargo.toml` | 项目配置，声明依赖 |
| `Cargo.lock` | 锁定精确版本，确保可重复构建 |
| `cargo update` | 更新依赖版本 |
| `rand::thread_rng()` | 获取随机数生成器 |
| `gen_range(start..=end)` | 生成闭区间随机数 |
| `cargo doc --open` | 打开本地文档 |

---

## 🔗 下一步

下一节我们将比较用户的猜测和随机数！
