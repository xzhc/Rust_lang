//! 第9章 - 错误处理 (Error Handling) 学习演示
//!
//! Rust 将错误分为两大类：
//! 1. 不可恢复错误 (Unrecoverable Errors) - 使用 panic!
//! 2. 可恢复错误 (Recoverable Errors) - 使用 Result<T, E>
//!
//! 项目结构：
//! - panic_demo: panic! 不可恢复错误
//! - result_basic: Result<T, E> 基础用法
//! - file_handling: 文件操作中的错误处理
//! - question_operator: ? 运算符传播错误
//! - error_conversion: From trait 错误类型转换
//! - best_practices: panic! vs Result 最佳实践
//! - option_vs_result: Option 与 Result 的区别与转换
//! - result_methods: Result/Option 实用方法

// 声明模块
mod best_practices;
mod error_conversion;
mod file_handling;
mod option_vs_result;
mod panic_demo;
mod question_operator;
mod result_basic;
mod result_methods;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     第9章 - 错误处理 (Error Handling) 学习演示             ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    // 1. panic! 不可恢复错误
    //panic_demo::demo();
    //panic_demo::demo_panic_unwind();

    // // 2. Result<T, E> 基础用法
    //result_basic::demo();
    //result_basic::demo_creation();
    //result_basic::demo_if_let();
    //result_basic::demo_match_guard();

    // // 3. 文件操作中的错误处理
    //file_handling::demo();

    // // 4. ? 运算符传播错误
    //question_operator::demo();

    // // 5. From trait 错误类型转换
    //error_conversion::demo();

    // // 6. 最佳实践
    //best_practices::demo();

    // // 7. Option vs Result
    //option_vs_result::demo();

    // // 8. Result/Option 实用方法
    result_methods::demo();

    // // 学习总结
    // print_summary();
}

/// 打印学习总结
fn print_summary() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                        学习总结                             ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    println!("\n📚 核心概念：");
    println!("   1. panic! - 用于不可恢复错误，会终止程序");
    println!("   2. Result<T, E> - 用于可恢复错误，需要显式处理");

    println!("\n🔧 关键技术：");
    println!("   3. ? 运算符 - 简化错误传播，自动返回 Err");
    println!("   4. From trait - 允许错误类型自动转换");
    println!("   5. 组合器方法 - map, and_then, unwrap_or 等");

    println!("\n💡 最佳实践：");
    println!("   - 默认使用 Result，只有确实无法恢复时才用 panic!");
    println!("   - Option 用于表示可能缺失的值，Result 用于可能失败的操作");
    println!("   - 为自定义错误类型实现 From trait 和 std::error::Error");

    println!("\n📖 推荐阅读：");
    println!("   - 官方文档: rustup doc --book (第9章)");
    println!("   - std::error::Error trait 文档");
    println!("   - anyhow / thiserror crate (生产级错误处理)");

    println!("\n🎯 恭喜你完成第9章的学习！\n");
}
