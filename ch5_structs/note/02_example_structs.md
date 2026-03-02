# 使用结构体的示例程序

> 📖 对应书中章节：Chapter 5.2 - An Example Program Using Structs

让我们通过一个计算矩形面积的程序，逐步体会为什么需要结构体。

---

## 🔄 逐步重构：从变量到结构体

### 版本 1：使用单独的变量

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

**问题**：`area` 函数的签名只显示两个参数，但看不出它们是相关的（都是矩形的尺寸）。

---

### 版本 2：使用元组

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

**改进**：现在只需要一个参数了。

**问题**：元组没有命名元素，必须用索引 `.0`、`.1` 访问，代码含义不清晰。容易混淆宽度和高度！

---

### 版本 3：使用结构体 ✅

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)  // 传递引用，不获取所有权
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

**改进**：
- 语义清晰：`width` 和 `height` 明确表示矩形的尺寸
- 代码自文档化：不需要记住索引 0 是宽还是高
- 借用而非获取所有权：`main` 仍然可以使用 `rect1`

---

## 🖨️ 打印结构体

### 直接打印会报错

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);  // ❌ 编译错误！
}
```

错误信息：
```
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

> 💡 Rust 不知道该怎么格式化输出自定义结构体。

---

### 使用 Debug trait

**步骤 1**：添加 `#[derive(Debug)]` 属性

```rust
#[derive(Debug)]  // 派生 Debug trait
struct Rectangle {
    width: u32,
    height: u32,
}
```

**步骤 2**：使用 `{:?}` 或 `{:#?}` 格式化

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // 单行格式
    println!("rect1 is {:?}", rect1);
    // 输出: rect1 is Rectangle { width: 30, height: 50 }

    // 多行格式（更易读）
    println!("rect1 is {:#?}", rect1);
    // 输出:
    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50,
    // }
}
```

---

### 使用 `dbg!` 宏

`dbg!` 宏会：
1. 打印**文件名和行号**
2. 打印**表达式的值**
3. **返回**表达式的值（获取所有权）

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),  // 打印并返回值
        height: 50,
    };

    dbg!(&rect1);  // 传递引用，避免移动所有权
}
```

输出：
```
[src/main.rs:10:16] 30 * scale = 60
[src/main.rs:14:5] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

> 💡 `dbg!` 输出到 **stderr**（标准错误流），而 `println!` 输出到 **stdout**（标准输出流）。这在重定向输出时很有用。

---

## 📊 `println!` vs `dbg!` 对比

| 特性 | `println!` | `dbg!` |
|------|-----------|--------|
| 输出流 | stdout | stderr |
| 所有权 | 接收引用 | 获取所有权（返回值） |
| 输出位置信息 | ❌ | ✅ 文件名:行号 |
| 格式化 | 需手动指定 | 自动使用 Debug 格式 |
| 用途 | 给用户看 | 调试用 |

---

## 📋 小结

| 场景 | 推荐方案 |
|------|----------|
| 相关数据需要组合 | 使用结构体 |
| 打印调试信息 | `#[derive(Debug)]` + `{:?}` |
| 美化调试输出 | `{:#?}` |
| 快速调试 | `dbg!` 宏 |

---

## 🔨 动手试试

1. 定义一个 `Rectangle` 结构体并派生 `Debug`
2. 创建几个矩形实例
3. 用 `{:?}` 和 `{:#?}` 打印它们，观察区别
4. 用 `dbg!` 调试一个计算过程
