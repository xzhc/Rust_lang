//! 第七部分：Option vs Result
//!
//! 【Option<T>】
//! - 表示可能有值或没有值
//! - 用于不涉及"错误"的缺失情况
//! - Some(T) / None
//!
//! 【Result<T, E>】
//! - 表示操作可能成功或失败
//! - 用于需要知道失败原因的情况
//! - Ok(T) / Err(E)
//!
//! 【转换方法】
//! - ok_or(): Option -> Result
//! - ok(): Result -> Option (丢弃错误)
//! - err(): Result -> Option (丢弃值)

/// 演示 Option 和 Result 的区别与转换
pub fn demo() {
    println!("\n=== 7. Option vs Result ===\n");

    demo_difference();
    demo_conversion();
    demo_chain_operations();
}

/// 演示 Option 和 Result 的使用场景区别
fn demo_difference() {
    println!("--- Option vs Result 使用场景 ---");

    // Option: 只是"有或没有"，不需要知道为什么没有
    fn find_user_by_id(id: u32) -> Option<&'static str> {
        match id {
            1 => Some("Alice"),
            2 => Some("Bob"),
            _ => None, // 用户不存在，但不是"错误"
        }
    }

    // Result: 需要知道失败的原因
    fn authenticate_user(username: &str, password: &str) -> Result<&'static str, &'static str> {
        if username.is_empty() {
            return Err("用户名不能为空");
        }
        if password.len() < 6 {
            return Err("密码太短");
        }
        if username == "admin" && password == "123456" {
            Ok("认证成功")
        } else {
            Err("用户名或密码错误")
        }
    }

    // Option 使用示例
    match find_user_by_id(1) {
        Some(user) => println!("找到用户: {}", user),
        None => println!("用户不存在"),
    }

    // Result 使用示例
    match authenticate_user("admin", "123456") {
        Ok(msg) => println!("认证: {}", msg),
        Err(e) => println!("认证失败: {}", e),
    }

    match authenticate_user("admin", "123") {
        Ok(msg) => println!("认证: {}", msg),
        Err(e) => println!("认证失败: {}", e),
    }
}

/// 演示 Option 和 Result 之间的转换
fn demo_conversion() {
    println!("\n--- Option 与 Result 转换 ---");

    // Option -> Result
    let some_value: Option<i32> = Some(42);
    let result: Result<i32, &str> = some_value.ok_or("没有值");
    println!("Some -> Result: {:?}", result);

    let none_value: Option<i32> = None;
    let result: Result<i32, &str> = none_value.ok_or("没有值");
    println!("None -> Result: {:?}", result);

    // Result -> Option (丢弃错误信息)
    let ok_result: Result<i32, &str> = Ok(42);
    let option: Option<i32> = ok_result.ok();
    println!("Ok -> Option: {:?}", option);

    let err_result: Result<i32, &str> = Err("错误");
    let option: Option<i32> = err_result.ok();
    println!("Err -> Option: {:?}", option);

    // Result -> Option (只保留错误)
    let err_result: Result<i32, &str> = Err("错误信息");
    let err_option: Option<&str> = err_result.err();
    println!("Err -> Option (错误): {:?}", err_option);

    // 使用 ok_or_else 延迟创建错误
    let value: Option<i32> = None;
    let result = value.ok_or_else(|| {
        println!("正在创建错误信息...");
        "动态生成的错误"
    });
    println!("ok_or_else: {:?}", result);
}

/// 演示链式操作
fn demo_chain_operations() {
    println!("\n--- 链式操作 ---");

    // Option 链式操作
    let result = Some("42")
        .map(|s| s.parse::<i32>()) // Option<Result<i32, _>>
        .transpose()               // Result<Option<i32>, _>
        .unwrap_or(None);          // Option<i32>

    println!("Option 链式结果: {:?}", result);

    // Result 链式操作
    let result = "42"
        .parse::<i32>()
        .ok()           // Option<i32>
        .map(|n| n * 2) // Option<i32>
        .ok_or("解析失败"); // Result<i32, &str>

    println!("Result 链式结果: {:?}", result);

    // 实际应用：从环境变量读取并解析
    fn get_port() -> Result<u16, String> {
        std::env::var("PORT")
            .ok()  // Option<String>
            .map(|s| s.parse::<u16>())  // Option<Result<u16, _>>
            .transpose()  // Result<Option<u16>, _>
            .map(|opt| opt.unwrap_or(8080))  // Result<u16, _>
            .map_err(|e| format!("解析端口失败: {}", e))
    }

    match get_port() {
        Ok(port) => println!("端口: {}", port), // 默认 8080
        Err(e) => println!("错误: {}", e),
    }

    // transpose() 方法：翻转 Option<Result<T, E>> 为 Result<Option<T>, E>
    let opt_res: Option<Result<i32, &str>> = Some(Ok(42));
    let res_opt: Result<Option<i32>, &str> = opt_res.transpose();
    println!("transpose: {:?}", res_opt);

    let opt_res: Option<Result<i32, &str>> = Some(Err("错误"));
    let res_opt: Result<Option<i32>, &str> = opt_res.transpose();
    println!("transpose (err): {:?}", res_opt);

    let opt_res: Option<Result<i32, &str>> = None;
    let res_opt: Result<Option<i32>, &str> = opt_res.transpose();
    println!("transpose (none): {:?}", res_opt);
}
