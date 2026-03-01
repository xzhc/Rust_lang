# 所有权与函数

> 📖 对应书中章节：Chapter 4.1 - Ownership and Functions / Return Values and Scope

## 📤 传递值给函数

将值传递给函数的机制与赋值类似：
- **Copy 类型**：复制传入，原变量仍有效
- **非 Copy 类型**：移动传入，原变量无效

---

## 🔄 函数中的移动

```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动进函数
                                    // ... 从此 s 不再有效！

    // println!("{}", s);           // ❌ 错误：s 已被移动

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 是 Copy 类型，复制传入
                                    // x 仍然有效！

    println!("x is still valid: {}", x);  // ✅ 正常
}

fn takes_ownership(some_string: String) {  // some_string 进入作用域
    println!("{}", some_string);
}  // some_string 离开作用域，调用 drop，内存被释放

fn makes_copy(some_integer: i32) {  // some_integer 进入作用域
    println!("{}", some_integer);
}  // some_integer 离开作用域，但它是 Copy，无特殊操作
```

### 执行流程图解

```text
main() 开始
    │
    ├─→ s = String::from("hello")
    │       │
    │       └─→ takes_ownership(s)  ←── s 被移动
    │               │                    s 在 main 中不再有效
    │               └─→ 函数结束，some_string 被 drop
    │
    ├─→ x = 5
    │       │
    │       └─→ makes_copy(x)  ←── x 被复制
    │               │               x 在 main 中仍然有效
    │               └─→ 函数结束，副本被丢弃
    │
    └─→ main() 结束
        x 离开作用域（被丢弃）
```

---

## 📥 返回值与所有权

返回值也可以转移所有权：

```rust
fn main() {
    let s1 = gives_ownership();         // 返回值移动给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 移动进函数
                                        // 返回值移动给 s3
}  // 作用域结束：
   // - s3 被 drop
   // - s2 已被移动，无操作
   // - s1 被 drop

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string                        // 返回值移动给调用者
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string                           // 返回值移动给调用者
}
```

---

## 🤔 问题来了

如果我们想让函数使用值但不获取所有权，该怎么办？

### 笨拙的方式：传进去再传出来

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);  // 传进去再拿回来

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)  // 返回 String 和长度
}
```

这太繁琐了！每次都要把所有权传来传去...

### 优雅的方式：引用（下一节主题）

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);  // 传引用，不转移所有权！

    println!("The length of '{}' is {}.", s1, len);  // s1 仍然有效
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s 离开作用域，但因为它不拥有数据，所以不会 drop
```

---

## 📊 所有权转移总结

### 变量赋值

```rust
let s1 = String::from("hello");
let s2 = s1;        // s1 → s2（移动）
// s1 不再有效
```

### 函数参数

```rust
let s = String::from("hello");
take(s);            // s → 函数（移动）
// s 不再有效
```

### 函数返回

```rust
fn create() -> String {
    let s = String::from("hello");
    s               // s → 调用者（移动）
}
```

### 从函数返回参数

```rust
fn pass_through(s: String) -> String {
    s               // 参数 → 调用者（移动）
}
```

---

## 🎯 最佳实践

1. **如果函数需要消费值**：获取所有权
   ```rust
   fn process_and_consume(s: String) {
       // 使用 s，之后不需要再用了
   }
   ```

2. **如果函数只需要读取**：使用引用（下节内容）
   ```rust
   fn read_only(s: &String) {
       // 只读，不获取所有权
   }
   ```

3. **如果函数需要修改**：使用可变引用（下节内容）
   ```rust
   fn modify(s: &mut String) {
       // 可以修改，但不获取所有权
   }
   ```

---

## 📝 练习

思考以下代码的输出和错误：

```rust
fn main() {
    let s = String::from("hello");

    let s2 = take_and_return(s);

    // println!("{}", s);   // 这行能编译吗？

    println!("{}", s2);     // 这行呢？
}

fn take_and_return(s: String) -> String {
    println!("Got: {}", s);
    s
}
```

<details>
<summary>点击查看答案</summary>

- `println!("{}", s);` **不能编译**，因为 `s` 已经移动到函数中
- `println!("{}", s2);` **可以编译**，所有权通过返回值移回来了

</details>

---

下一节我们将学习**引用和借用**，这是解决所有权传来传去的关键！
