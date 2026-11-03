// intro1.rs
//
// 关于 `I AM NOT DONE`（我还没完成）这个标记：
// 我们有时会鼓励你在给定的练习中多尝试一些东西，即使你已经搞懂了它。
// 如果你已经让所有内容正常工作，并准备好进入下一个练习，请删除下方的 `I AM NOT DONE` 注释。
//
// 如果你使用 `rustlings watch` 运行：
// 当你修改下面的某一行时，练习文件将自动重新加载！试着添加一行 `println!`，
// 或者尝试在终端中更改其输出内容。试着删除一个分号，看看会发生什么！
//
// 执行 `rustlings hint intro1` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

fn main() {
    println!("Hello and");
    println!(r#"       welcome to...                      "#);
    println!(r#"                 _   _ _                  "#);
    println!(r#"  _ __ _   _ ___| |_| (_)_ __   __ _ ___  "#);
    println!(r#" | '__| | | / __| __| | | '_ \ / _` / __| "#);
    println!(r#" | |  | |_| \__ \ |_| | | | | | (_| \__ \ "#);
    println!(r#" |_|   \__,_|___/\__|_|_|_| |_|\__, |___/ "#);
    println!(r#"                               |___/      "#);
    println!();
    println!("This exercise compiles successfully. The remaining exercises contain a compiler");
    println!("or logic error. The central concept behind Rustlings is to fix these errors and");
    println!("solve the exercises. Good luck!");
    println!();
    println!("The source for this exercise is in `exercises/intro/intro1.rs`. Have a look!");
    println!(
        "Going forward, the source of the exercises will always be in the success/failure output."
    );
    println!();
    println!(
        "If you want to use rust-analyzer, Rust's LSP implementation, make sure your editor is set"
    );
    println!("up, and then run `rustlings lsp` before continuing.")
}
