// clippy1.rs
//
// Clippy 工具是一系列 lint 检查的集合，用于分析你的代码，
// 以便发现常见错误并改进你的 Rust 代码。
//
// 在这些练习中，当存在 clippy 警告时代码将编译失败。
// 查看输出中 clippy 的建议来解决此练习。
//
// 执行 `rustlings hint clippy1` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

use std::f32;

fn main() {
    let pi = 3.14f32;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
