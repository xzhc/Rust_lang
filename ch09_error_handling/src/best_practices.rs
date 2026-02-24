//! 第六部分：何时使用 panic! vs Result
//!
//! 【使用 panic! 的场景】
//! 1. 示例代码、原型开发
//! 2. 测试代码
//! 3. 比编译器知道更多信息时
//! 4. 程序处于无效状态，无法继续
//!
//! 【使用 Result 的场景】
//! 1. 预期可能失败的操作（文件I/O、网络请求等）
//! 2. 调用者需要处理错误
//! 3. 库代码（给使用者选择权）
//!
//! 【最佳实践】
//! - 默认使用 Result
//! - 只有在真正"不可恢复"时才用 panic!

/// 演示错误处理的最佳实践
pub fn demo() {
    println!("\n=== 6. 最佳实践 ===\n");

    demo_safe_unwrap();
    demo_default_values();
    demo_validating_inputs();
}

/// 安全使用 unwrap/expect 的场景
fn demo_safe_unwrap() {
    println!("--- 安全使用 unwrap/expect ---");

    let numbers = vec![1, 2, 3, 4, 5];

    // 场景1：我们确定有元素，可以用 expect
    // 但最好加上注释说明为什么安全
    let first = numbers.first().expect("numbers 已知非空，因为我们刚创建了它");
    println!("第一个数字: {}", first);

    // 场景2：更好的方式 - 使用模式匹配
    if let Some(first) = numbers.first() {
        println!("安全地获取第一个: {}", first);
    }

    // 场景3：测试代码中可以使用 unwrap
    // 因为测试失败本来就应该终止
}

/// 使用默认值代替 panic
fn demo_default_values() {
    println!("\n--- 使用默认值 ---");

    let maybe_number: Option<i32> = None;

    // unwrap_or - 提供简单的默认值
    let number = maybe_number.unwrap_or(0);
    println!("unwrap_or 默认值: {}", number);

    // unwrap_or_else - 使用闭包计算默认值（延迟计算）
    let number = maybe_number.unwrap_or_else(|| {
        println!("执行闭包计算默认值...");
        42
    });
    println!("unwrap_or_else 计算后的值: {}", number);

    // unwrap_or_default - 使用类型的 Default 值
    let number: i32 = maybe_number.unwrap_or_default();
    println!("unwrap_or_default: {}", number); // i32 的默认值是 0

    // 检查是否为 None（不使用 unwrap_none，因为它需要 nightly 特性）
    let definitely_none: Option<i32> = None;
    if definitely_none.is_none() {
        println!("确认是 None");
    }

    // 或 (or) 操作 - 提供备选 Option
    let result = maybe_number.or(Some(100));
    println!("or 提供备选: {:?}", result);

    // 或 else (or_else) - 使用闭包创建备选
    let result = maybe_number.or_else(|| Some(200));
    println!("or_else 闭包备选: {:?}", result);
}

/// 验证输入并在无效时返回错误
fn demo_validating_inputs() {
    println!("\n--- 验证输入 ---");

    /// 验证年龄（应该在合理范围内）
    fn validate_age(age: i32) -> Result<i32, String> {
        if age < 0 {
            Err("年龄不能为负数".to_string())
        } else if age > 150 {
            Err("年龄不能超过 150".to_string())
        } else {
            Ok(age)
        }
    }

    /// 验证用户名（非空且长度合理）
    fn validate_username(name: &str) -> Result<String, String> {
        let name = name.trim();
        if name.is_empty() {
            Err("用户名不能为空".to_string())
        } else if name.len() > 20 {
            Err("用户名不能超过 20 个字符".to_string())
        } else {
            Ok(name.to_string())
        }
    }

    // 测试验证
    match validate_age(25) {
        Ok(age) => println!("有效年龄: {}", age),
        Err(e) => println!("年龄无效: {}", e),
    }

    match validate_age(-5) {
        Ok(age) => println!("有效年龄: {}", age),
        Err(e) => println!("年龄无效: {}", e),
    }

    match validate_username("  alice  ") {
        Ok(name) => println!("有效用户名: '{}'", name),
        Err(e) => println!("用户名无效: {}", e),
    }

    match validate_username("   ") {
        Ok(name) => println!("有效用户名: '{}'", name),
        Err(e) => println!("用户名无效: {}", e),
    }
}

/// 演示何时应该使用 panic!
#[allow(dead_code)]
pub fn demo_when_to_panic() {
    // 这些情况下可以考虑使用 panic!:

    // 1. 恶意的输入或破坏性数据结构
    // 例如：访问数组越界是 bug，应该 panic

    // 2. 程序员错误（不是用户错误）
    // 例如：传递了无效参数给函数，而文档说明不允许

    // 3. 初始化失败
    // 例如：必要的配置文件缺失，程序无法运行

    // 示例：函数期望非空向量
    fn sum_positive_numbers(numbers: &[i32]) -> i32 {
        // 如果约定这个函数只接受非空切片，可以用 assert
        assert!(!numbers.is_empty(), "numbers 不能为空");

        numbers.iter().filter(|&&n| n > 0).sum()
    }

    let nums = vec![1, -2, 3, 4];
    println!("正数之和: {}", sum_positive_numbers(&nums));

    // sum_positive_numbers(&[]); // 这会 panic
}
