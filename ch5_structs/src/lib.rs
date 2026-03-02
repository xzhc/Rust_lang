//define a struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
#[cfg(test)]
mod tests {

    use super::*;

    // 知识点1：创建实例、访问字段、修改字段
    #[test]
    fn test_basic_struct() {
        let mut user1 = User {
            active: true,
            username: String::from("user1"),
            email: String::from("user1@example.com"),
            sign_in_count: 1,
        };

        // 访问字段
        assert_eq!(user1.email, "user1@example.com");
        assert!(user1.active);

        // 修改字段（整个实例必须是 mut）
        user1.email = String::from("user1_new@example.com");
        assert_eq!(user1.email, "user1_new@example.com");
    }

    // 知识点2：构造函数（冗余写法）
    #[test]
    fn test_build_user_verbose() {
        let user = build_user_verbose(String::from("user2@example.com"), String::from("user2"));
        assert_eq!(user.email, "user2@example.com");
        assert_eq!(user.username, "user2");
    }

    // 知识点3：字段初始化简写（Field Init Shorthand）
    #[test]
    fn test_build_user_concise() {
        let user = build_user_concise(String::from("user3@example.com"), String::from("user3"));
        assert_eq!(user.email, "user3@example.com");
        assert_eq!(user.username, "user3");
    }

    // 知识点4：结构体更新语法（Struct Update Syntax）
    #[test]
    fn test_build_user_update() {
        let user1 = User {
            username: String::from("someone"),
            email: String::from("someone@example.com"),
            active: false,
            sign_in_count: 1,
        };
        let user2 = User {
            email: String::from("another@example.com"),
            ..user1
        };

        assert_eq!(user2.email, "another@example.com");
        assert_eq!(user2.username, "someone");
        assert_eq!(user2.active, false);
        assert_eq!(user2.sign_in_count, 1);
    }

    //知识点5: 所有权问题
    #[test]
    fn test_ownership() {
        let user1 = User {
            username: String::from("someone"),
            email: String::from("someone@example.com"),
            active: false,
            sign_in_count: 1,
        };
        let user2 = User {
            email: String::from("another@example.com"),
            ..user1
        };

        assert_eq!(user2.email, "another@example.com");
        assert_eq!(user2.username, "someone");
        assert_eq!(user2.active, false);
        assert_eq!(user2.sign_in_count, 1);
        //error[E0382]: borrow of moved value: `user1.username`
        //assert_eq!(user1.username, "someone");
        assert_eq!(user1.email, "someone@example.com");
    }

    // 知识点6：元组结构体（Tuple Struct）
    #[test]
    fn test_tuple_struct() {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);

        let r = black.0;
        let x = origin.0;

        assert_eq!(r, 0);
        assert_eq!(x, 0);
    }

    // 知识点7：单元结构体（Unit-Like Struct）
    #[test]
    fn test_unit_struct() {
        #[derive(PartialEq, Debug)]
        struct AlwaysEqual;

        let subject = AlwaysEqual;
        let subject2 = AlwaysEqual;

        assert_eq!(subject, subject2);
    }
}

//verbose version
fn build_user_verbose(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// concise version
fn build_user_concise(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
