//! 第三部分：实际例子 - 文件操作
//!
//! 文件操作是最常见的可能失败的操作之一
//! File::open 返回 Result<File, io::Error>

use std::fs::File;

/// 演示文件操作中的错误处理
pub fn demo() {
    println!("\n=== 3. 文件操作中的错误处理 ===\n");

    //demo_match_handling();
    //demo_unwrap_expect();
    demo_error_kind();
}

/// 方法1：使用 match 处理
fn demo_match_handling() {
    println!("--- 方法1：使用 match 处理 ---");

    let file_result = File::open("不存在的文件.txt");

    // 使用 match 优雅地处理成功和失败两种情况
    let _file = match file_result {
        Ok(file) => {
            println!("文件打开成功!");
            Some(file)
        }
        Err(error) => {
            println!("文件打开失败: {}", error);
            None
        }
    };
}

/// 方法2：使用 unwrap() 和 expect()
fn demo_unwrap_expect() {
    println!("\n--- 方法2：unwrap() 和 expect() ---");

    // unwrap(): 成功返回值，失败则 panic
    // 不推荐在生产代码中使用，除非你确定不会失败
    //let file = File::open("不存在的文件.txt").unwrap(); // 会 panic!

    // expect(): 类似 unwrap，但可以自定义错误信息
    let file = File::open("不存在的文件.txt").expect("无法打开文件"); // 会 panic 并显示自定义信息

    println!("unwrap() 和 expect() 在失败时会 panic");
    println!("建议：仅在原型开发或确定不会失败时使用");
    println!("生产代码中应使用 match 或 ? 运算符");
}

/// 方法3：使用 error.kind() 获取更详细的错误信息
fn demo_error_kind() {
    println!("\n--- 方法3：处理不同类型的错误 ---");

    use std::io::ErrorKind;

    let file_result = File::open("不存在的文件.txt");

    let _file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("错误：文件不存在");
                // 这里可以尝试创建文件
                match File::create("不存在的文件.txt") {
                    Ok(fc) => {
                        println!("已创建新文件");
                        fc
                    }
                    Err(e) => {
                        println!("创建文件失败: {}", e);
                        return;
                    }
                }
            }
            ErrorKind::PermissionDenied => {
                println!("错误：权限不足");
                return;
            }
            _ => {
                println!("错误：{}", error);
                return;
            }
        },
    };
}
