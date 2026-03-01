# 函数

> 📖 对应书中章节：Chapter 3.3 - Functions

函数在 Rust 中无处不在。你已经见过最重要的函数：`main` 函数，它是程序的入口点。

## 📝 函数定义

使用 `fn` 关键字声明函数，函数名使用 **snake_case** 命名风格（全小写，下划线分隔）：

```rust
fn main() {
    println!("Hello, world!");

    another_function();  // 调用函数
}

fn another_function() {
    println!("另一个函数。");
}
```

> 💡 **位置无关**：Rust 不关心函数定义的位置，只要它在调用者可见的作用域内即可。你可以把 `another_function` 定义在 `main` 之前或之后。

## 📥 参数（Parameters）

函数可以有参数，参数是函数签名中的特殊变量。

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("x 的值是: {x}");
}
```

### 多个参数

用逗号分隔多个参数：

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("测量值是: {value}{unit_label}");
}
```

输出：
```
测量值是: 5h
```

> ⚠ **必须声明类型**：在函数签名中，**必须**声明每个参数的类型。这是 Rust 的设计决策，使得编译器能提供更好的错误信息。

### 参数 vs 实参

- **参数（Parameters）**：函数定义中的变量
- **实参（Arguments）**：调用函数时传入的具体值

```rust
fn print_value(x: i32) {  // x 是参数
    // ...
}

print_value(5);  // 5 是实参
```

## 📤 语句和表达式

理解语句和表达式的区别对 Rust 非常重要：

| | 语句（Statement） | 表达式（Expression） |
|--|------------------|---------------------|
| 定义 | 执行操作的指令 | 计算产生值 |
| 返回值 | 不返回值 | 返回值 |
| 分号 | 有 | 无 |

### 语句示例

```rust
fn main() {
    let x = 5;  // 这是一个语句
}
```

语句**不返回值**，所以不能这样写：

```rust
fn main() {
    let x = (let y = 6);  // ❌ 编译错误！
}
```

### 表达式示例

表达式**会返回值**：

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1  // 注意：没有分号！这是一个表达式
    };

    println!("y 的值是: {y}");  // 输出：y 的值是: 4
}
```

> 💡 **关键区别**：表达式**不加分号**。如果加了分号，它就变成了语句，不再返回值。

### 常见表达式

```rust
5 + 6              // 数学运算是表达式
func_name()        // 函数调用是表达式
macro_name!()      // 宏调用是表达式
{ let x = 1; x }   // 代码块是表达式
if condition { a } else { b }  // if 是表达式
```

## 🔙 返回值

函数可以向调用者返回值。返回值用 `->` 声明类型，但不命名。

```rust
fn five() -> i32 {
    5  // 这是返回值，没有分号
}

fn main() {
    let x = five();
    println!("x 的值是: {x}");  // 输出：x 的值是: 5
}
```

### 返回值的规则

1. 用 `->` 声明返回类型
2. 返回值等于函数体**最后一个表达式**的值
3. 可以用 `return` 关键字提前返回

```rust
fn plus_one(x: i32) -> i32 {
    x + 1  // 隐式返回
}

fn main() {
    let x = plus_one(5);
    println!("x 的值是: {x}");  // 输出：x 的值是: 6
}
```

### 🔄 显式返回 vs 隐式返回

Rust 提供两种返回值的方式：

#### 隐式返回（Implicit Return）✅ 推荐

函数体最后一个**表达式**（不加分号）自动作为返回值：

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y  // 隐式返回，无分号
}
```

#### 显式返回（Explicit Return）

使用 `return` 关键字主动返回，常用于**提前退出**：

```rust
fn check_positive(n: i32) -> &'static str {
    if n < 0 {
        return "负数";  // 提前返回
    }
    "非负数"  // 隐式返回
}
```

#### 关键区别对比

| | 显式返回 | 隐式返回 |
|---|---|---|
| 语法 | `return 值;` | 最后一行表达式（无分号） |
| 位置 | 任意位置 | 只能在函数末尾 |
| 常见用途 | 提前退出、条件分支 | 函数正常返回值 |

> 💡 **Rust 惯用风格**：优先使用隐式返回，仅在需要提前退出时才用 `return`。

### ⚠ 常见错误：加分号

如果在返回表达式后面加分号，会变成语句，导致错误：

```rust
fn plus_one(x: i32) -> i32 {
    x + 1;  // ❌ 加了分号，变成语句，返回 () 而不是 i32
}
```

编译错误：

```
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: remove this semicolon to return this value
```

> 💡 编译器会给出明确的建议：删除分号！

### 使用 `return` 提前返回

```rust
fn check_positive(x: i32) -> bool {
    if x > 0 {
        return true;  // 提前返回
    }
    false  // 最后一个表达式作为返回值
}
```

## 📊 语句 vs 表达式对比

```rust
fn main() {
    // 语句：不返回值
    let x = 5;

    // 表达式：返回值
    let y = {
        let x = 3;
        x + 1  // 表达式（无分号）→ 返回 4
    };

    // 表达式变成语句（加分号）
    let z = {
        let x = 3;
        x + 1;  // 语句（有分号）→ 返回 ()
    };
}
```

## 🎯 实践示例

```rust
fn main() {
    // 调用无参数函数
    say_hello();

    // 调用有参数函数
    print_sum(5, 3);

    // 使用返回值
    let result = multiply(4, 5);
    println!("4 * 5 = {result}");

    // 表达式作为返回值
    let abs_value = absolute(-10);
    println!("|-10| = {abs_value}");
}

fn say_hello() {
    println!("Hello!");
}

fn print_sum(a: i32, b: i32) {
    println!("{a} + {b} = {}", a + b);
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b  // 隐式返回
}

fn absolute(x: i32) -> i32 {
    if x < 0 {
        -x  // 表达式，返回负值
    } else {
        x   // 表达式，返回原值
    }
}
```

输出：
```
Hello!
5 + 3 = 8
4 * 5 = 20
|-10| = 10
```

## 📝 速查表

```rust
// 无参数、无返回值
fn function_name() {
    // 函数体
}

// 有参数、无返回值
fn function_name(param: Type) {
    // 函数体
}

// 有参数、有返回值
fn function_name(param: Type) -> ReturnType {
    // 函数体
    return_value  // 最后一个表达式（无分号）
}

// 提前返回
fn function_name(param: Type) -> ReturnType {
    if condition {
        return value;  // 提前返回
    }
    other_value
}
```

## 🎯 练习建议

1. 写一个 `add` 函数，接受两个 `i32` 参数，返回它们的和
2. 写一个 `greet` 函数，接受一个 `&str` 参数，打印问候语
3. 故意在返回表达式后加分号，观察编译错误
4. 写一个函数，使用代码块表达式计算并返回一个值
