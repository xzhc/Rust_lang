//! 第二部分：可恢复错误 - Result<T, E>
//!
//! 【概念】
//! Result 是一个泛型枚举，用于可能失败的操作：
//!
//!   enum Result<T, E> {
//!       Ok(T),   // 成功，包含返回值 T
//!       Err(E),  // 失败，包含错误值 E
//!   }
//!
//! 【优点】
//! - 显式处理错误，不会意外忽略
//! - 错误信息可以自定义类型
//! - 可以将错误向上传播

/// 演示 Result 的基本用法 - 使用 match
pub fn demo() {
    println!("\n=== 2. Result 基础用法 ===\n");

    // 创建成功和失败的 Result
    let success_result: Result<i32, &str> = Ok(42);
    let error_result: Result<i32, &str> = Err("出错了");

    // 使用 match 处理 Result
    println!("--- 使用 match 处理 ---");

    match success_result {
        Ok(value) => println!("成功! 值为: {}", value),
        Err(e) => println!("错误: {}", e),
    }

    match error_result {
        Ok(value) => println!("成功! 值为: {}", value),
        Err(e) => println!("错误: {}", e),
    }
}

/// 演示不同的 Result 创建方式
pub fn demo_creation() {
    println!("\n--- Result 创建方式 ---");

    // 直接使用 Ok 和 Err
    let ok_val: Result<i32, String> = Ok(100);
    let err_val: Result<i32, String> = Err("失败".to_string());

    println!("Ok 值: {:?}", ok_val);
    println!("Err 值: {:?}", err_val);
}

/// 演示使用 if let 处理 Result
pub fn demo_if_let() {
    println!("\n--- 使用 if let 处理 ---");

    let result: Result<&str, &str> = Ok("你好");

    // 只关心成功的情况
    if let Ok(msg) = result {
        println!("收到消息: {}", msg);
    }

    // 只关心错误的情况
    let error: Result<&str, &str> = Err("连接失败");
    if let Err(e) = error {
        println!("发生错误: {}", e);
    }
}

/// 演示使用 match 守卫
pub fn demo_match_guard() {
    println!("\n--- 使用 match 守卫 ---");

    fn check_number(result: Result<i32, &str>) {
        match result {
            Ok(n) if n > 50 => println!("大数字: {}", n),
            Ok(n) => println!("小数字: {}", n),
            Err(e) => println!("错误: {}", e),
        }
    }

    check_number(Ok(100));
    check_number(Ok(30));
    check_number(Err("无效"));
}
