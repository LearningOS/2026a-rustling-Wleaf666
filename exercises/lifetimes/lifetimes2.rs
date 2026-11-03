// lifetimes2.rs
//
// 如果编译器只是在验证传递给标注了生命周期的参数与返回类型之间的引用关系，那么我们需要修改什么？
//
// 执行 `rustlings hint lifetimes2` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}
