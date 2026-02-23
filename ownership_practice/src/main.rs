
fn main() {
    // println!("Hello, world!");
    //exercise1();
    //exercise2();
   // exercise3();
   //exercise4();
   exercise5();
}


fn exercise1() {
    println!("--exercise1-ownership move--");

    let s1 = String::from("hello");
    println!("s1 = {}", s1);

    let s2 = s1;

    //println!("s1 = {}", s1);
    //因为在这段代码里发生了**所有权移动（move）**：

    // - 在 `ownership_practice/src/main.rs:11`，`s1` 是 `String`。
    // - 在 `ownership_practice/src/main.rs:13`，`let s2 = s1;` 会把 `s1` 的所有权移动给 `s2`（`String` 不是 `Copy` 类型）。
    // - 所以到 `ownership_practice/src/main.rs:15` 时，`s1` 已经失效，`println!("s1 = {}", s1);` 就触发 `E0382: borrow of moved value`。

    // 可以这样改：

    // 1. 需要两个独立字符串时用克隆：
    // ```rust
    // let s2 = s1.clone();
    // println!("s1 = {}", s1);
    // println!("s2 = {}", s2);
    // ```

    // 2. 只想“借用”不转移所有权时，用引用：
    // ```rust
    // let s2 = &s1;
    // println!("s1 = {}", s1);
    // println!("s2 = {}", s2);
    // ```

    // 3. 或者在 move 前先使用 `s1`。

    println!("s2 = {}", s2);
}

fn exercise2(){
    println!("--exercise2:clone vs copy--");
    //string type: need clone() to deep copy

    let s1 = String::from( "hello");
    let s2 = s1.clone();
    println!("s1={}, s2={}", s1, s2);

    //integer number type: to achieve copy trait
    let x = 5;
    let y = x; //automatic copy
    println!("x = {}, y ={}",x, y)
}

fn exercise3() {
    println!("--exercise3 ownership of function--");

    let s = String::from( "hello");

    //takes_ownership( s);
    //println!("s={}", s);

    let len = calculate_length(&s);
    println!("The length of '{}' is {}", s, len);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn calculate_length(s: &String) -> usize{
    s.len()
}

fn exercise4(){
    println!("--exercise4:reference and borrowing--");

    let mut s = String::from( "hello");

    let r1 = &s;
    let r2 = &s;

     println!("r1={}, r2={}", r1, r2);

    //error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    let r3 =&mut s;
    r3.push_str("world");
    println!("r3={}", r3);
}

fn exercise5(){
    println!("--exercise5:slice--");
    
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world =&s[6..11];

    println!("first world: {}", hello);
    println!("second world: {}", world);
    let world = first_world(&s);
    println!("first world = {}", world)
}

fn first_world(s: &str) -> &str{
    let bytes = s.as_bytes();
    //println!("{:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}