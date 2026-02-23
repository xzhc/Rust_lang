# `max_by_key` 和 `map` 的作用

> 来源：`exercise.rs` 中计算众数（mode）的代码片段

## 问题代码

```rust
let mode = counts
    .iter()
    .max_by_key(|&(_, count)| count)
    .map(|(&num, _)| num)
    .unwrap_or(0);
```

---

## `max_by_key` — 按指定条件找最大值

```rust
.max_by_key(|&(_, count)| count)
```

**作用**：遍历迭代器中的所有元素，按照你指定的"评分函数"找出**得分最高**的那个元素。

假设此时 `counts` 是：
```
{ 2: 3, 7: 3, 4: 1, 9: 1, 5: 1 }
```

`.iter()` 产生的每个元素是一个 `(&key, &value)` 元组，比如 `(&2, &3)`、`(&7, &3)` ...

`max_by_key` 的闭包 `|&(_, count)| count` 意思是：
- 把元组解构为 `(key, count)`，其中 key 用 `_` 忽略掉
- 返回 `count` 作为"评分"
- 找出 count **最大**的那对 `(key, count)`

返回类型是 `Option<(&key, &value)>`，即 `Some((&2, &3))` 或 `Some((&7, &3))`。

---

## `map` — 变换 Option 内部的值

```rust
.map(|(&num, _)| num)
```

**作用**：对 `Option` 中的值做转换，**不改变 Some/None 结构**。

上一步 `max_by_key` 返回的是 `Some((&2, &3))`，但我们只关心 key（即数字本身），不需要 count。

所以用 `map` 把 `Some((&2, &3))` 变换成 `Some(2)`：
- `&num` 解构引用，拿到整数 `2`
- `_` 忽略 count
- 返回 `num`

---

## 类型变化总结

| 步骤 | 类型 | 值（举例）|
|---|---|---|
| `max_by_key` 后 | `Option<(&i32, &i32)>` | `Some((&2, &3))` |
| `.map(...)` 后 | `Option<i32>` | `Some(2)` |
| `.unwrap_or(0)` 后 | `i32` | `2` |

---

## 一句话总结

- **`max_by_key`**：按某个字段比大小，找出最大的那个元素
- **`map`**：对 `Option` 里的值做"变形"，把不需要的部分去掉，只保留想要的
