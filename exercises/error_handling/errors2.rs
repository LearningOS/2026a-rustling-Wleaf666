// errors2.rs
//
// 假设我们正在编写一个游戏，玩家可以用代币购买物品。所有物品花费 5 个代币，
// 且每次购买物品都会收取 1 个代币的手续费。游戏玩家将输入他们想购买的物品数量，
// `total_cost` 函数将计算总代币成本。然而，由于玩家输入的是数量，我们得到的是字符串——
// 他们可能输入了任何内容，而不仅仅是数字！
//
// 目前该函数根本没有处理错误情况（也没有正确处理成功情况）。我们希望做到的是：
// 如果对非数字字符串调用 `parse` 函数，该函数将返回 `ParseIntError`，在这种情况下，
// 我们希望立即从函数中返回该错误，而不是尝试乘法和加法。
//
// 至少有两种正确的实现方式——但其中一种要短得多！
//
// 执行 `rustlings hint errors2` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
