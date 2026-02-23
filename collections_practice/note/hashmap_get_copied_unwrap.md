# HashMap get().copied().unwrap_or() Pattern

## Code

```rust
let score = scores.get("xzh").copied().unwrap_or(0);
```

## Problem

What does `scores.get("xzh").copied().unwrap_or(0)` do? Why chain three methods together?

## Answer

### Step-by-step

#### 1. `scores.get("xzh")` → `Option<&i32>`

`get()` returns an **`Option`** because the key might not exist:
- Key found → `Some(&100)` (a **reference** to the value)
- Key not found → `None`

#### 2. `.copied()` → `Option<i32>`

Converts `Option<&i32>` to `Option<i32>` by **copying** the value out of the reference:
- `Some(&100)` → `Some(100)`
- `None` → `None`

Without `.copied()`, you'd have an `Option<&i32>` (a reference), which can be inconvenient to work with.

#### 3. `.unwrap_or(0)` → `i32`

**Unwraps** the `Option`, providing a **default value** if it's `None`:
- `Some(100)` → `100`
- `None` → `0` (the fallback)

### Visual flow

```
scores.get("xzh")  →  Some(&100)    // Option<&i32>
    .copied()       →  Some(100)     // Option<i32>
    .unwrap_or(0)   →  100           // i32
```

### Why not just `.unwrap()`?

| Method | If `None` | Use when |
|---|---|---|
| `.unwrap()` | **Panics!** 💥 | You're 100% sure the key exists |
| `.unwrap_or(0)` | Returns `0` | You want a safe default value |
| `.unwrap_or_default()` | Returns type's default | Same, but uses the type's default (0 for integers) |

So this pattern `get().copied().unwrap_or()` is the **idiomatic, safe** way to get a value from a HashMap with a fallback!
