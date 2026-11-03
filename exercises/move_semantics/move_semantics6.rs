// move_semantics6.rs
//
// 除了添加或删除引用符号（&）外，不能修改任何内容。
//
// 执行 `rustlings hint move_semantics6` 或在 watch 模式下使用 `hint` 子命令以获取提示。

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
