// generics1.rs
//
// 这个购物清单程序无法编译！运用你的泛型知识来修复它。
//
// 执行 `rustlings hint generics1` 或在 watch 模式下使用 `hint` 子命令以获取提示。


fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
