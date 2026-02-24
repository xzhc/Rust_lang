# Rust Panic 与栈展开（Unwind）笔记

> 适合人群：Rust 初学者，刚接触错误处理章节

---

## 一、什么是 `panic!`？

`panic!` 是 Rust 处理**不可恢复错误**的方式。

当程序遇到它完全无法处理的情况时（比如数组越界、断言失败），就会调用 `panic!`，此时程序会：

1. 打印错误信息（包含文件名和行号）
2. 开始清理工作
3. 退出程序

```rust
panic!("程序崩溃了！");
// 输出：thread 'main' panicked at '程序崩溃了！', src/main.rs:2:5
```

> 💡 **类比**：就像电器过载时跳闸的保险丝——与其带病运行损坏整个系统，不如立刻断电保护全局。

---

## 二、作用域（Scope）是什么？

**作用域**就是一对花括号 `{}` 所包裹的代码区域。变量在作用域内诞生，在作用域结束时被销毁。

```rust
fn main() {
    let x = 5;          // x 在这里诞生

    {                   // 内层作用域开始
        let y = 10;     // y 在这里诞生
        println!("{}", y);
    }                   // ← y 在这里被销毁

    println!("{}", x);  // x 还活着
}                       // ← x 在这里被销毁
```

常见的作用域边界：

| 类型 | 边界位置 |
|------|----------|
| 函数体 | 函数的 `{` 到 `}` |
| `if / else` 块 | 块的 `}` |
| `for / while / loop` | 每次循环迭代结束的 `}` |
| 手动块 | 任意手写的 `{ ... }` |

> 💡 **一句话记住**：变量在哪个 `}` 结束，就在那里被销毁。

---

## 三、`Drop` trait 与自动清理

Rust 提供了 `Drop` trait，允许你在值**离开作用域时**自动执行清理逻辑（比如释放内存、关闭文件、解锁互斥锁等）。

```rust
struct Droppable;

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("我被清理了！");
    }
}

fn main() {
    let _d = Droppable;
    println!("函数即将结束...");
}   // ← _d 在这里离开作用域，drop() 自动被调用
    // 输出："我被清理了！"
```

你**不需要手动调用** `drop()`，Rust 编译器在编译时就确定了每个值的生命周期，在正确的位置自动插入清理调用。

---

## 四、栈展开（Unwind）全过程

当 `panic!` 触发时，Rust 默认执行**栈展开**：

```
panic!("触发 panic")
        ↓
打印错误信息 + 位置
        ↓
沿调用栈逐帧回溯
        ↓
对每个栈帧中存活的值调用 drop()
        ↓
程序退出
```

用之前 demo 的输出来验证这个流程：

```
thread 'main' panicked at panic_demo.rs:50:5:
触发 panic，观察栈展开                          ← ① panic 信息打印

Droppable 正在被清理（栈展开）                  ← ② drop() 在展开时被调用
```

> 关键：**即使程序崩溃，Rust 也保证 `drop()` 被执行，资源不会泄漏。**

---

## 五、如何读懂 Backtrace（调用栈信息）

运行时加上 `RUST_BACKTRACE=1` 环境变量，可以看到完整的调用链：

```bash
RUST_BACKTRACE=1 cargo run
```

输出示例（从下往上读，越下面越早调用）：

```
stack backtrace:
   0: rust_begin_unwind              ← panic 处理入口（Rust 内部）
   1: core::panicking::panic_fmt     ← 格式化 panic 信息（Rust 内部）
   2: ch09::panic_demo::demo_panic_unwind   ← ⭐ 你的代码触发 panic 的位置
             at ./src/panic_demo.rs:50:5
   3: ch09::main                     ← ⭐ main 函数调用了上面的函数
             at ./src/main.rs:34:5
   4: core::ops::function::FnOnce::call_once  ← Rust 运行时入口（内部）
```

> 💡 **阅读技巧**：找到第一个写着你自己代码文件路径的帧（如 `panic_demo.rs`），那就是 panic 的发源地。

---

## 六、Unwind vs Abort —— 两种 panic 策略

| 策略 | 行为 | 适用场景 |
|------|------|----------|
| **Unwind**（默认） | 沿调用栈回溯，逐一调用 `drop()`，然后退出 | 大多数应用程序 |
| **Abort** | 立即终止进程，**不调用** `drop()`，由 OS 回收内存 | 嵌入式、极致精简二进制文件 |

可以在 `Cargo.toml` 中配置：

```toml
[profile.release]
panic = "abort"   # 发布模式下使用 abort 策略（二进制更小）

[profile.dev]
panic = "unwind"  # 开发模式下保持默认 unwind（便于调试）
```

---

## 七、什么时候该用 `panic!`？

| ✅ 应该用 `panic!` | ❌ 不应该用 `panic!` |
|-------------------|---------------------|
| 程序进入了不可能发生的状态 | 用户输入了错误的数据 |
| 外部代码传入了违反约定的参数 | 文件不存在、网络超时等 |
| 测试中断言失败 | 任何可以预见并优雅处理的错误 |
| 原型开发阶段（`unwrap()`） | 生产代码中的可恢复错误 |

> 💡 **经验法则**：如果错误是"调用方的 bug"，用 `panic!`；如果是"可以预料的失败"，用 `Result<T, E>`。

---

## 八、完整演示代码

```rust
struct Droppable;

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Droppable 正在被清理（栈展开）");
    }
}

pub fn demo_panic_unwind() {
    let _d = Droppable;   // _d 在此作用域存活

    panic!("触发 panic，观察栈展开");
    // _d 离开作用域时，drop() 自动被调用（即使是 panic 过程中）
}
```

运行输出：

```
thread 'main' panicked at '触发 panic，观察栈展开', src/panic_demo.rs:X:Y
Droppable 正在被清理（栈展开）
```

---

## 九、总结

```
panic!
  ├── 触发条件：不可恢复的错误
  ├── 默认行为：栈展开（Unwind）
  │     └── 保证所有值的 drop() 被调用
  ├── 可选行为：直接终止（Abort）
  │     └── 不调用 drop()，二进制更小
  └── 调试工具：RUST_BACKTRACE=1 查看调用链
```

Rust 的 `panic!` + 自动 `drop` 机制共同保证了：**即使程序崩溃，也不会留下资源泄漏的烂摊子。** 这正是 Rust 内存安全的重要组成部分——无需 GC，靠编译器静态分析就能做到。
