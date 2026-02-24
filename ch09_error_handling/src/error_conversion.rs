//! 第五部分：错误类型转换 - From trait
//!
//! 【概念】
//! ? 运算符会自动调用 From::from() 转换错误类型
//! 这允许函数返回统一的错误类型
//!
//! 【好处】
//! - 函数可以返回统一的错误类型
//! - 调用者不需要处理多种错误类型
//! - 错误信息更加结构化

use std::io;
use std::num::ParseIntError;

/// 自定义错误类型
/// 使用枚举来统一表示不同来源的错误
#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
    Custom(String),
}

/// 为 AppError 实现 Display trait（用于打印错误信息）
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO 错误: {}", e),
            AppError::Parse(e) => write!(f, "解析错误: {}", e),
            AppError::Custom(msg) => write!(f, "自定义错误: {}", msg),
        }
    }
}

/// 实现 std::error::Error trait
impl std::error::Error for AppError {}

/// 实现 From<io::Error> - 允许 ? 自动转换 io::Error
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

/// 实现 From<ParseIntError> - 允许 ? 自动转换 ParseIntError
impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::Parse(error)
    }
}

/// 演示错误类型转换
pub fn demo() {
    println!("\n=== 5. 错误类型转换 ===\n");

    demo_basic_conversion();
    demo_unified_error();
}

/// 基本的错误转换示例
fn demo_basic_conversion() {
    println!("--- 基本错误转换 ---");

    // 这个函数可能产生 ParseIntError
    // 但通过 ? 运算符自动转换为 AppError
    fn parse_and_double(input: &str) -> Result<i32, AppError> {
        let num: i32 = input.parse()?; // ParseIntError -> AppError
        Ok(num * 2)
    }

    match parse_and_double("21") {
        Ok(n) => println!("21 * 2 = {}", n),
        Err(e) => println!("错误: {:?}", e),
    }

    match parse_and_double("abc") {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("解析 'abc' 失败: {:?}", e),
    }
}

/// 统一错误类型处理多种错误源
fn demo_unified_error() {
    println!("\n--- 统一错误类型 ---");

    /// 一个函数可能产生多种类型的错误
    /// 使用 AppError 统一处理
    fn read_and_parse(filename: &str) -> Result<i32, AppError> {
        // 读取文件 - 可能产生 io::Error
        let content = std::fs::read_to_string(filename)?; // io::Error -> AppError

        // 解析数字 - 可能产生 ParseIntError
        let num: i32 = content.trim().parse()?; // ParseIntError -> AppError

        Ok(num)
    }

    // 测试文件不存在的情况（io::Error）
    match read_and_parse("不存在的数字文件.txt") {
        Ok(n) => println!("读取的数字: {}", n),
        Err(e) => println!("失败: {}", e), // AppError::Io
    }

    // 创建一个包含无效数字的文件来测试解析错误
    println!("\n创建测试文件...");
    std::fs::write("test_invalid_number.txt", "不是数字").unwrap();

    match read_and_parse("test_invalid_number.txt") {
        Ok(n) => println!("读取的数字: {}", n),
        Err(e) => println!("失败: {}", e), // AppError::Parse
    }

    // 清理测试文件
    std::fs::remove_file("test_invalid_number.txt").ok();
}
