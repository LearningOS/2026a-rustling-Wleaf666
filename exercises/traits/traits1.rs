// traits1.rs
//
// 是时候实现一些 trait 了！你的任务是为 `String` 类型实现 `AppendBar` trait。
// AppendBar trait 只有一个函数，该函数会向实现此 trait 的任何对象追加 "Bar"。
//
// 执行 `rustlings hint traits1` 或在 watch 模式下使用 `hint` 子命令以获取提示。


trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(self) ->String{
        format!("{}Bar",self)
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
