pub fn run() {
    // create strings

    //1. 'String::new()' to create an empey string
    let mut s = String::new();

    //2.'to_string()'
    let data = "initial content";
    let s = data.to_string();
    //can also be used directly for literals
    let s = "initial contents".to_string();

    //3.'String::from()'
    let s = String::from("initial contents");

    //update strings
    //1. use'push_str()'to add string slice
    let mut s = String::from("xzh ");
    s.push_str("is a handsome man");
    println!("{}", s);

    //why use &str instead of &String?
    //because &String can't be borrowed as &str
    let mut s1 = String::from("xzh is a ");
    let s2 = "handsome man";
    s1.push_str(&s2);
    println!("{}", s1);
    println!("{}", s2);

    //2.use 'push' to add a character
    let mut s = String::from("xzh");
    s.push('!');
    println!("{}", s);

    //3. use '+' operator to add strings
    let s1 = String::from("xzh ");
    let s2 = String::from("handsome");
    let s3 = s1 + &s2;
    //println!("{}", s1);
    println!("{}", s3);
    println!("{}", s2);

    //4. use 'format!' macro to add strings(recommend)
    let s1 = String::from("xzh");
    let s2 = String::from("handsome");
    let s3 = String::from("man");

    //let s4 = s1 + "-" + &s2 + "-" + &s3;
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s4);

    //strings index
    // rust does not suppporting string index
    let s = String::from("hello");
    //let h = s[0]; //error[E0277]: the type `String` cannot be indexed by `usize`

    //string slices
    //although rust does not support string index, we can use string slices to get a part of a string
    let s = String::from("zxcvbnm");
    let s1 = &s[0..4];
    println!("{}", s1);

    //If a slice is taken in the middle of a multi-byte character, the program will panic!
    // let hello = "Здравствуйте";
    // let s = &hello[0..1];
    // println!("{}", s);

    //iterate string
    //1. use 'chars()' to iterate unicode scalar values
    let s = String::from("zxcvbnm");
    for c in s.chars() {
        println!("{}", c);
    }

    //2. use 'bytes()' to iterate bytes
    let s = String::from("zxcvbnm");
    for b in s.bytes() {
        println!("{}", b);
    }
}
