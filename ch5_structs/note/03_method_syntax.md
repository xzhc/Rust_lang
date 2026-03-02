# 方法语法

> 📖 对应书中章节：Chapter 5.3 - Method Syntax

## 🎯 方法 vs 函数

| 特性 | 函数 (Function) | 方法 (Method) |
|------|----------------|---------------|
| 定义位置 | 任何地方 | `impl` 块内 |
| 第一个参数 | 任意 | `self`（表示调用者实例） |
| 调用方式 | `function(arg)` | `instance.method()` |
| 关联性 | 独立 | 与特定类型关联 |

---

## 📦 定义方法

使用 `impl`（implementation）块为结构体定义方法：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法的第一个参数是 &self
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // 方法调用语法
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

### `self` 的几种形式

| 形式 | 含义 | 使用场景 |
|------|------|----------|
| `&self` | 不可变借用 | 只读取数据（最常用） |
| `&mut self` | 可变借用 | 需要修改数据 |
| `self` | 获取所有权 | 转换数据，防止调用者继续使用 |

```rust
impl Rectangle {
    // 不可变借用 - 只读
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 可变借用 - 需要修改
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // 获取所有权 - 转换后原实例不可用
    fn into_square(self) -> Option<Rectangle> {
        if self.width == self.height {
            Some(self)  // self 被移动
        } else {
            None
        }
    }
}
```

---

## 🏷️ 方法名可以和字段名相同

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法名为 width，和字段名相同
    fn width(&self) -> bool {
        self.width > 0  // 返回宽度是否大于 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {  // 调用方法（有括号）
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
        //                                                        ^^^^^^^^^^ 访问字段（无括号）
    }
}
```

> 💡 这种模式常用于 **getter** 方法：字段是私有的，但方法是公开的，从而提供只读访问。

---

## 🔧 自动引用和解引用

在 C/C++ 中：
- `object.method()` - 直接调用
- `pointer->method()` - 指针调用（需要先解引用）

**Rust 没有这个区分**，它会自动添加 `&`、`&mut` 或 `*`：

```rust
#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let x_squared = f64::powi(other.x - self.x, 2);
        let y_squared = f64::powi(other.y - self.y, 2);
        f64::sqrt(x_squared + y_squared)
    }
}

fn main() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 6.5 };

    // 这两种写法等价
    p1.distance(&p2);      // Rust 自动添加引用
    (&p1).distance(&p2);   // 手动添加引用
}
```

> 💡 Rust 能够自动判断方法是需要 `&self`、`&mut self` 还是 `self`。

---

## 📐 带更多参数的方法

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法可以有额外参数
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));  // true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));  // false
}
```

---

## 🏗️ 关联函数（Associated Functions）

`impl` 块中**不以 `self` 为参数**的函数叫**关联函数**。

常用于**构造器**（constructor）：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数 - 不需要实例，用 :: 调用
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // 使用 :: 语法调用关联函数
    let sq = Rectangle::square(3);

    println!("{:?}", sq);  // Rectangle { width: 3, height: 3 }
}
```

### `Self` 关键字

在 `impl` 块中，`Self` 是当前类型的别名：

```rust
impl Rectangle {
    fn square(size: u32) -> Self {   // Self = Rectangle
        Self {                        // Self = Rectangle
            width: size,
            height: size,
        }
    }
}
```

> 💡 `new` 不是 Rust 的关键字，只是一个常用的命名约定。你可以用 `square`、`from`、`default` 等任何名字。

---

## 📚 多个 `impl` 块

Rust 允许为同一个类型定义多个 `impl` 块：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 第一个 impl 块
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 第二个 impl 块 - 完全合法！
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

> 💡 通常没必要分开写，但在第 10 章学习 trait 时会看到这种写法的用途。

---

## 📋 方法 vs 关联函数 速查表

| 类型 | 第一个参数 | 调用方式 | 用途 |
|------|-----------|----------|------|
| 方法 | `self` / `&self` / `&mut self` | `instance.method()` | 操作实例 |
| 关联函数 | 无 `self` | `Type::function()` | 构造器、工具函数 |

---

## 📊 完整示例

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数：构造器
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // 关联函数：正方形
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // 方法：计算面积
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法：能否容纳另一个矩形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 方法：缩放（需要修改 self）
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    // 使用关联函数创建实例
    let rect1 = Rectangle::new(30, 50);
    let square = Rectangle::square(10);

    println!("rect1 area: {}", rect1.area());
    println!("square area: {}", square.area());
    println!("rect1 can hold square? {}", rect1.can_hold(&square));

    // 使用可变方法
    let mut rect2 = Rectangle::new(5, 10);
    rect2.scale(2);
    println!("rect2 after scaling: {:?}", rect2);
}
```

---

## 🔨 动手试试

1. 为 `Rectangle` 添加一个方法 `perimeter()` 计算周长
2. 添加一个关联函数 `from_width_height(width: u32, height: u32)`
3. 添加一个方法 `is_square()` 判断是否是正方形
4. 创建一个 `Circle` 结构体，实现 `area()` 和 `circumference()` 方法
