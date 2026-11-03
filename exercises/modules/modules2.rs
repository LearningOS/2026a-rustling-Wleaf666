// modules2.rs
//
// 你可以使用 'use' 和 'as' 关键字将模块路径引入作用域并为其指定别名。
// 修复这些 'use' 语句以使代码通过编译。
//
// 执行 `rustlings hint modules2` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

mod delicious_snacks {
    // TODO: Fix these use statements
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;
    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
