// as_ref_mut.rs
//
// AsRef 和 AsMut 允许进行开销很小的引用到引用的转换。
// 请分别在以下链接中了解更多信息：
// https://doc.rust-lang.org/std/convert/trait.AsRef.html 和
// https://doc.rust-lang.org/std/convert/trait.AsMut.html。
//
// 执行 `rustlings hint as_ref_mut` 或在 watch 模式下使用 `hint` 子命令以获取提示。


// 获取传入参数的字节数（不是字符数）。
// TODO: 为泛型参数添加合适的 AsRef 特征约束。
fn byte_counter<T:AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// 获取传入参数的字符数（不是字节数）。
// TODO: 为泛型参数添加合适的 AsRef 特征约束。
fn char_counter<T:AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// 使用 as_mut() 将一个数修改为它的平方。
// TODO: 添加合适的特征约束。
fn num_sq<T:AsMut<u32>>(arg: &mut T) {
    // TODO: 实现函数体。
    let n=arg.as_mut();
    *n=*n * *n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
