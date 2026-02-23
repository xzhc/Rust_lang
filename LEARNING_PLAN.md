# Rust 学习路线图

> 基于《The Rust Programming Language》官方书籍
> 版本：Rust 1.90.0+ (2024 Edition)

## 📚 学习资料位置
- 本地路径：`file:///Users/xzh/.rustup/toolchains/stable-aarch64-apple-darwin/share/doc/rust/html/book/`
- 在线版本：https://doc.rust-lang.org/stable/book/
- 命令行查看：`rustup doc --book`

---

## 🎯 学习阶段划分

### 🌱 阶段一：入门基础（第1-3章）
**目标**：搭建环境，编写第一个程序，掌握基本语法

#### 第1章 - Getting Started（入门开始）
- [ ] 1.1 Installation（安装 Rust）
- [ ] 1.2 Hello, World!（第一个程序）
- [ ] 1.3 Hello, Cargo!（包管理工具）

**重点**：
- 安装 Rust 工具链（rustup）
- 理解 Cargo（构建系统和包管理器）
- 编译并运行第一个 Rust 程序

#### 第2章 - Programming a Guessing Game（猜数字游戏）
**项目实战**：通过一个完整的互动游戏学习 Rust 基础

**涵盖知识点**：
- 变量和数据类型
- 用户输入输出
- 随机数生成
- match 表达式
- 循环

#### 第3章 - Common Programming Concepts（通用编程概念）
- [ ] 3.1 Variables and Mutability（变量与可变性）
- [ ] 3.2 Data Types（数据类型）
- [ ] 3.3 Functions（函数）
- [ ] 3.4 Comments（注释）
- [ ] 3.5 Control Flow（控制流）

**重点**：
- 理解 Rust 的变量默认不可变特性
- 标量类型和复合类型
- 函数定义与调用
- if/else、loop、while、for 循环

---

### 🌿 阶段二：核心概念（第4-6章）
**目标**：理解 Rust 最独特的特性，打下坚实基础

#### 第4章 - Understanding Ownership（理解所有权）⭐
**这是 Rust 最重要的概念！**
- [ ] 4.1 What is Ownership?（什么是所有权）
- [ ] 4.2 References and Borrowing（引用与借用）
- [ ] 4.3 The Slice Type（切片类型）

**重点**：
- 所有权三规则
- 借用规则（Borrowing Rules）
- 可变引用与不可变引用
- 字符串切片

#### 第5章 - Structs（结构体）
- [ ] 5.1 Defining and Instantiating Structs（定义结构体）
- [ ] 5.2 An Example Program Using Structs（实例：矩形面积计算）
- [ ] 5.3 Methods（方法语法）

**重点**：
- 定义结构体
- 元组结构体和单元结构体
- 方法和关联函数

#### 第6章 - Enums and Pattern Matching（枚举与模式匹配）
- [ ] 6.1 Defining an Enum（定义枚举）
- [ ] 6.2 The match Control Flow Construct（match 表达式）
- [ ] 6.3 Concise Control Flow with if let（if let 简写）

**重点**：
- Option 枚举（处理空值）
- match 表达式
- if let 和 let...else

---

### 🌳 阶段三：模块系统与标准库（第7-9章）
**目标**：组织代码、使用集合、处理错误

#### 第7章 - Packages, Crates, and Modules（包、箱和模块）
- [ ] 7.1 Packages and Crates（包与箱）
- [ ] 7.2 Control Scope and Privacy with Modules（模块的作用域与隐私）
- [ ] 7.3 Paths for Referring to an Item（路径引用）
- [ ] 7.4 Bringing Paths Into Scope with use（use 关键字）
- [ ] 7.5 Separating Modules into Different Files（模块文件分离）

**重点**：
- 模块系统
- pub 关键字和可见性
- use 关键字导入

#### 第8章 - Common Collections（常用集合）
- [ ] 8.1 Storing Lists of Values with Vectors（向量）
- [ ] 8.2 Storing UTF-8 Encoded Text with Strings（字符串）
- [ ] 8.3 Storing Keys with Associated Values in Hash Maps（哈希映射）

**重点**：
- Vec<T> 动态数组
- String 和 &str 的区别
- HashMap<K, V> 键值对存储

#### 第9章 - Error Handling（错误处理）
- [ ] 9.1 Unrecoverable Errors with panic!（不可恢复错误）
- [ ] 9.2 Recoverable Errors with Result（可恢复错误）
- [ ] 9.3 To panic! or Not to panic!（错误处理策略）

**重点**：
- panic! 宏
- Result<T, E> 枚举
- 错误传播运算符 ?

---

### 🔧 阶段四：中级特性（第10-14章）
**目标**：编写可复用、可测试的代码

#### 第10章 - Generic Types, Traits, and Lifetimes（泛型、Trait和生命周期）
- [ ] 10.1 Generic Data Types（泛型数据类型）
- [ ] 10.2 Defining Shared Behavior with Traits（定义共享行为）
- [ ] 10.3 Validating References with Lifetimes（生命周期）

**重点**：
- 泛型函数和结构体
- Trait 定义与实现
- 生命周期注解

#### 第11章 - Testing（测试）
- [ ] 11.1 How to Write Tests（编写测试）
- [ ] 11.2 Controlling How Tests Are Run（控制测试运行）
- [ ] 11.3 Test Organization（测试组织）

**重点**：
- assert!、assert_eq!、assert_ne! 宏
- 单元测试和集成测试
- 测试组织结构

#### 第12章 - An I/O Project（I/O 项目实战）
**项目**：构建命令行程序（类似 grep）

- [ ] 12.1 Accepting Command Line Arguments（命令行参数）
- [ ] 12.2 Reading a File（读取文件）
- [ ] 12.3 Refactoring to Improve Modularity（重构模块化）
- [ ] 12.4 Adding Functionality with Test Driven Development（TDD）
- [ ] 12.5 Working with Environment Variables（环境变量）
- [ ] 12.6 Redirecting Errors to Standard Error（错误重定向）

**重点**：
- 文件 I/O 操作
- 测试驱动开发
- 错误处理最佳实践

#### 第13章 - Functional Language Features（函数式语言特性）
- [ ] 13.1 Closures（闭包）
- [ ] 13.2 Processing a Series of Items with Iterators（迭代器）
- [ ] 13.3 Improving Our I/O Project（改进 I/O 项目）
- [ ] 13.4 Performance in Loops vs. Iterators（性能对比）

**重点**：
- 闭包语法和捕获
- 迭器适配器
- 函数式编程模式

#### 第14章 - More about Cargo and Crates.io（Cargo 进阶）
- [ ] 14.1 Customizing Builds with Release Profiles（发布配置）
- [ ] 14.2 Publishing a Crate to Crates.io（发布包）
- [ ] 14.3 Cargo Workspaces（工作空间）
- [ ] 14.4 Installing Binaries with cargo install（安装二进制）
- [ ] 14.5 Extending Cargo with Custom Commands（扩展 Cargo）

**重点**：
- Cargo 发布配置
- 发布开源包
- 多项目工作空间

---

### 🚀 阶段五：高级特性（第15-21章）
**目标**：掌握高级主题，构建复杂系统

#### 第15章 - Smart Pointers（智能指针）
- [ ] 15.1 Using Box<T> to Point to Data on the Heap（箱指针）
- [ ] 15.2 Treating Smart Pointers Like Regular References（Deref trait）
- [ ] 15.3 Running Code on Cleanup with the Drop Trait（清理机制）
- [ ] 15.4 Rc<T>, the Reference Counted Smart Pointer（引用计数）
- [ ] 15.5 RefCell<T> and the Interior Mutability Pattern（内部可变性）
- [ ] 15.6 Reference Cycles Can Leak Memory（循环引用）

**重点**：
- 智能指针与引用的区别
- Box、Rc、RefCell 的使用场景
- 内存泄漏风险

#### 第16章 - Fearless Concurrency（无畏并发）
- [ ] 16.1 Using Threads to Run Code Simultaneously（线程）
- [ ] 16.2 Transfer Data Between Threads with Message Passing（消息传递）
- [ ] 16.3 Shared-State Concurrency（共享状态）
- [ ] 16.4 Extensible Concurrency with Send and Sync（Send 和 Sync trait）

**重点**：
- 线程创建与等待
- 通道（channel）通信
- Mutex 互斥锁
- Arc 原子引用计数

#### 第17章 - Async, Await, Futures, and Streams（异步编程）
- [ ] 17.1 Futures and the Async Syntax（Future 和 async 语法）
- [ ] 17.2 Applying Concurrency with Async（异步并发）
- [ ] 17.3 Working With Any Number of Futures（多个 Future）
- [ ] 17.4 Streams: Futures in Sequence（流）
- [ ] 17.5 A Closer Look at the Traits for Async（异步 trait）
- [ ] 17.6 Futures, Tasks, and Threads（运行时模型）

**重点**：
- async/await 语法
- 异步编程模型
- 异步运行时（如 Tokio）

#### 第18章 - Object Oriented Programming Features（面向对象特性）
- [ ] 18.1 Characteristics of Object-Oriented Languages（OOP 特征）
- [ ] 18.2 Using Trait Objects to Abstract over Shared Behavior（trait 对象）
- [ ] 18.3 Implementing an Object-Oriented Design Pattern（OOP 设计模式）

**重点**：
- trait 对象实现动态分发
- 状态模式（State Pattern）

#### 第19章 - Patterns and Matching（模式与匹配）
- [ ] 19.1 All the Places Patterns Can Be Used（模式的使用位置）
- [ ] 19.2 Refutability: Whether a Pattern Might Fail to Match（可反驳性）
- [ ] 19.3 Pattern Syntax（模式语法）

**重点**：
- 模式匹配的完整语法
- 可反驳与不可反驳模式

#### 第20章 - Advanced Features（高级特性）
- [ ] 20.1 Unsafe Rust（不安全 Rust）
- [ ] 20.2 Advanced Traits（高级 trait）
- [ ] 20.3 Advanced Types（高级类型）
- [ ] 20.4 Advanced Functions and Closures（高级函数）
- [ ] 20.5 Macros（宏）

**重点**：
- unsafe 块和裸指针
- 关联类型、默认类型参数
- 宏定义（声明宏和过程宏）

#### 第21章 - Final Project: Building a Multithreaded Web Server（最终项目）
**项目**：构建多线程 Web 服务器

- [ ] 21.1 Building a Single-Threaded Web Server（单线程服务器）
- [ ] 21.2 From Single-Threaded to Multithreaded Server（多线程服务器）
- [ ] 21.3 Graceful Shutdown and Cleanup（优雅关闭）

**重点**：
- TCP 监听与连接
- 线程池实现
- 优雅关闭机制

---

### 📖 附录部分（随时查阅）
- [ ] Appendix A - Keywords（关键字）
- [ ] Appendix B - Operators and Symbols（运算符和符号）
- [ ] Appendix C - Derivable Traits（可派生 trait）
- [ ] Appendix D - Useful Development Tools（开发工具）
- [ ] Appendix E - Editions（版本 editions）
- [ ] Appendix F - Translations（翻译版本）
- [ ] Appendix G - How Rust is Made and "Nightly Rust"（Rust 开发流程）

---

## 💡 学习建议

### 新手友好提示
1. **不要跳过第4章（所有权）** - 这是 Rust 最核心的概念
2. **多动手实践** - 每章的代码示例都要自己敲一遍
3. **理解编译错误** - Rust 的编译器提示非常友好，学会阅读错误信息
4. **完成练习** - 第2章和第12-13章的项目是很好的练手机会
5. **善用文档** - 记得使用 `rustup doc --book` 和 `rustup doc --std`

### 学习节奏建议
- **每日学习时间**：1-2小时
- **阶段一（第1-3章）**：3-5天
- **阶段二（第4-6章）**：5-7天（重点章节，多花时间）
- **阶段三（第7-9章）**：5-7天
- **阶段四（第10-14章）**：7-10天
- **阶段五（第15-21章）**：10-15天

### 实践项目建议
完成每个阶段后，尝试写一些小项目：
- 阶段一后：写一个简单的计算器
- 阶段二后：实现一个简单的待办事项列表
- 阶段三后：写一个文件搜索工具
- 阶段四后：重构你的项目，增加测试
- 阶段五后：参与开源项目或构建自己的应用

---

## 🔗 有用资源
- Rust 官方文档：https://doc.rust-lang.org/
- Rust by Example：https://doc.rust-lang.org/rust-by-example/
- Crates.io（包仓库）：https://crates.io/
- Rust 官方论坛：https://users.rust-lang.org/

---

**开始时间**：_____________
**当前进度**：_____________
**学习笔记**：[创建一个 notes.md 文件记录学习心得]
