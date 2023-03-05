### 内容 
如何使用模块在文件和文件夹中组织代码
模块成员的可见性
如何在 crates.io 上发布你的代码

#### Rust 中的模块化编程
Rust 项目的代码组织包含以下三个基本概念:
- Package(包)
- Crate(箱)
- Module(模块)

##### Package
Package 用于管理一个或多个 Crate. 创建一个 Package 的方式是使用 cargo new

当我们输入命令时, Cargo 创建了一个目录以及一个 Cargo.toml 文件, 这就是一个 Package. 默认情况下, src/main.rs 是与 Package 同名的二进制 Crate 的入口文件, 因此我们可以说我们现在有一个 my-project Package 以及一个二进制 my-project Crate. 同样, 如果在创建 Package 的时候带上 --lib, 那么 src/lib.rs 将是它的 Crate 入口文件, 且它是一个库 Crate.

如果我们的 src 目录中同时包含 main.rs 和 lib.rs, 那么我们将在这个 Package 中同时得到一个二进制 Crate 和一个库 Crate, 这在开发一些基础库时非常有用, 例如你使用 Rust 中实现了一个 MD5 函数, 你既希望这个 MD5 函数能作为库被别人引用, 又希望你能获得一个可以进行 MD5 计算的命令行工具: 那就同时添加 main.rs 和 lib.rs 吧!

##### Crate
Crate 是 Rust 的最小编译单元, 即 Rust 编译器是以 Crate 为最小单元进行编译的. Crate 在一个范围内将相关的功能组合在一起, 并最终通过编译生成一个二进制或库文件. 例如, 我们在上一章中实现的猜数字游戏就使用了 rand 依赖, 这个 rand 就是一个 Crate.

##### Module
Module 允许我们将一个 Crate 中的代码组织成独立的代码块, 以便于增强可读性和代码复用. 同时, Module 还控制代码的可见性, 即将代码分为公开代码和私有代码. 公开代码可以在项目外被使用, 私有代码则只有项目内部的代码才可以访问. 定义一个模块最基本的方式是使用 mod 关键字:


#### 模块可见性


#### 结构体可见性

