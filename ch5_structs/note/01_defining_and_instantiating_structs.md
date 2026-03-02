# 定义和实例化结构体

> 📖 对应书中章节：Chapter 5.1 - Defining and Instantiating Structs

## 🏠 什么是结构体（Struct）？

**结构体**是一种自定义数据类型，让你可以把多个相关的值打包在一起，并给它们起一个有意义的名字。

如果你熟悉面向对象语言，可以把结构体理解为"对象的数据属性"。

### 🔍 结构体 vs 元组

| 特性 | 元组 (Tuple) | 结构体 (Struct) |
|------|-------------|----------------|
| 存储多个值 | ✅ | ✅ |
| 值可以是不同类型 | ✅ | ✅ |
| 每个值有名字 | ❌ 只能靠索引 | ✅ 有字段名 |
| 访问方式 | `tuple.0` | `struct.field` |
| 顺序依赖 | 强依赖顺序 | 不依赖顺序 |

> 💡 结构体比元组更灵活：你不需要依赖数据的顺序来指定或访问值。

---

## 📝 定义结构体

使用 `struct` 关键字定义，在大括号内定义字段（field）的名称和类型：

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

> ⚠ 结构体定义**不**创建实例，它只是一个"模板"。

---

## 🎯 创建结构体实例

使用 `key: value` 对来创建实例：

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

> 💡 字段顺序不需要和定义时的顺序一致！

---

## 🔧 访问和修改字段

### 访问字段

使用**点号语法**：

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("Email: {}", user1.email);
}
```

### 修改字段

实例必须是 `mut` 的，**整个实例**必须可变（Rust 不允许只标记某些字段为可变）：

```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

---

## ⚡ 字段初始化简写（Field Init Shorthand）

当参数名和字段名**完全相同时**，可以简写：

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 普通写法 - 有点冗余
fn build_user_verbose(email: String, username: String) -> User {
    User {
        active: true,
        username: username,  // 参数名和字段名相同
        email: email,        // 参数名和字段名相同
        sign_in_count: 1,
    }
}

// 简写形式 - 更简洁
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,   // 等同于 username: username
        email,      // 等同于 email: email
        sign_in_count: 1,
    }
}
```

---

## 🔄 结构体更新语法（Struct Update Syntax）

当你想基于一个实例创建另一个实例，只修改部分字段时：

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 不使用更新语法 - 需要手动写每个字段
    let user2_verbose = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // 使用更新语法 - 更简洁
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1   // 其余字段来自 user1
    };
}
```

> ⚠ **`..user1` 必须放在最后！**

### ⚠ 所有权问题

结构体更新语法使用 `=`，会**移动**数据：

```rust
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // ❌ 错误！username 的所有权已移动到 user2
    // println!("{}", user1.username);

    // ✅ 仍然可用！email 没有被移动
    println!("{}", user1.email);

    // ✅ 仍然可用！active 和 sign_in_count 是 Copy 类型
    println!("{}", user1.active);
}
```

---

## 📦 元组结构体（Tuple Structs）

当你想给元组起个名字，但不需要为每个字段命名时：

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 访问元素 - 和元组一样用索引
    let r = black.0;
    let x = origin.0;

    // 解构
    let Point(x, y, z) = origin;
}
```

> ⚠ `Color` 和 `Point` 是**不同类型**！即使它们内部都是三个 `i32`，也不能互相赋值。

---

## 🔲 类单元结构体（Unit-Like Structs）

没有任何字段的结构体，类似于单元类型 `()`：

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

> 💡 当你需要为某个类型实现 trait，但不需要存储任何数据时，这种结构体很有用。我们会在第 10 章详细讨论。

---

## 🔐 结构体数据的所有权

在 `User` 结构体中，我们使用了 `String` 而不是 `&str`：

```rust
struct User {
    active: bool,
    username: String,   // 拥有所有权
    email: String,      // 拥有所有权
    sign_in_count: u64,
}
```

这是**刻意的选择**：让结构体拥有它所有数据的所有权，确保数据在结构体有效期内始终有效。

### ❌ 在结构体中存储引用

如果想在结构体中存储引用，需要使用**生命周期（lifetimes）**（第 10 章讲解）：

```rust
// ❌ 这段代码无法编译！
struct User {
    active: bool,
    username: &str,   // 缺少生命周期标注
    email: &str,      // 缺少生命周期标注
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
```

编译器错误：
```
error[E0106]: missing lifetime specifier
 --> src/main.rs:3:15
  |
3 |     username: &str,
  |               ^ expected named lifetime parameter
```

> 💡 在学习生命周期之前，使用 `String` 等拥有所有权的类型来避免这个问题。

---

## 📋 小结

| 概念 | 语法 | 说明 |
|------|------|------|
| 定义结构体 | `struct Name { field: Type }` | 创建自定义类型 |
| 创建实例 | `Name { field: value }` | 填充具体数据 |
| 访问字段 | `instance.field` | 用点号 |
| 字段简写 | `field` | 当参数名=字段名 |
| 更新语法 | `..instance` | 复用其他字段 |
| 元组结构体 | `struct Name(Type, Type)` | 有名字的元组 |
| 类单元结构体 | `struct Name;` | 无字段的结构体 |

---

## 🔨 动手试试

1. 定义一个 `Book` 结构体，包含书名、作者、页数、是否已读
2. 创建几本书的实例
3. 使用字段简写创建一本书
4. 使用更新语法基于一本书创建另一本（只修改书名）
5. 尝试打印结构体的某个字段
