# 切片类型

> 📖 对应书中章节：Chapter 4.3 - The Slice Type

## 🎯 什么是切片？

**切片（Slice）** 让你引用集合中一段连续的元素序列，而不引用整个集合。

切片是一种引用，所以**没有所有权**。

---

## 🤔 为什么需要切片？

假设我们要写一个函数，返回字符串中的第一个单词：

### 方案一：返回索引

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {  // 找到空格
            return i;
        }
    }

    s.len()  // 没找到空格，返回整个字符串长度
}
```

**问题**：返回的索引与字符串是分离的！

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);  // word = 5

    s.clear();  // 字符串被清空了！

    // word 仍然是 5，但 s 已经是空字符串了
    // 如果我们用 word 来索引 s，会出问题！
}
```

### 方案二：使用字符串切片 ✅

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];  // 返回切片
        }
    }

    &s[..]  // 返回整个字符串的切片
}
```

---

## 🔤 字符串切片

字符串切片的类型是 `&str`。

### 基本语法

```rust
let s = String::from("hello world");

let hello = &s[0..5];   // "hello"
let world = &s[6..11];  // "world"
```

### 内存布局

```text
┌─────────────────┐
│ s (String)      │
│  ptr ────────────────→ "hello world" (堆)
│  len  = 11      │            ↑
│  cap  = 11      │            │
└─────────────────┘            │
┌─────────────────┐            │
│ world (切片)    │            │
│  ptr ────────────────────────┘
│  len  = 5       │    指向索引 6，长度 5
└─────────────────┘
```

切片只存储：**起始位置** + **长度**

---

## 📝 切片语法糖

### 从开头开始

```rust
let s = String::from("hello");

// 这两种写法等价
let slice = &s[0..2];
let slice = &s[..2];   // 省略开头的 0
```

### 到结尾结束

```rust
let s = String::from("hello");

// 这两种写法等价
let slice = &s[3..s.len()];
let slice = &s[3..];   // 省略结尾
```

### 整个字符串

```rust
let s = String::from("hello");

// 这三种写法等价
let slice = &s[0..s.len()];
let slice = &s[..];
let slice = &s;        // 直接引用也是切片
```

---

## 🛡️ 切片的安全性

切片与借用遵守相同的规则：

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);  // 不可变借用

    s.clear();  // ❌ 错误！需要可变借用，但已有不可变借用

    println!("the first word is: {}", word);  // word 还在使用
}

fn first_word(s: &String) -> &str {
    // ...
    &s[..]
}
```

错误：
```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
```

Rust 编译器确保切片引用的数据不会被意外修改！

---

## 🔤 字符串字面值就是切片

还记得字符串字面值吗？

```rust
let s = "Hello, world!";
```

这里 `s` 的类型是 `&str`：

- 它是一个切片，指向程序二进制文件中的特定位置
- 这就是为什么字符串字面值是不可变的——`&str` 是不可变引用

---

## 🎯 字符串切片作为参数

### 更好的函数签名

```rust
// 好，但只能接受 &String
fn first_word(s: &String) -> &str { ... }

// 更好！可以接受 &String 和 &str
fn first_word(s: &str) -> &str { ... }
```

使用 `&str` 作为参数，函数更灵活：

```rust
fn main() {
    let my_string = String::from("hello world");

    // 可以传 &String
    let word = first_word(&my_string);

    // 可以传 String 的切片
    let word = first_word(&my_string[0..6]);

    // 可以传整个 String 的引用（自动转换为 &str）
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // 可以传 &str（字符串字面值本身就是 &str）
    let word = first_word(my_string_literal);

    // 可以传字面值的切片
    let word = first_word(&my_string_literal[0..6]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

> 💡 **最佳实践**
> 如果函数只需要读取字符串，使用 `&str` 作为参数类型，而不是 `&String`。

---

## 📦 其他类型的切片

切片不只用于字符串！数组也有切片：

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];  // 类型是 &[i32]

assert_eq!(slice, &[2, 3]);
```

数组切片的类型是 `&[T]`（T 是元素类型）。

---

## 📊 总结对比

| 类型 | 语法 | 用途 |
|------|------|------|
| `String` | `String::from("hello")` | 拥有所有权的堆字符串 |
| `&String` | `&s` | 对 String 的引用 |
| `&str` | `&s[0..5]` 或 `"hello"` | 字符串切片（不可变） |
| `&mut str` | 很少用 | 可变字符串切片 |

---

## 🎯 本章核心概念回顾

### 所有权规则

```text
1. 每个值有一个所有者
2. 同一时刻只能有一个所有者
3. 所有者离开作用域时，值被丢弃
```

### 引用规则

```text
1. 要么一个可变引用，要么任意数量不可变引用
2. 引用必须总是有效的
```

### 切片

- 引用集合的一部分
- 字符串切片：`&str`
- 数组切片：`&[T]`

---

## 📝 综合练习

```rust
fn main() {
    // 1. 所有权
    let s1 = String::from("hello");
    let s2 = s1;  // s1 移动到 s2
    // println!("{}", s1);  // 错误！

    // 2. 克隆
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("{} {}", s3, s4);  // 都能用

    // 3. 借用
    let s5 = String::from("hello");
    let len = get_length(&s5);
    println!("{} has length {}", s5, len);

    // 4. 可变借用
    let mut s6 = String::from("hello");
    append_world(&mut s6);
    println!("{}", s6);  // hello, world

    // 5. 切片
    let s7 = String::from("hello world");
    let hello = &s7[0..5];
    let world = &s7[6..11];
    println!("{} {}", hello, world);

    // 6. 切片作为参数
    let first = first_word(&s7);
    println!("First word: {}", first);

    // 也适用于字符串字面值
    let literal = "hello world";
    let first = first_word(literal);
    println!("First word: {}", first);
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn append_world(s: &mut String) {
    s.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

---

恭喜你完成了第4章的学习！所有权是 Rust 最重要的概念，理解它将帮助你更好地理解 Rust 的其他特性。

下一章我们将学习**结构体（Structs）**，学习如何将相关数据组合在一起。
