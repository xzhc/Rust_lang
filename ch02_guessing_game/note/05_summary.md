# 第2章总结

> 📖 对应书中章节：Chapter 2 - Summary

---

## 🎯 本章概览

通过「猜数字游戏」项目，我们实践了以下 Rust 核心概念：

| 概念 | 代码示例 |
|------|----------|
| 变量 | `let mut guess = String::new();` |
| 关联函数 | `String::new()` |
| 方法调用 | `.read_line()`, `.trim()`, `.parse()` |
| 外部 crate | `rand = "0.8.5"` |
| Result 处理 | `.expect()`, `match Ok/Err` |
| 模式匹配 | `match ordering { ... }` |
| 循环控制 | `loop`, `break`, `continue` |
| 变量遮蔽 | `let guess: u32 = ...` |
| 引用 | `&mut guess` |

---

## 📋 完整代码

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

## 🔑 关键命令速查

| 命令 | 作用 |
|------|------|
| `cargo new name --bin` | 创建二进制项目 |
| `cargo build` | 编译项目 |
| `cargo run` | 编译并运行 |
| `cargo update` | 更新依赖 |
| `cargo doc --open` | 打开本地文档 |

---

## 📚 后续章节预告

| 章节 | 主题 | 重要性 |
|------|------|--------|
| 第3章 | 变量、数据类型、函数 | ⭐⭐⭐ 基础 |
| **第4章** | **所有权** | ⭐⭐⭐⭐⭐ **核心** |
| 第5章 | 结构体 | ⭐⭐⭐ |
| 第6章 | 枚举与模式匹配 | ⭐⭐⭐⭐ |

> 💡 **强烈建议**：按顺序学习，第4章（所有权）是 Rust 最核心的概念！

---

## 🎮 扩展练习

尝试给游戏添加新功能：

1. **限制猜测次数**：最多猜 10 次
2. **难度选择**：让用户选择范围（1-50 / 1-100 / 1-1000）
3. **计分系统**：记录用了多少次猜中
4. **重新开始**：猜对后询问是否再玩一局

---

## 📝 笔记文件清单

| 文件 | 内容 |
|------|------|
| `01_setup_and_input.md` | 项目设置、变量、用户输入 |
| `02_random_number.md` | 外部 crate、随机数生成 |
| `03_compare_guess.md` | 类型转换、比较、match |
| `04_loop_and_error_handling.md` | 循环、错误处理 |
| `05_summary.md` | 本章总结（本文件） |

---

**🎉 恭喜完成第2章学习！**
