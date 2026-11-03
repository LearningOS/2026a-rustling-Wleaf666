// using_as.rs
//
// Rust 中的类型转换是通过使用 `as` 运算符完成的。请注意，
// `as` 运算符不仅用于类型转换，还可以帮助重命名导入项。
//
// 目标是确保除法运算不会编译失败，并且返回正确的类型。
//
// 执行 `rustlings hint using_as` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len()
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
