pub fn run() {
    //1. vectors

    // use Vec::new to create a new vector
    //let v: Vec<i32> = Vec::new();

    // use vec! macro to create a new vector with initial values
    //let v = vec![1, 2, 3, 4, 5];

    // use 'push' to add elements to a vector
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    //read vector's element

    //1.using index syntax (&v[index])
    let second = &v[1]; //index from zero to length-1
    println!("The second element is {}", second);

    //2.using get method( get())
    let third = v.get(2);
    match third {
        Some(third) => println!("the third element is {}", third),
        None => println!("There is no third element"),
    }

    let fourth = v.get(4);
    match fourth {
        Some(fourth) => println!("the fourth element is {}", fourth),
        None => println!("There is no fourth element"),
    }

    //browwing rule and vector

    //can't edit vector while having a reference to it

    //why is designed this way?
    //A vector is stored contiguously in memory. When adding a new element:
    //(1). If the current space is insufficient, new memory space must be allocated.
    //(2). All elements are copied to new location.
    //(3). The old memory is disallocated.
    //If the 'first' reference is still pointing to the old memory location, a dangling reference will occure!
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    //v.push(4); //error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("The first element is {}", first);

    //iterate vector
    //1. iterate immutable reference
    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        print!("{} ", i);
    }
    println!();

    //2. iterate mutable reference(edit elements)
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 1;
    }
    println!("{:?}", v);

    //storage multiple types by using enum
    //The vector has a limit: it can only store one type of element.
    //But what if we want to store multiple types of elements?
    //Solution: We can use enum to store multiple types of elements.
    enum SpreedsheetShell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    //why this is possibile?
    //because all variants of an enum belong to the same enum type, so the vector stores elements of the same type.
    let row = vec![
        SpreedsheetShell::Int(1),
        SpreedsheetShell::Float(1.0),
        SpreedsheetShell::Text(String::from("hello")),
    ];

    for i in row {
        match i {
            SpreedsheetShell::Int(i) => println!("{} is an integer", i),
            SpreedsheetShell::Float(i) => println!("{} is a float", i),
            SpreedsheetShell::Text(i) => println!("{} is a text", i),
        }
    }

    //Vectors and disallocation
    //rule: when a vector goes out of scope, it automatically disallocated, including all elements it contains.
    {
        let v = vec![1, 2, 3]; // use 'v'
    } // 'v' goes out of scope, it automatically disallocated
}
