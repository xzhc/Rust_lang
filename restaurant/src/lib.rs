mod back_of_house;
mod front_of_house;

//use 'use' keyword to bring the path into scope
// use crate::front_of_house::hosting;
// pub use：重导出，让外部代码可以用 restaurant::hosting::...
// 而不是 restaurant::front_of_house::hosting::...
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    //crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    //front_of_house::hosting::add_to_waitlist();

    //use keyword
    hosting::add_to_waitlist();

    // let mut meal = back_of_house::Breakfast::Breakfast("white");
    // meal.toast = String::from("brown");
    // println!("I'd like {} toast please.", meal.toast);
    // //meal.seasonal_fruit = String::from("blueberries");

    // let order1 = back_of_house::Appetizer::Soup;
    // let order2 = back_of_house::Appetizer::Salad;
    // println!("{:?},{:?}", order1, order2);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakfast() {
        let meal = back_of_house::Breakfast::Breakfast("white");
        assert_eq!(meal.toast, "white");
    }

    #[test]
    fn test_appetizer() {
        let _soup = back_of_house::Appetizer::Soup;
        let _salad = back_of_house::Appetizer::Salad;
    }

    #[test]
    fn test_hosting() {
        hosting::add_to_waitlist();
    }
}
