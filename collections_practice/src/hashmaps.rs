pub fn run() {
    use std::collections::HashMap;

    //create a new hashmap and insert key-value pairs
    let mut scores = HashMap::new();
    scores.insert(String::from("xzh"), 100);
    scores.insert(String::from("xzh2"), 200);
    println!("{:?}", scores);

    //created using iterators and 'collect'
    let teams = vec![String::from("xzh"), String::from("xzh2")];
    let initial_scores = vec![100, 200];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);

    //visit value of  hashmap
    //1. use'get'
    let score = scores.get("xzh").copied().unwrap_or(0);
    println!("{:?}", score);

    //2. iterate hashmap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //hashmap and ownership
    //1. the value is moved
    let field_name = String::from("favorite color");
    let field_value = String::from("blue");

    let mut colors = HashMap::new();
    colors.insert(field_name, field_value);
    println!("{:?}", colors);
    //borrow of moved value: `field_name`
    //println!("{}", field_name);
    //borrow of moved value: `field_value`
    //println!("{}", field_value);

    //update hashmap
    //1. overwrite old value
    let mut scores = HashMap::new();
    scores.insert(String::from("xzh"), 100);
    scores.insert(String::from("xzh"), 300);
    println!("{}", scores.get("xzh").copied().unwrap_or(0));

    //2. only insert if the key is not present
    //use 'entry' api

    let mut colors = HashMap::new();
    colors.insert(String::from("blue"), 10);
    colors.entry(String::from("blue")).or_insert(20); //don't change existed value
    colors.entry(String::from("yellow")).or_insert(20); //insert new value

    println!("{:?}", colors);

    //3. update value based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    //hash function
    //default hash function is SipHash
    //long: refuse attacked by dos
    //short: it's not the fast hash function
}
