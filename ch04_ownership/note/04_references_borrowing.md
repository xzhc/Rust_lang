# 引用与借用

> 📖 对应书中章节：Chapter 4.2 - References and Borrowing

## 🎯 什么是引用？

**引用（Reference）** 允许你使用值但不获取其所有权。

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);  // &s1 创建一个指向 s1 的引用

    println!("The length of '{}' is {}.", s1, len);
    // s1 仍然有效！
}

fn calculate_length(s: &String) -> usize {  // s 是一个引用
    s.len()
}  // s 离开作用域，但因为它不拥有数据，所以不会 drop
```

---

## 📖 借用（Borrowing）

创建引用的行为称为**借用**。

> 类比：就像现实生活中借东西，用完要还，你不能随便处置它。

```rust
let s1 = String::from("hello");
let s2 = &s1;  // s2 "借用"了 s1

// s2 可以读取 s1 的值
println!("{}", s2);

// 但 s2 不拥有它，离开作用域时不会释放
```

---

## 🏗️ 引用的内存布局

```text
┌─────────────────┐        ┌─────────────────┐
│ s1 (所有者)     │        │ 堆上的数据      │
│  ptr ────────────────→   │ "hello"         │
│  len  = 5       │        └─────────────────┘
│  cap  = 5       │              ↑
└─────────────────┘              │
┌─────────────────┐              │
│ s (引用)        │              │
│  ptr ──────────────────────────┘
└─────────────────┘
```

引用 `s` 只存储一个指针，不拥有数据。

---

## 🔒 引用默认不可变

默认情况下，引用是**不可变的**，你不能修改借来的值：

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");  // ❌ 错误！不能修改
}
```

错误信息：
```text
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
```

---

## ✏️ 可变引用

如果需要修改借来的值，使用**可变引用** `&mut`：

```rust
fn main() {
    let mut s = String::from("hello");  // 1. s 必须是 mut

    change(&mut s);                      // 2. 传递 &mut s

    println!("{}", s);  // 输出: hello, world
}

fn change(some_string: &mut String) {    // 3. 参数是 &mut String
    some_string.push_str(", world");
}
```

### 可变引用的三个要求

1. **原变量必须是 `mut`**
2. **传递时使用 `&mut`**
3. **函数参数类型是 `&mut`**

---

## ⚠️ 可变引用的限制

### 限制一：同一时刻只能有一个可变引用

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;  // ❌ 错误！已经有一个可变引用了

println!("{}, {}", r1, r2);
```

错误：
```text
error[E0499]: cannot borrow `s` as mutable more than once at a time
```

### 为什么有这个限制？

**防止数据竞争（Data Race）**！

数据竞争发生在三个条件同时满足时：
1. 两个或更多指针同时访问同一数据
2. 至少有一个指针在写入
3. 没有同步机制

Rust 在编译时就阻止了这种情况！

### 解决方案：使用作用域分隔

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
    // 使用 r1
}  // r1 离开作用域，可以创建新的可变引用了

let r2 = &mut s;  // ✅ 没问题！
```

---

## ⚠️ 可变引用与不可变引用不能同时存在

```rust
let mut s = String::from("hello");

let r1 = &s;      // 不可变引用
let r2 = &s;      // 不可变引用
let r3 = &mut s;  // ❌ 错误！已有不可变引用时不能创建可变引用

println!("{}, {}, {}", r1, r2, r3);
```

### 为什么？

不可变引用的用户不希望值在他们眼皮底下被改变！

### 解决方案：不可变引用使用完毕后再创建可变引用

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{} and {}", r1, r2);
// r1 和 r2 在这之后不再使用

let r3 = &mut s;  // ✅ 没问题！
println!("{}", r3);
```

> 💡 **引用的作用域**
> 引用的作用域从它被创建开始，到**最后一次被使用**结束（不一定是到花括号）。

---

## 📊 引用规则速查表

| 场景 | 允许？ | 例子 |
|------|--------|------|
| 多个不可变引用 | ✅ | `let r1 = &s; let r2 = &s;` |
| 一个可变引用 | ✅ | `let r1 = &mut s;` |
| 多个可变引用 | ❌ | `let r1 = &mut s; let r2 = &mut s;` |
| 不可变 + 可变 | ❌ | `let r1 = &s; let r2 = &mut s;` |

### 简化记忆

> 在任意给定时间，**要么**只能有一个可变引用，**要么**只能有任意数量的不可变引用。

---

## 🕳️ 悬垂引用（Dangling References）

**悬垂引用**：指向已被释放内存的引用。Rust 保证永远不会出现这种情况！

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {  // ❌ 错误！
    let s = String::from("hello");

    &s  // 返回 s 的引用
}  // s 在这里被 drop！引用指向了无效内存！
```

错误信息：
```text
error[E0106]: missing lifetime specifier
this function's return type contains a borrowed value,
but there is no value for it to be borrowed from
```

### 正确做法：返回所有权

```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 返回 String 本身，所有权转移
}
```

---

## 📝 引用规则总结

记住这两条规则：

```text
1. 在任意给定时间，要么只能有一个可变引用，
   要么只能有任意数量的不可变引用。

2. 引用必须总是有效的（不能悬垂）。
```

---

## 🎯 实践示例

```rust
fn main() {
    // 1. 基本借用
    let s = String::from("hello");
    let len = get_length(&s);
    println!("Length of '{}' is {}", s, len);

    // 2. 可变借用
    let mut s = String::from("hello");
    append_world(&mut s);
    println!("{}", s);  // hello, world

    // 3. 多个不可变引用
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);  // ✅ 没问题

    // 4. 作用域分隔
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str(" there");
    }
    let r2 = &mut s;
    r2.push_str("!");
    println!("{}", s);  // hello there!
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn append_world(s: &mut String) {
    s.push_str(", world");
}
```

---

下一节我们将学习**切片（Slices）**，这是另一种引用类型！
