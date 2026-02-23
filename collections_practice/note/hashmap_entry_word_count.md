# HashMap Word Frequency Counter with entry().or_insert()

## Code

```rust
let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
println!("{:?}", map);
// Output: {"wonderful": 1, "hello": 1, "world": 2}
```

## Problem

How does `map.entry(word).or_insert(0)` and `*count += 1` work together to count word frequencies?

## Answer

### Step-by-step

#### 1. `text.split_whitespace()`
Splits the string by whitespace, producing an iterator:
```
"hello" → "world" → "wonderful" → "world"
```

#### 2. `map.entry(word)` → `Entry`
Returns an **`Entry`** enum — it represents a slot in the HashMap that either:
- **`Occupied`** — the key already exists
- **`Vacant`** — the key doesn't exist yet

#### 3. `.or_insert(0)` → `&mut V`
- If **Vacant** → inserts `0` as the value, then returns a **mutable reference** to it
- If **Occupied** → just returns a **mutable reference** to the existing value

So `count` is always a `&mut i32` pointing to the value in the map.

#### 4. `*count += 1`
Dereferences and increments the value (same `*` pattern as the vector example!).

### Walkthrough of each iteration

| Iteration | `word` | `map` before | `entry` | `count` after `or_insert` | After `*count += 1` |
|---|---|---|---|---|---|
| 1 | `"hello"` | `{}` | Vacant | `&mut 0` | `{"hello": 1}` |
| 2 | `"world"` | `{"hello": 1}` | Vacant | `&mut 0` | `{"hello": 1, "world": 1}` |
| 3 | `"wonderful"` | `{..}` | Vacant | `&mut 0` | `{.., "wonderful": 1}` |
| 4 | `"world"` | `{..}` | **Occupied** | `&mut 1` (existing!) | `{.., "world": 2}` |

### Final result
```
{"wonderful": 1, "hello": 1, "world": 2}
```

### Key insight 💡

The magic of `entry().or_insert()` is that it handles both cases (**insert new** or **get existing**) in a single, elegant expression — no need for an `if` check! This is the idiomatic Rust way to do "upsert" operations.
