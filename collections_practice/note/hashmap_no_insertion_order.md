# HashMap Does Not Preserve Insertion Order

## Code

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("xzh"), 100);
scores.insert(String::from("xzh2"), 200);
println!("{:?}", scores);
// Output: {"xzh2": 200, "xzh": 100}
```

## Problem

Why does `"xzh2"` appear before `"xzh"` in the output, even though `"xzh"` was inserted first?

## Answer

**`HashMap` does not preserve insertion order.**

### Why?

`HashMap` uses a **hash function** to determine where to store each key-value pair internally. The storage location is based on the **hash value of the key**, not the order you inserted it.

So the order you see in the output is determined by the internal hash table layout, which is essentially **random** (and can even differ between program runs!).

### Summary

| Collection | Order |
|---|---|
| `Vec` | ✅ Preserves insertion order |
| `HashMap` | ❌ **No guaranteed order** |

If you need a map that preserves insertion order, you could use `BTreeMap` from `std::collections`, which sorts keys **alphabetically/numerically**:

```rust
use std::collections::BTreeMap;

let mut scores = BTreeMap::new();
scores.insert(String::from("xzh"), 100);
scores.insert(String::from("xzh2"), 200);
println!("{:?}", scores);
// Output: {"xzh": 100, "xzh2": 200}  ← sorted by key
```

But for most use cases, `HashMap` is preferred because it's **faster** (O(1) lookup vs O(log n) for `BTreeMap`). Just don't rely on the order of its output!
