# Creating HashMap with Iterators, Zip, and Collect

## Code

```rust
let teams = vec![String::from("xzh"), String::from("xzh2")];
let initial_scores = vec![100, 200];

let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
println!("{:?}", scores);
```

## Problem

How does `teams.into_iter().zip(initial_scores.into_iter()).collect()` work? What does each part do?

## Answer

### Step-by-step breakdown

#### 1. `teams.into_iter()`
Converts the `teams` vector into an **iterator**:
```
"xzh" → "xzh2"
```

#### 2. `.zip(initial_scores.into_iter())`
`zip` **pairs up** elements from two iterators, like a zipper 🤐:
```
("xzh", 100) → ("xzh2", 200)
```
It produces an iterator of **tuples** `(String, i32)`.

#### 3. `.collect()`
Consumes the iterator and **collects** the results into a collection. But what type? Rust needs to know — that's what the type annotation is for.

#### 4. `HashMap<_, _>` (type annotation)
- This tells `.collect()`: "I want a `HashMap`"
- The `_` means "Rust, please **infer** the key and value types for me"
- Rust figures out it's `HashMap<String, i32>` from the iterator's tuple types

### Visual flow

```
teams:           ["xzh",  "xzh2"]
                    ↓        ↓
initial_scores:  [100,     200  ]
                    ↓        ↓
zip:             [("xzh", 100), ("xzh2", 200)]
                    ↓
collect:         {"xzh": 100, "xzh2": 200}
```

### Why `into_iter()` instead of `iter()`?

| Method | Produces | Ownership |
|---|---|---|
| `.iter()` | `&T` (references) | Original vec is **kept** |
| `.into_iter()` | `T` (owned values) | Original vec is **consumed** |

Here `into_iter()` is used because `HashMap` needs to **own** its keys and values. After this line, `teams` and `initial_scores` can no longer be used — their ownership has been **moved** into the HashMap.
