use core::slice;

fn main() {
    // 1. ownership basics
    let _literal = "hello";

    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    //2. move_clone_copy

    // simply move by stack data
    let x = 6;
    let y = x; //copy x's value to y

    println!("x = {}, y = {}", x, y);

    // move by heap data
    let s1 = String::from("hello");
    let s2 = s1; // move s1 to s2

    println!("{}", s2);
    //println!("{}", s1); // error: s1 is moved

    // clone by heap data
    let s3 = String::from("hello");
    let s4 = s3.clone(); // clone s3 to s4

    println!("{}", s4);
    println!("{}", s3); // s3 is still valid

    //3.reference and borrowing
    //basic borrowing
    let s = String::from("hello");

    let len = get_length(&s);
    println!("The length of '{}' is {}.", s, len);

    // mutable borrowing
    let mut s = String::from("hello");
    append_world(&mut s);
    println!("{}", s);

    //more than one basic borrowing
    let s1 = String::from("hello");
    let s2 = &s1;
    let s3 = &s1;
    println!("{}, {}", s2, s3);

    //more than one mutable borrowing
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    //error[E0499]: cannot borrow `s1` as mutable more than once at a time
    //let s3 = &mut s1;
    println!("{}, {}", s2, s3);

    //scope separation
    let mut s1 = String::from("hello");
    {
        let s2 = &mut s1;
        s2.push_str(", world");
    }
    let s3 = &mut s1;
    s3.push_str("!");
    println!("{}", s3);

    //4.slices
    //from beginning
    let s = String::from("hello world");
    let slice = &s[0..5];
    let slice = &s[..5];
    println!("{}", slice);

    //to end
    let slice = &s[5..];
    let slice = &s[5..s.len()];
    println!("{}", slice);

    //whole string
    let slice = &s[..];
    let slice = &s;
    let slice = &s[0..s.len()];
    println!("{}", slice);

    //string literal
    let s = "hello world";
    let slice = &s[..];
    println!("{}", slice);

    let s2 = String::from("hello world");
    let hello = &s2[0..5];
    let world = &s2[6..11];
    println!("{}, {}", hello, world);

    //slice as a parameter
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);

    //also use for string literal
    let s2 = "hello world";
    let word2 = first_word(s2);
    println!("{}", word2);
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn append_world(s: &mut String) {
    s.push_str(", world!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
