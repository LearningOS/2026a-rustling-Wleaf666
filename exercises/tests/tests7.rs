// tests7.rs
//
// 在构建包时，有些依赖项既无法在 `Cargo.toml` 中导入，也无法直接链接；
// 有些预处理过程从代码生成到配置特定包各不相同。
//
// Cargo 的目的并不是取代其他构建工具，而是通过名为 `build.rs` 的自定义构建脚本与它们集成。
// 该文件通常放置在项目的根目录下，而在本题中则位于该练习的同一目录下。
//
// 它可以用于：
// - 构建捆绑的 C 库。
// - 在宿主系统上查找 C 库。
// - 根据规范生成 Rust 模块。
// - 为 crate 执行所需的任何平台特定配置。
//
// 在进行配置时，我们可以在构建脚本中通过 `println!` 告诉 Cargo 执行某些指令。通用格式为：
//
//     println!("cargo:{}", your_command_in_string);
//
// 有关构建脚本的更多信息，请参阅 Cargo 官方文档：
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// 在本练习中，我们查找一个环境变量并期望它落在一定范围内。你可以查看测试用例以了解细节。
//
// 你不应修改此文件。修改同目录下的 `build.rs` 以通过此练习。
//
// 执行 `rustlings hint tests7` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let s = std::env::var("TEST_FOO").unwrap();
        let e: u64 = s.parse().unwrap();
        assert!(timestamp >= e && timestamp < e + 10);
    }
}
