struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("--fifth chacpter structs exercise --");
    println!("{}", "=".repeat( 50));
    // exercise1:basic structs
    let user1 = creaet_user(String::from( "xzh990520@gmail.com"), String::from( "xzh"));

    println!("user1 information");

    println!("email:{}", user1.email);
    println!("username:{}", user1.username);
    println!("active:{}", user1.active);
    println!("login numbers:{}", user1.sign_in_count);

    println!("{}","=".repeat( 50));
    
    //exercise2:the field init shorthand
    let user2 = build_user(String::from( "1838998336@qq.com"), String::from( "18389"));

    println!("user2 information");

    println!("email:{}", user2.email);
    println!("username:{}", user2.username);
    println!("active:{}", user2.active);
    println!("login numbers:{}", user2.sign_in_count);

    println!("{}", "=".repeat( 50));
    
    //exercise3 update structs
    let user3 = User{
        email: String::from( "1047252400@qq.com"),
        ..user2
    };

    println!("updated user:{} ({})\n", user3.username, user3.email);
    println!("email:{}", user2.email);
    //println!("username:{}", user2.username);

    println!("{}", "=".repeat( 50));
    //exercise4:tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("黑色RGB({},{},{})", black.0, black.1, black.2);
    println!("原点坐标{},{},{}", origin.0, origin.1, origin.2);

    println!("{}", "=".repeat( 50));

    //exercise5: units struct
    #[derive(Debug)]
    struct AlawysEqual;
    let _subject = AlawysEqual;
    println!("create unit struct instance: {:?}\n", _subject);
    
    println!("{}", "=".repeat( 50));


    //exercise6 Rectangle structs
    println!("exercise6: Rectangle struct");
    #[derive(Debug)]
    struct Rectangle {
        height: i32,
        width: i32,
    }

    let rec1 = Rectangle{height: 20, width: 30};

    println!("rec1 is {:?}", rec1);
    println!("rec1 is {:#?}", rec1);

    println!("{}", "=".repeat( 50));


    //exercise7 method area
    println!("exercise7: method - area()");

    impl Rectangle {
        fn area(&self) -> i32 {
            self.width * self.height
        }

        fn width(&self) -> bool {
            self.width > 0
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        //关联函数：impl中定义的所有函数
        fn suqare(size: i32) -> Self {
            Self { height: size, width: size }
        }
    }

    let rec1 = Rectangle{width:20, height:10};
    println!("area:{}", rec1.area());

    //方法与字段同名
    if rec1.width() { //调用方法
        println!("rec1's width:{}", rec1.width); //访问字段
    }

    //Methods with more parameters
    let rec2 = Rectangle{width: 25, height:15};
    println!("{}", rec1.can_hold( &rec2));

    let sq = Rectangle::suqare(10);
    println!("{},{}",sq.width, sq.height);

    
}

//create user
fn creaet_user(email: String, username: String) -> User{
    User {
        active: true,
        username:username,
        email: email,
        sign_in_count: 1,
    }
}

//Using the field init shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
