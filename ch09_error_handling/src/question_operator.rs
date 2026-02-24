//! 第四部分：传播错误 - ? 运算符
//!
//! 【概念】
//! ? 运算符是错误传播的简写方式：
//! - 如果 Result 是 Ok(v)，返回 v
//! - 如果 Result 是 Err(e)，立即返回 Err(e)
//!
//! 【要求】
//! ? 只能在返回 Result 或 Option 的函数中使用
//! 错误类型必须兼容（可以实现 From trait 转换）

use std::fs::File;
use std::io::{self, Read};

/// 演示 ? 运算符的用法
pub fn demo() {
    println!("\n=== 4. ? 运算符演示 ===\n");

    //demo_comparison();
    demo_option_question();
}

/// 对比传统 match 和 ? 运算符
fn demo_comparison() {
    println!("--- 传统 match vs ? 运算符 ---\n");

    // 使用 match 版本的函数
    match read_username_from_file_match() {
        Ok(username) => println!("match 版本读取成功: {}", username),
        Err(e) => println!("match 版本读取失败: {}", e),
    }

    // 使用 ? 运算符版本
    match read_username_from_file_question() {
        Ok(username) => println!("? 版本读取成功: {}", username),
        Err(e) => println!("? 版本读取失败: {}", e),
    }

    // 链式调用版本
    match read_username_from_file_chain() {
        Ok(username) => println!("链式调用版本读取成功: {}", username),
        Err(e) => println!("链式调用版本读取失败: {}", e),
    }

    // 标准库版本
    match read_username_from_file_std() {
        Ok(username) => println!("标准库版本读取成功: {}", username),
        Err(e) => println!("标准库版本读取失败: {}", e),
    }
}

/// 传统方式：使用 match 传播错误
/// 代码较长，但清晰展示了错误传播的过程
fn read_username_from_file_match() -> Result<String, io::Error> {
    let file_result = File::open("username.txt");

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // 遇到错误，提前返回
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/// 简洁方式：使用 ? 运算符
/// ? 自动处理：成功则继续，失败则提前返回 Err
fn read_username_from_file_question() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?; // 失败时自动返回 Err
    let mut username = String::new();
    file.read_to_string(&mut username)?; // 失败时自动返回 Err
    Ok(username)
}

/// 更简洁：链式调用
/// 多个 ? 运算符可以链式使用
fn read_username_from_file_chain() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

/// 最简洁：使用标准库函数
/// std::fs::read_to_string 已经封装了打开文件和读取的操作
fn read_username_from_file_std() -> Result<String, io::Error> {
    std::fs::read_to_string("username.txt")
}

/// 演示 ? 运算符在 Option 类型中的使用
fn demo_option_question() {
    println!("\n--- ? 运算符在 Option 中的使用 ---");

    // ? 运算符同样适用于 Option
    // 如果是 None，提前返回 None
    fn get_last_char(s: &str) -> Option<char> {
        let last = s.chars().last()?; // 如果是 None，返回 None
        Some(last)
    }

    fn get_first_n_chars(s: &str, n: usize) -> Option<String> {
        let char_at_n = s.chars().nth(n)?; // 如果索引越界，返回 None
        Some(s[..char_at_n.len_utf8() * n].to_string())
    }

    println!("'hello' 最后字符: {:?}", get_last_char("hello"));
    println!("空字符串最后字符: {:?}", get_last_char(""));
    println!("'hello' 前3个字符: {:?}", get_first_n_chars("hello", 3));
    println!("'hi' 前5个字符: {:?}", get_first_n_chars("hi", 5));
}
