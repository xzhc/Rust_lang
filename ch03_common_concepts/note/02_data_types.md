# 数据类型

> 📖 对应书中章节：Chapter 3.2 - Data Types

Rust 是**静态类型语言**（statically typed），编译器必须在编译时知道所有变量的类型。

```rust
// 通常类型可以自动推断
let x = 5;  // 编译器推断为 i32

// 但有时需要显式标注
let guess: u32 = "42".parse().expect("不是数字！");
```

Rust 的数据类型分为两大类：**标量类型**（Scalar）和**复合类型**（Compound）。

## 🔢 标量类型（Scalar Types）

标量类型表示单个值。Rust 有四种基本标量类型：

### 1️⃣ 整数类型

整数是没有小数部分的数字。

| 长度 | 有符号 | 无符号 |
|------|--------|--------|
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| 架构相关 | `isize` | `usize` |

**有符号 vs 无符号：**
- **有符号**（signed）：可以存储负数，范围是 -2^(n-1) 到 2^(n-1)-1
- **无符号**（unsigned）：只能存储非负数，范围是 0 到 2^n-1

```rust
fn main() {
    let a: i8 = -128;   // i8 范围：-128 到 127
    let b: u8 = 255;    // u8 范围：0 到 255
    let c: i32 = 0;     // i32 是默认整数类型
    let d: usize = 100; // 用于数组索引
}
```

> 💡 **默认类型**：如果不确定用什么，就用 `i32`，它是 Rust 的默认整数类型。

### 整数字面量写法

| 数字字面量 | 示例 |
|-----------|------|
| 十进制 | `98_222` |
| 十六进制 | `0xff` |
| 八进制 | `0o77` |
| 二进制 | `0b1111_0000` |
| 字节（仅 u8） | `b'A'` |

```rust
fn main() {
    let decimal = 98_222;      // 下划线提高可读性
    let hex = 0xff;            // 十六进制：255
    let octal = 0o77;          // 八进制：63
    let binary = 0b1111_0000;  // 二进制：240
    let byte = b'A';           // 字节：65
}
```

### ⚠ 整数溢出

当值超出类型范围时会发生整数溢出：

```rust
fn main() {
    let mut x: u8 = 255;
    x = x + 1;  // 溢出！
}
```

- **Debug 模式**：程序会 **panic**（崩溃）
- **Release 模式**：会**回绕**（wrap around），255 + 1 变成 0

> ⚠ **建议**：不要依赖溢出的回绕行为。如果需要处理溢出，使用标准库提供的方法：
> - `wrapping_add()` - 回绕
> - `checked_add()` - 返回 `None`
> - `overflowing_add()` - 返回值和溢出标志
> - `saturating_add()` - 饱和在最大/最小值

### 2️⃣ 浮点类型

Rust 有两种浮点类型：`f32`（32位）和 `f64`（64位）。

```rust
fn main() {
    let x = 2.0;      // f64，默认类型
    let y: f32 = 3.0; // f32
}
```

> 💡 **默认类型**：`f64` 是默认浮点类型，因为现代 CPU 上它的速度和 `f32` 差不多，但精度更高。

### 3️⃣ 数值运算

```rust
fn main() {
    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;  // 整数除法，结果为 -1

    // 取余
    let remainder = 43 % 5;
}
```

> ⚠ **注意**：整数除法会**向零截断**。`-5 / 3` 的结果是 `-1`，而不是 `-2`。

### 4️⃣ 布尔类型

布尔类型只有两个值：`true` 和 `false`，占用 1 字节。

```rust
fn main() {
    let t = true;
    let f: bool = false;  // 显式类型标注
}
```

### 5️⃣ 字符类型

`char` 类型是 Rust 最原始的字母类型，占用 4 字节，表示一个 Unicode 标量值。

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    let chinese = '中';
}
```

> 💡 **重要区别**：
> - `char` 用**单引号**，表示单个 Unicode 字符
> - 字符串用**双引号**
> - `char` 可以表示 emoji、中文等任何 Unicode 字符

## 📦 复合类型（Compound Types）

复合类型可以将多个值组合成一个类型。

### 1️⃣ 元组（Tuple）

元组可以存储**不同类型**的值，长度固定。

```rust
fn main() {
    // 创建元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 解构元组
    let (x, y, z) = tup;
    println!("y 的值是: {y}");

    // 通过索引访问
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}
```

### 单元类型（Unit）

没有值的元组 `()` 叫做**单元类型**（unit type），表示空值或空返回。

```rust
fn main() {
    let unit = ();
}
```

### 2️⃣ 数组（Array）

数组存储**相同类型**的值，长度固定。

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    // 显式类型标注：[元素类型; 长度]
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // 重复初始化：[值; 长度]
    let c = [3; 5];  // 等同于 [3, 3, 3, 3, 3]

    // 访问元素
    let first = a[0];
    let second = a[1];
}
```

### 数组 vs 向量（Vector）

| 特性 | 数组 | 向量 |
|------|------|------|
| 长度 | 固定 | 可变 |
| 内存 | 栈 | 堆 |
| 灵活性 | 低 | 高 |
| 性能 | 略优 | 略低 |

> 💡 **选择建议**：如果不确定用哪个，就用 `Vec`。只有当你确定元素数量不会改变时，才用数组。

### ⚠ 数组越界访问

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("请输入数组索引：");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("读取失败");

    let index: usize = index
        .trim()
        .parse()
        .expect("请输入数字");

    let element = a[index];  // 如果越界，会 panic！

    println!("索引 {index} 处的值是: {element}");
}
```

如果输入 `10`，程序会 panic：

```
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
```

> 💡 **内存安全**：Rust 会在运行时检查数组边界，防止非法内存访问。这是 Rust 内存安全保证的一部分。

## 📊 类型速查表

### 标量类型

| 类型 | 描述 | 示例 |
|------|------|------|
| `i8` ~ `i128` | 有符号整数 | `-128`, `127` |
| `u8` ~ `u128` | 无符号整数 | `0`, `255` |
| `isize`, `usize` | 架构相关整数 | 用于索引 |
| `f32`, `f64` | 浮点数 | `3.14` |
| `bool` | 布尔值 | `true`, `false` |
| `char` | Unicode 字符 | `'a'`, `'中'`, `'😻'` |

### 复合类型

| 类型 | 描述 | 示例 |
|------|------|------|
| `(T1, T2, ...)` | 元组 | `(1, "hello", 3.14)` |
| `[T; N]` | 数组 | `[1, 2, 3]` |
| `()` | 单元类型 | `()` |

## 🎯 练习建议

1. 创建各种整数类型的变量，尝试打印它们
2. 尝试让一个 `u8` 变量溢出（Debug 和 Release 模式分别测试）
3. 创建一个包含不同类型的元组，练习解构和索引访问
4. 创建一个数组，尝试访问合法和非法索引
