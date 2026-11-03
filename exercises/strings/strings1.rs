// strings1.rs
//
// 在不修改函数签名的情况下让代码通过编译！
//
// 执行 `rustlings hint strings1` 或在 watch 模式下使用 `hint` 子命令以获取提示。

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue".to_string()
}
