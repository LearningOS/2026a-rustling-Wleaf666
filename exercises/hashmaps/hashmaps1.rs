// hashmaps1.rs
//
// 需要定义一个哈希表形式的水果篮。键代表水果的名字，值代表篮子中该特定水果的数量。
// 你必须在篮子里放入至少三种不同类型的水果（例如 apple、banana、mango），
// 且所有水果的总数至少为 5 个。
//
// 让代码通过编译并使测试通过！
//
// 执行 `rustlings hint hashmaps1` 或在 watch 模式下使用 `hint` 子命令以获取提示。


use std::collections::{HashMap, hash_map};

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket here.
    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("orange"), 2);
    basket.insert(String::from("watermelon"), 2);

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
