// traits3.rs
//
// 你的任务是为两个结构体都实现 Licensed trait，并让它们在不编写两次相同函数的情况下返回相同的信息。
//
// 考虑一下你可以向 Licensed trait 中添加什么。
//
// 执行 `rustlings hint traits3` 或在 watch 模式下使用 `hint` 子命令以获取提示。

// I AM NOT DONE

pub trait Licensed {
    fn licensing_info(&self) -> String{
        String::from("Some information")
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line
impl Licensed for OtherSoftware {} // Don't edit this line

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
