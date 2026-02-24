//! 第八部分：实用的 Result 和 Option 方法
//!
//! Result 和 Option 提供了丰富的组合器方法
//! 可以让我们以函数式的方式处理错误

/// 演示 Result 和 Option 的实用方法
pub fn demo() {
    println!("\n=== 8. Result/Option 实用方法 ===\n");

    demo_result_methods();
    demo_option_methods();
    demo_combinators();
}

/// Result 的实用方法
fn demo_result_methods() {
    println!("--- Result 方法 ---");

    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("出错了");

    // 检查状态
    println!("is_ok: {}", success.is_ok());
    println!("is_err: {}", failure.is_err());
    println!("is_ok_and: {}", success.is_ok_and(|n| n > 0));
    println!("is_err_and: {}", failure.is_err_and(|e| e.contains("错")));

    // map - 转换成功值
    let doubled = success.map(|n| n * 2);
    println!("map: {:?}", doubled);

    // map_err - 转换错误值
    let mapped_err = failure.map_err(|e| format!("错误: {}", e));
    println!("map_err: {:?}", mapped_err);

    // map_or - 转换成功值，失败时返回默认值
    let value = success.map_or(0, |n| n * 2);
    println!("map_or (success): {}", value);

    let value = failure.map_or(0, |n| n * 2);
    println!("map_or (failure): {}", value);

    // map_or_else - 使用闭包提供默认值
    let value = failure.map_or_else(
        |e| {
            println!("  错误发生: {}", e);
            -1
        },
        |n| n * 2,
    );
    println!("map_or_else: {}", value);

    // and / or
    let another_ok: Result<i32, &str> = Ok(100);
    let another_err: Result<i32, &str> = Err("另一个错误");

    println!("Ok and Ok: {:?}", success.and(another_ok)); // 返回第二个
    println!("Ok and Err: {:?}", success.and(another_err)); // 返回 Err
    println!("Err and Ok: {:?}", failure.and(another_ok)); // 返回 Err

    println!("Ok or Ok: {:?}", success.or(another_ok)); // 返回第一个 Ok
    println!("Err or Ok: {:?}", failure.or(another_ok)); // 返回 Ok
    println!("Err or Err: {:?}", failure.or(another_err)); // 返回第二个 Err

    // and_then (flat map)
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("除数不能为零".to_string())
        } else {
            Ok(a / b)
        }
    }

    let result = Ok(100).and_then(|n| divide(n, 2));
    println!("and_then (divide by 2): {:?}", result);

    let result = Ok(100).and_then(|n| divide(n, 0));
    println!("and_then (divide by 0): {:?}", result);

    // or_else
    let result: Result<i32, &str> = failure.or_else(|e| {
        println!("  第一个错误: {}, 提供备选", e);
        Ok(0)
    });
    println!("or_else: {:?}", result);

    // unwrap variants
    println!("\n--- unwrap 变体 ---");
    let ok_val: Result<i32, &str> = Ok(42);

    println!("unwrap: {}", ok_val.unwrap());
    println!("expect: {}", ok_val.expect("应该有值"));
    println!("unwrap_or: {}", failure.unwrap_or(0));
    println!("unwrap_or_default: {}", failure.unwrap_or_default());
    println!("unwrap_or_else: {}", failure.unwrap_or_else(|e| {
        println!("  处理错误: {}", e);
        -1
    }));

    // inspect - 执行副作用但不修改值
    let result: Result<i32, &str> = Ok(42).inspect(|&n| println!("  inspect: 值是 {}", n));
    println!("inspect 返回: {:?}", result);

    // copied / cloned
    let ok_ref: Result<&i32, &str> = Ok(&42);
    let ok_copied: Result<i32, &str> = ok_ref.copied();
    println!("copied: {:?}", ok_copied);
}

/// Option 的实用方法
fn demo_option_methods() {
    println!("\n--- Option 方法 ---");

    let some: Option<i32> = Some(42);
    let none: Option<i32> = None;

    // 检查
    println!("is_some: {}", some.is_some());
    println!("is_none: {}", none.is_none());
    println!("is_some_and: {}", some.is_some_and(|n| n > 0));

    // map
    let doubled = some.map(|n| n * 2);
    println!("map: {:?}", doubled);

    // map_or
    let value = some.map_or(0, |n| n * 2);
    println!("map_or: {}", value);

    let value = none.map_or(0, |n| n * 2);
    println!("map_or (none): {}", value);

    // and / or
    let another: Option<i32> = Some(100);
    println!("Some and Some: {:?}", some.and(another));
    println!("Some or Some: {:?}", some.or(another));
    println!("None or Some: {:?}", none.or(another));

    // and_then
    fn checked_sqrt(n: i32) -> Option<i32> {
        if n >= 0 {
            Some((n as f64).sqrt() as i32)
        } else {
            None
        }
    }

    let result = Some(16).and_then(checked_sqrt);
    println!("and_then sqrt(16): {:?}", result);

    let result = Some(-4).and_then(checked_sqrt);
    println!("and_then sqrt(-4): {:?}", result);

    // filter
    let result = some.filter(|&n| n > 20);
    println!("filter (>20): {:?}", result);

    let result = some.filter(|&n| n > 50);
    println!("filter (>50): {:?}", result);

    // zip
    let a = Some(1);
    let b = Some("hello");
    println!("zip: {:?}", a.zip(b));

    let a = Some(1);
    let b: Option<&str> = None;
    println!("zip with None: {:?}", a.zip(b));

    // flatten
    let nested: Option<Option<i32>> = Some(Some(42));
    println!("flatten: {:?}", nested.flatten());

    let nested: Option<Option<i32>> = Some(None);
    println!("flatten (Some(None)): {:?}", nested.flatten());

    // transpose (Option<Result<T, E>> -> Result<Option<T>, E>)
    let opt_res: Option<Result<i32, &str>> = Some(Ok(42));
    let transposed: Result<Option<i32>, &str> = opt_res.transpose();
    println!("transpose: {:?}", transposed);
}

/// 组合器实战
fn demo_combinators() {
    println!("\n--- 组合器实战 ---");

    // 实际应用：解析配置
    #[derive(Debug)]
    struct Config {
        host: String,
        port: u16,
    }

    fn parse_config(input: &str) -> Option<Config> {
        let parts: Vec<&str> = input.split(':').collect();

        let host = parts.get(0)?.to_string();
        let port = parts.get(1)?.parse().ok()?;

        Some(Config { host, port })
    }

    // 使用链式调用处理
    let config = parse_config("localhost:8080");
    println!("配置: {:?}", config);

    let config = parse_config("invalid");
    println!("无效配置: {:?}", config);

    // 使用 Result 和 ? 的版本
    fn parse_config_result(input: &str) -> Result<Config, String> {
        let parts: Vec<&str> = input.split(':').collect();

        let host = parts.get(0).ok_or("缺少主机")?.to_string();
        let port = parts.get(1).ok_or("缺少端口")?.parse().map_err(|e| format!("端口解析失败: {}", e))?;

        Ok(Config { host, port })
    }

    match parse_config_result("localhost:8080") {
        Ok(config) => println!("Result 版本配置: {:?}", config),
        Err(e) => println!("错误: {}", e),
    }

    match parse_config_result("localhost") {
        Ok(config) => println!("Result 版本配置: {:?}", config),
        Err(e) => println!("错误: {}", e),
    }
}
