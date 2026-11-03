// strings2.rs
//
// 在不修改函数签名的情况下让代码通过编译！
//
// 执行 `rustlings hint strings2` 或在 watch 模式下使用 `hint` 子命令以获取提示。

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
