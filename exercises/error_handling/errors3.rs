// errors3.rs
//
// 这是一个试图使用上一个练习中完成版 `total_cost` 函数的程序。但它无法正常运行！
// 为什么不行？我们应该怎么修复它？
//
// 执行 `rustlings hint errors3` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
