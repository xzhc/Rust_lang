//! 第一部分：不可恢复错误 - panic!
//!
//! 【概念】
//! panic! 用于程序遇到无法处理的错误时，立即终止程序。
//! 它会展开栈（unwind）并清理数据，或直接中止（abort）。
//!
//! 【何时使用】
//! - 程序处于坏状态，无法继续执行
//! - 外部代码传入无效值
//! - 测试中的断言失败
//!
//! 【注意】
//! panic! 会打印错误信息和调用栈，然后退出程序

/// 演示 panic! 的基本用法
pub fn demo() {
    println!("\n=== 1. panic! 演示 ===\n");

    // 直接调用 panic! 会导致程序崩溃
    // 取消下面的注释可以看到 panic 效果：
    // panic!("程序崩溃了！");

    // panic! 通常在运行时检测到问题时使用
    let v = vec![1, 2, 3];

    // 访问越界索引会触发 panic（Rust 的内置检查）
    // 取消注释会触发：index out of bounds: the len is 3 but the index is 99
    // let element = v[99];

    println!("向量 v: {:?}", v);
    println!("提示：取消注释中的 panic! 代码可以看到崩溃效果");
}

/// 演示 panic! 的展开（unwind）行为
/// 注意：这个函数会触发 panic，仅用于演示
#[allow(dead_code)]
pub fn demo_panic_unwind() {
    struct Droppable;

    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("Droppable 正在被清理（栈展开）");
        }
    }

    let _d = Droppable;

    // 当 panic 发生时，Rust 会沿着调用栈清理所有值
    // Droppable 的 drop 方法会被调用
    panic!("触发 panic，观察栈展开");

    // println!("（此演示未触发 panic，Droppable 会在函数结束时正常清理）");
}
