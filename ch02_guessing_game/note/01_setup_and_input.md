# 项目设置与用户输入

> 📖 对应书中章节：Chapter 2 - Setting Up a New Project & Processing a Guess

本章通过一个「猜数字游戏」项目，带你快速上手 Rust。你将学到变量、函数、外部 crate、错误处理等核心概念。

---

## 🎮 游戏规则

程序生成一个 1-100 的随机整数，玩家输入猜测，程序提示「太大」「太小」或「猜对了」。

---

## 📁 创建项目

```bash
# 第2章是一个可执行程序，所以使用 --bin（默认值）
cargo new ch02_guessing_game --bin
cd ch02_guessing_game
```

> 💡 **项目类型选择**：
> - `--bin`：二进制项目，生成 `src/main.rs`，用于可执行程序
> - `--lib`：库项目，生成 `src/lib.rs`，用于供其他项目引用的代码库
>
> 本章需要可执行程序，所以用 `--bin`。

---

## 📦 项目结构

```
ch02_guessing_game/
├── Cargo.toml      # 项目配置文件
├── src/
│   └── main.rs     # 程序入口
└── note/           # 学习笔记
```

---

## 🚀 第一步：打印和获取输入

### 完整代码

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

---

## 🔍 逐行解析

### 1️⃣ 引入标准库

```rust
use std::io;
```

- `use`：把外部模块引入当前作用域
- `std`：Rust 标准库
- `io`：输入/输出模块

> 💡 Rust 有一个「prelude」（预导入），包含最常用的类型。但 `io` 不在其中，需要手动引入。

---

### 2️⃣ main 函数

```rust
fn main() {
    // ...
}
```

- `fn`：声明函数
- `main`：程序入口点
- `{}`：函数体

---

### 3️⃣ 打印字符串

```rust
println!("Guess the number!");
```

- `println!` 是一个**宏**（macro），不是函数
- `!` 是宏的标志
- 宏比函数更强大，可以在编译时生成代码

---

### 4️⃣ 创建变量

```rust
let mut guess = String::new();
```

这里信息量很大！拆解一下：

| 部分 | 含义 |
|------|------|
| `let` | 声明变量 |
| `mut` | 可变的（mutable） |
| `guess` | 变量名 |
| `=` | 绑定值 |
| `String` | 字符串类型 |
| `::new()` | 关联函数（类似静态方法） |
| `()` | 调用函数 |

#### 变量的可变性

```rust
let apples = 5;        // 不可变（默认）
let mut bananas = 5;   // 可变
bananas = 10;          // OK
// apples = 10;        // 编译错误！
```

> ⚠ Rust 变量**默认不可变**，这是安全性的体现。需要修改时显式加 `mut`。

#### 关联函数

```rust
String::new()  // 创建一个空字符串
```

- `::` 语法用于访问命名空间中的项
- `new()` 是很多类型都有的关联函数，用于创建新实例
- 类似于其他语言的 `new String()` 或 `String()`

---

### 5️⃣ 获取用户输入

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

#### 拆解

| 部分 | 作用 |
|------|------|
| `io::stdin()` | 获取标准输入句柄 |
| `.read_line(&mut guess)` | 读取一行，存入 `guess` |
| `.expect(...)` | 处理可能的错误 |

#### 引用 `&`

```rust
&mut guess  // 可变引用
```

- `&` 表示**引用**（reference），让你访问数据而不获取所有权
- 默认引用也是不可变的，需要 `mut` 才能修改
- 第4章会详细讲解引用和所有权

> 💡 `read_line` 会把用户输入**追加**到字符串末尾，而不是覆盖。

---

### 6️⃣ 处理错误：Result 和 expect

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

`read_line` 返回一个 `Result` 类型：

```rust
enum Result<T, E> {
    Ok(T),   // 成功，包含返回值
    Err(E),  // 失败，包含错误信息
}
```

#### expect 的作用

```rust
// 如果 Result 是 Ok，返回里面的值
// 如果是 Err，程序崩溃并打印消息
.expect("Failed to read line");
```

> ⚠ 如果不加 `expect`，编译器会警告：你忽略了可能发生的错误！

---

### 7️⃣ 打印变量值

```rust
println!("You guessed: {guess}");
```

`{}` 是占位符，有两种写法：

```rust
// 方式1：变量名直接放入花括号
println!("You guessed: {guess}");

// 方式2：空花括号 + 参数列表
let x = 5;
let y = 10;
println!("x = {} and y = {}", x, y);
```

---

## 🏃 运行测试

```bash
cargo run
```

输出示例：

```
   Compiling ch02_guessing_game v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/ch02_guessing_game`
Guess the number!
Please input your guess.
42
You guessed: 42
```

---

## 📝 小结

| 概念 | 说明 |
|------|------|
| `use` | 引入模块 |
| `let` | 声明变量 |
| `mut` | 可变变量 |
| `fn` | 声明函数 |
| `::` | 访问关联函数/模块路径 |
| `&` | 引用 |
| `Result` | 错误处理类型 |
| `expect` | 处理 Result，失败则崩溃 |
| `{}` | 格式化占位符 |

---

## 🔗 下一步

下一节我们将学习如何生成随机数，需要引入外部 crate！
