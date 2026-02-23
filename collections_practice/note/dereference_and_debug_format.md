# Dereference Operator (`*`) and Debug Format (`{:?}`)

## Code

```rust
// iterate mutable reference (edit elements)
let mut v = vec![1, 2, 3];
for i in &mut v {
    *i += 1;
}
println!("{:?}", v); // Output: [2, 3, 4]
```

## Problem

In the code above, why do we need `*` in `*i += 1`? And why do we use `{:?}` instead of `{}` in `println!("{:?}", v)`?

## Answer

### 1. `*` — The Dereference Operator

```rust
for i in &mut v {
    *i += 1;
}
```

When you iterate with `&mut v`, each `i` is a **mutable reference** (`&mut i32`), not the value itself. It's like a pointer to the element inside the vector.

To **modify the actual value** that `i` points to, you need to **dereference** it with `*`. This tells Rust: "follow the reference and change the value at that memory location."

| Expression | Type | Meaning |
|---|---|---|
| `i` | `&mut i32` | A mutable reference (pointer to the element) |
| `*i` | `i32` | The actual value the reference points to |

Without `*`, you'd be trying to do math on a reference, which doesn't make sense — you want to change the **value inside the vector**, not the reference itself.

### 2. `{:?}` — The Debug Format Trait

```rust
println!("{:?}", v);  // Output: [2, 3, 4]
```

- **`{}`** uses the **`Display`** trait — meant for user-friendly, pretty output.
- **`{:?}`** uses the **`Debug`** trait — meant for developer/debugging output.

`Vec<T>` **does not** implement `Display`, so `{}` won't work with it:

```rust
println!("{}", v);   // ❌ Error: Vec doesn't implement Display
println!("{:?}", v); // ✅ Works: Vec implements Debug → [2, 3, 4]
```

Think of it this way:
- `Display` (`{}`) → "How should an end-user see this?"
- `Debug` (`{:?}`) → "How should a developer see this for debugging?"

Simple types like `i32`, `String`, etc. implement both. But collections like `Vec` only implement `Debug`, since there's no single "right way" to display a collection to a user.

> 💡 **Bonus**: You can also use `{:#?}` for **pretty-printed** debug output (with indentation), which is handy for nested structures.
