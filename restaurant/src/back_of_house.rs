//公开结构体，但字段可以选择公开/私有混合
pub struct Breakfast {
    pub toast: String,      //公开字段
    seasonal_fruit: String, //私有字段
}

//因为有私有字段，所以必须提供公开的构造函数
impl Breakfast {
    pub fn Breakfast(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

//公开枚举所有变体自动公开
#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
}

fn deliver_order() {}

fn fix_incorrect_order() {
    cook_order();

    deliver_order();
}

fn cook_order() {}
