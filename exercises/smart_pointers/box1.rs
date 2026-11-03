// box1.rs
//
// 在编译时，Rust 需要知道某个类型占用多少空间。对于递归类型，这就会产生问题，
// 因为递归类型的值可能将相同类型的另一个值作为自身的一部分。
// 为了解决这个问题，我们可以使用 `Box`——一种用于在堆上存储数据的智能指针，它也允许我们包装递归类型。
//
// 我们在这个练习中实现的递归类型是 `cons list`——函数式编程语言中常见的数据结构。
// cons list 中的每一项包含两个元素：当前项的值和下一项。最后一项是一个名为 `Nil` 的值。
//
// 步骤 1：在枚举定义中使用 `Box` 使代码通过编译
// 步骤 2：通过替换 `todo!()` 分别创建空和非空的 cons list
//
// 注意：测试用例不应被修改
//
// 执行 `rustlings hint box1` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    todo!()
}

pub fn create_non_empty_list() -> List {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
