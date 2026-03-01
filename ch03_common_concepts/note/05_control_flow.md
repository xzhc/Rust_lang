# 控制流

> 📖 对应书中章节：Chapter 3.5 - Control Flow

控制流决定了代码执行的顺序。Rust 提供了 `if` 表达式和多种循环结构。

## 🔀 if 表达式

`if` 允许根据条件执行不同的代码分支：

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("条件为真");
    } else {
        println!("条件为假");
    }
}
```

### ⚠ 条件必须是 bool

Rust 不会自动将非布尔值转换为布尔值：

```rust
fn main() {
    let number = 3;

    if number {  // ❌ 编译错误！
        println!("number 是三");
    }
}
```

错误：
```
error[E0308]: mismatched types
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer
```

正确做法：

```rust
fn main() {
    let number = 3;

    if number != 0 {  // ✅ 显式比较
        println!("number 不是零");
    }
}
```

> 💡 **与动态语言的区别**：JavaScript/Python 会将 0 视为 false，非零视为 true。Rust 要求显式表达。

### else if

处理多个条件：

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number 能被 4 整除");
    } else if number % 3 == 0 {
        println!("number 能被 3 整除");
    } else if number % 2 == 0 {
        println!("number 能被 2 整除");
    } else {
        println!("number 不能被 4、3、2 整除");
    }
}
```

输出：
```
number 能被 3 整除
```

> 💡 **注意**：只执行第一个匹配的分支，后续分支不再检查。

### if 是表达式

`if` 可以用在 `let` 语句右边：

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("number 的值是: {number}");
}
```

> ⚠ **类型必须一致**：`if` 和 `else` 分支的返回值类型必须相同：

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };  // ❌ 类型不匹配！
}
```

## 🔄 循环

Rust 有三种循环：`loop`、`while` 和 `for`。

### loop：无限循环

`loop` 会一直执行，直到明确告诉它停止：

```rust
fn main() {
    loop {
        println!("again!");
        // 按 Ctrl-C 停止
    }
}
```

#### 使用 break 退出

```rust
fn main() {
    let mut count = 0;

    loop {
        count += 1;

        if count == 3 {
            println!("最终计数: {count}");
            break;  // 退出循环
        }
    }
}
```

#### 从 loop 返回值

`break` 可以返回值：

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // 返回 20
        }
    };

    println!("结果是 {result}");
}
```

#### 循环标签

嵌套循环时，`break` 和 `continue` 默认作用于最内层循环。使用标签可以指定：

```rust
fn main() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;  // 只退出内层循环
            }
            if count == 2 {
                break 'counting_up;  // 退出外层循环
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("最终 count = {count}");
}
```

> 💡 **标签语法**：循环标签以单引号开头，如 `'label_name:`

### while：条件循环

当条件为真时持续循环：

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("发射！！！");
}
```

输出：
```
3!
2!
1!
发射！！！
```

### for：遍历集合

`for` 是最常用的循环，用于遍历集合：

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("值是: {element}");
    }
}
```

#### for vs while 遍历数组

```rust
// 使用 while（容易出错）
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("值是: {}", a[index]);
        index += 1;
    }
}

// 使用 for（更安全、更简洁）
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("值是: {element}");
    }
}
```

> 💡 **for 的优势**：
> - 更简洁
> - 不会越界
> - 性能更好（不需要运行时边界检查）

#### Range

使用 `Range` 进行数字序列循环：

```rust
fn main() {
    // 正序
    for number in 1..4 {
        println!("{number}!");
    }
    // 输出：1! 2! 3!

    // 倒序
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("发射！！！");
}
```

输出：
```
3!
2!
1!
发射！！！
```

## 📊 循环对比

| 循环类型 | 用途 | 示例 |
|---------|------|------|
| `loop` | 无限循环，需要手动退出 | 重试操作、服务循环 |
| `while` | 条件循环 | 等待某个条件 |
| `for` | 遍历集合/范围 | 遍历数组、数字范围 |

## 🔧 break 和 continue

| 关键字 | 作用 |
|--------|------|
| `break` | 立即退出循环 |
| `continue` | 跳过本次迭代，进入下一次 |

```rust
fn main() {
    for i in 1..=10 {
        if i == 3 {
            continue;  // 跳过 3
        }
        if i == 7 {
            break;  // 到 7 停止
        }
        println!("{i}");
    }
}
```

输出：
```
1
2
4
5
6
```

## 🎯 综合示例

```rust
fn main() {
    // if 表达式
    let number = 6;
    let result = if number % 2 == 0 { "偶数" } else { "奇数" };
    println!("{number} 是 {result}");

    // for 循环遍历数组
    let fruits = ["苹果", "香蕉", "橙子"];
    for fruit in fruits {
        println!("我喜欢吃{fruit}");
    }

    // while 循环
    let mut countdown = 3;
    while countdown > 0 {
        println!("倒计时: {countdown}");
        countdown -= 1;
    }
    println!("开始！");

    // loop 带返回值
    let mut n = 1;
    let doubled = loop {
        n *= 2;
        if n > 100 {
            break n;
        }
    };
    println!("第一个大于 100 的 2 的幂是: {doubled}");
}
```

## 📝 速查表

```rust
// if 表达式
if condition {
    // ...
} else if other_condition {
    // ...
} else {
    // ...
}

// if 作为表达式
let value = if condition { a } else { b };

// loop 循环
loop {
    if should_stop {
        break;  // 或 break value;
    }
}

// while 循环
while condition {
    // ...
}

// for 循环
for item in collection {
    // ...
}

// Range
for i in 0..5 { }      // 0, 1, 2, 3, 4
for i in 0..=5 { }     // 0, 1, 2, 3, 4, 5
for i in (1..4).rev()  // 3, 2, 1

// 循环标签
'outer: loop {
    loop {
        break 'outer;  // 退出外层循环
    }
}
```

## 🎯 练习建议

1. 用 `for` 循环打印斐波那契数列的前 10 个数
2. 使用嵌套循环和标签打印九九乘法表
3. 写一个程序，用 `loop` 和 `break` 实现猜数字游戏
4. 用 `if` 表达式实现一个简单的温度转换器（华氏 ↔ 摄氏）
5. 打印 "The Twelve Days of Christmas" 的歌词，利用循环减少重复代码
