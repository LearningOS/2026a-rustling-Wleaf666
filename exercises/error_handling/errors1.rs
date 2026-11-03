// errors1.rs
//
// 如果传入空字符串，该函数将拒绝生成打印在胸牌上的文本。如果它能解释问题出在哪里，
// 而不是仅仅有时返回 `None`，那就更好了。幸运的是，Rust 有一个类似于 `Option` 的结构
// `Result`，可以用来表达错误条件。让我们用上它！
//
// 执行 `rustlings hint errors1` 或在 watch 模式下使用 `hint` 子命令以获取提示。

pub fn generate_nametag_text(name: String) -> Result<String,String>{
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".to_string())
    } else {
        Ok(format!("Hi! My name is {}", name).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
