// options1.rs
//
// 执行 `rustlings hint options1` 或在 watch 模式下使用 `hint` 子命令以获取提示。
//
// 这个函数返回冰箱里还剩下多少冰淇淋。
// 如果在晚上 10 点之前，还剩 5 份。到了晚上 10 点，有人把它们全吃光了，
// 所以就一份也不剩了 :(

fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    if time_of_day>23
    {
        Option::None
    }else if time_of_day>20
    {
        Option::Some(0)
    }else{
        Option::Some(5)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        let icecreams = maybe_icecream(12).unwrap();
        assert_eq!(icecreams, 5);
    }
}
