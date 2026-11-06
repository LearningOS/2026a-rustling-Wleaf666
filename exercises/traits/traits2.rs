// traits2.rs
//
// 你的任务是为字符串向量（vector of strings）实现 `AppendBar` trait。
// 在实现此 trait 时，请稍加思考向字符串向量“追加 "Bar"”意味着什么。
//
// 这次没有样板代码，你能做到的！
//
// 执行 `rustlings hint traits2` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String>{
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
