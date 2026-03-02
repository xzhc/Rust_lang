//Step by step refactoring: from variables to structs
//version1: use separate variables
fn use_separate_variables() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}

//version2: use tuple
fn use_tuple() {
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
}

//version3: use struct
fn use_struct() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    //error[E0277]: `Rectangle` doesn't implement `Debug`
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}

fn main() {
    use_separate_variables();
    use_tuple();
    use_struct();
}
