// lifetimes1.rs
//
// Rust 编译器需要知道如何检查传入的引用是否有效，以便在引用被使用前存在超出作用域的风险时提醒程序员。
// 请记住，引用是借用，并不拥有自己的数据。如果它们的所有者超出了作用域会怎么样？
//
// 执行 `rustlings hint lifetimes1` 或在 watch 模式下使用 `hint` 子命令以获取提示。

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
