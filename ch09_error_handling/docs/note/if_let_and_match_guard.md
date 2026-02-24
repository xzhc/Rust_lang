# `if let` 与 `match` 守卫语法糖笔记

> 适合人群：刚学完 `match` 基础用法，想简化代码的 Rust 初学者

---

## 一、先回顾：`match` 处理 `Result`

`Result<T, E>` 是一个枚举，有两个变体：

```rust
enum Result<T, E> {
    Ok(T),   // 成功，包含返回值
    Err(E),  // 失败，包含错误信息
}
```

用 `match` 处理时，必须覆盖**所有**变体：

```rust
let result: Result<&str, &str> = Ok("你好");

match result {
    Ok(msg) => println!("收到消息: {}", msg),
    Err(e)  => println!("发生错误: {}", e),
}
```

这很好，但如果你**只关心其中一种情况**，写两个分支显得冗余。

---

## 二、`if let` —— 只关心一个变体时的语法糖

### 基本语法

```rust
if let 模式 = 表达式 {
    // 匹配成功时执行
}
```

### 例子：只处理成功情况

```rust
let result: Result<&str, &str> = Ok("你好");

// ✅ 简洁写法
if let Ok(msg) = result {
    println!("收到消息: {}", msg);
}
// Err 的情况直接忽略，什么都不做
```

等价的 `match` 展开：

```rust
match result {
    Ok(msg) => println!("收到消息: {}", msg),
    _ => {}   // 其他情况什么都不做
}
```

### 例子：只处理失败情况

```rust
let error: Result<&str, &str> = Err("连接失败");

if let Err(e) = error {
    println!("发生错误: {}", e);
}
// Ok 的情况直接忽略
```

---

## 三、`if let` 的执行顺序

```rust
if let Ok(msg) = result {
//  ①      ②       ③
    println!("收到消息: {}", msg);  // ④
}
```

| 步骤 | 说明 |
|------|------|
| **③ 求值** | 先对 `=` 右侧的 `result` 求值 |
| **② 模式匹配** | 检查 `result` 是否符合 `Ok(msg)` 这个模式 |
| **② 变量绑定** | 如果匹配成功，把 `Ok` 内部的值绑定到 `msg` |
| **④ 执行块** | 匹配成功才进入 `{}` 块，`msg` 在此可用 |
| **跳过块** | 匹配失败（如是 `Err`），整个块被跳过 |

---

## 四、加上 `else` 处理另一种情况

```rust
if let Ok(msg) = result {
    println!("成功: {}", msg);   // 匹配 Ok 时
} else {
    println!("失败了");           // 不是 Ok 时（即 Err）
}
```

> 💡 **选择原则**：
> - 只关心**一个**变体 → 用 `if let`（更简洁）
> - 两个变体都要处理 → 用 `match`（更清晰）

---

## 五、`match` 守卫（Match Guard）—— 在模式上加条件

有时候匹配到 `Ok(n)` 还不够，还想根据 `n` 的值进一步区分。这时用 **match 守卫**（`if` 关键字加在模式后面）：

### 语法

```rust
match 表达式 {
    模式 if 条件 => 执行,
    模式         => 执行,
}
```

### 例子

```rust
fn check_number(result: Result<i32, &str>) {
    match result {
        Ok(n) if n > 50 => println!("大数字: {}", n),   // ← 守卫：n 必须 > 50
        Ok(n)           => println!("小数字: {}", n),   // ← 其余的 Ok
        Err(e)          => println!("错误: {}", e),
    }
}

check_number(Ok(100));   // 输出：大数字: 100
check_number(Ok(30));    // 输出：小数字: 30
check_number(Err("无效")); // 输出：错误: 无效
```

### 执行顺序

```
① 匹配 Ok(n) if n > 50
   ├─ 先检查是不是 Ok 变体
   └─ 再检查守卫条件 n > 50
       ├─ 都满足 → 执行这个分支
       └─ 不满足 → 继续往下匹配

② 匹配 Ok(n)
   └─ 是 Ok 但守卫失败的情况都到这里

③ 匹配 Err(e)
   └─ 是 Err 变体的情况
```

> ⚠️ **注意**：守卫失败不会导致报错，只会继续尝试下一个分支，就像没匹配上一样。

---

## 六、三种写法对比

```rust
let result: Result<i32, &str> = Ok(100);

// 写法①：match（最完整，所有情况都覆盖）
match result {
    Ok(n) if n > 50 => println!("大数字: {}", n),
    Ok(n)           => println!("小数字: {}", n),
    Err(e)          => println!("错误: {}", e),
}

// 写法②：if let（只关心 Ok，忽略 Err）
if let Ok(n) = result {
    println!("数字: {}", n);
}

// 写法③：if let + else（两种情况都处理，但不需要守卫时）
if let Ok(n) = result {
    println!("成功: {}", n);
} else {
    println!("失败");
}
```

---

## 七、总结

```
处理 Result / Option 时如何选择写法：

只关心一个变体？
  ├─ 是 → if let（简洁）
  └─ 否 → match（完整）

需要对匹配值进一步判断？
  └─ 是 → match + 守卫（if 条件写在模式后面）

记住：if let 是 match 的语法糖，本质一样，只是省略了不关心的分支。
```
