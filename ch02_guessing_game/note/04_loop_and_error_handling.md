# 循环与错误处理

> 📖 对应书中章节：Chapter 2 - Allowing Multiple Guesses with Looping

本节学习如何使用 `loop` 实现循环，以及更优雅的错误处理方式。

---

## 🔁 问题：只能猜一次

当前代码只能让用户猜一次就结束了。我们需要：

1. 让用户反复猜测，直到猜对
2. 猜对后退出程序
3. 优雅处理非法输入（如输入字母）

---

## 🔄 loop：无限循环

### 基本语法

```rust
loop {
    println!("again!");  // 无限打印
}
```

`loop` 会无限执行，直到遇到 `break`。

---

### 添加循环

```rust
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

### 问题

现在程序会永远循环，即使猜对了也不会退出！

---

## 🚪 break：退出循环

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;  // 退出循环
    }
}
```

---

## ⚠️ 问题：非法输入导致崩溃

如果用户输入字母：

```
Please input your guess.
abc

thread 'main' panicked at 'Please type a number!'
```

程序崩溃了！因为 `parse()` 失败时 `expect()` 会 panic。

---

## 🛡️ 优雅处理错误

### 用 match 替代 expect

```rust
// 之前：失败就崩溃
let guess: u32 = guess.trim().parse().expect("Please type a number!");

// 现在：失败则继续循环
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

### 解析

```rust
match guess.trim().parse() {
    Ok(num) => num,      // 成功：提取数字
    Err(_) => continue,  // 失败：跳过本次循环，重新输入
}
```

| 模式 | 含义 |
|------|------|
| `Ok(num)` | 成功，`num` 是解析出的数字 |
| `Err(_)` | 失败，`_` 是通配符，忽略错误详情 |
| `continue` | 跳到下一次循环迭代 |

---

## ✅ 完整代码

```rust
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 优雅处理解析错误
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
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
Please input your guess.
50
You guessed: 50
Too big!
Please input your guess.
25
You guessed: 25
Too small!
Please input your guess.
abc            # 非法输入，被忽略
Please input your guess.
37
You guessed: 37
You win!       # 猜对了，程序退出
```

---

## 🎮 最终版本

移除调试用的秘密数字打印：

```rust
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

---

## 📝 小结

| 概念 | 说明 |
|------|------|
| `loop` | 无限循环 |
| `break` | 退出循环 |
| `continue` | 跳过本次迭代，进入下一次 |
| `match` | 模式匹配，可处理 `Result` |
| `Ok(value)` | 匹配成功结果，绑定值 |
| `Err(_)` | 匹配错误，`_` 忽略详情 |

---

## 🎉 恭喜！

你已经完成了猜数字游戏！在这个过程中学到了：

- ✅ `let`、`mut`、变量
- ✅ `match` 表达式
- ✅ 关联函数和方法
- ✅ 外部 crate（`rand`）
- ✅ `Result` 错误处理
- ✅ `loop`、`break`、`continue`
- ✅ 变量遮蔽

---

## 🔗 下一章预告

第3章将深入学习 Rust 的基础概念：
- 变量和可变性
- 数据类型
- 函数
- 控制流
