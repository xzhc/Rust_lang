# 比较猜测与秘密数字

> 📖 对应书中章节：Chapter 2 - Comparing the Guess to the Secret Number

本节学习如何比较两个数字，并引入 `match` 表达式和类型转换。

---

## 🎯 目标

比较用户输入的猜测和随机生成的秘密数字，输出：
- 「太小了！」（Too small!）
- 「太大了！」（Too big!）
- 「你赢了！」（You win!）

---

## 🔢 类型问题

首先看一段**无法编译**的代码：

```rust
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    // ⚠ 这里会编译错误！
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

### 编译错误

```
error[E0308]: mismatched types
  --> src/main.rs:23:21
   |
23 |     match guess.cmp(&secret_number) {
   |                 --- ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`
   |
   = note: expected reference `&String`
              found reference `&{integer}`
```

### 原因分析

| 变量 | 类型 | 来源 |
|------|------|------|
| `guess` | `String` | 用户输入的字符串 |
| `secret_number` | `i32`（默认整数类型） | 随机生成的数字 |

**无法比较字符串和数字！** 我们需要把字符串转换成数字。

---

## 🔄 类型转换：String → u32

### 解决方案

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

这行代码做了什么？

```
guess (String)          // 原始输入 "42\n"
    │
    ▼
.trim()                 // 去除首尾空白 → "42"
    │
    ▼
.parse()                // 解析为数字 → Ok(42) 或 Err(...)
    │
    ▼
.expect("...")          // 处理 Result → 42（或崩溃）
    │
    ▼
guess: u32              // 新变量，类型是 u32
```

---

## 🔍 逐个解析

### 1️⃣ trim() - 去除空白

```rust
let input = "  42  \n";
let cleaned = input.trim();  // "42"
```

用户输入后会按回车，字符串末尾会有换行符 `\n`（Windows 是 `\r\n`）。

| 输入 | `.trim()` 后 |
|------|-------------|
| `"42\n"` | `"42"` |
| `"  42  "` | `"42"` |
| `"42\r\n"` | `"42"` |

---

### 2️⃣ parse() - 解析为数字

```rust
let num: u32 = "42".parse().expect("Not a number!");
```

- `parse()` 把字符串解析成指定类型
- 返回 `Result<u32, ParseIntError>`
- 需要指定目标类型（通过类型注解或类型推断）

#### 类型注解

```rust
let guess: u32 = ...;  // 显式注解
```

---

### 3️⃣ 变量遮蔽（Shadowing）

```rust
let mut guess = String::new();  // guess 是 String
// ...
let guess: u32 = ...;           // guess 被遮蔽，现在是 u32
```

> 💡 **变量遮蔽**：用同一个名字声明新变量，旧变量被「遮住」了。这在类型转换时很有用！

---

## 📊 数字类型

| 类型 | 含义 | 范围 |
|------|------|------|
| `i32` | 32位有符号整数 | -2,147,483,648 ~ 2,147,483,647 |
| `u32` | 32位无符号整数 | 0 ~ 4,294,967,295 |
| `i64` | 64位有符号整数 | 非常大 |
| `u64` | 64位无符号整数 | 非常大 |

> 💡 本章使用 `u32`，因为猜测数字是 1-100 的正整数。Rust 会根据上下文推断 `secret_number` 也是 `u32`。

---

## 🎭 match 表达式

```rust
use std::cmp::Ordering;

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

### cmp 方法

```rust
guess.cmp(&secret_number)  // 比较 guess 和 secret_number
```

返回 `Ordering` 枚举：

```rust
enum Ordering {
    Less,    // 小于
    Greater, // 大于
    Equal,   // 等于
}
```

### match 语法

```rust
match 值 {
    模式1 => 代码1,
    模式2 => 代码2,
    模式3 => 代码3,
}
```

- `match` 会依次检查每个**分支**（arm）
- 找到匹配的模式后，执行对应的代码
- **必须覆盖所有可能的情况**（Rust 强制穷尽性）

---

## ✅ 完整代码

```rust
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // 关键：类型转换
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

---

## 🏃 运行测试

```bash
cargo run
```

```
Guess the number!
The secret number is: 58
Please input your guess.
42
You guessed: 42
Too small!
```

```
Guess the number!
The secret number is: 7
Please input your guess.
76
You guessed: 76
Too big!
```

---

## 📝 小结

| 概念 | 说明 |
|------|------|
| `std::cmp::Ordering` | 比较结果枚举（Less/Greater/Equal） |
| `.cmp(&other)` | 比较两个值 |
| `match` | 模式匹配表达式 |
| `.trim()` | 去除字符串首尾空白 |
| `.parse()` | 字符串解析为其他类型 |
| 变量遮蔽 | 用同名变量覆盖旧变量 |
| `u32` | 32位无符号整数 |

---

## 🔗 下一步

下一节我们将添加循环，让用户可以多次猜测！
