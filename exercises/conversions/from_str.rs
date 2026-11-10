// from_str.rs
//
// 这与 from_into.rs 类似，但这次我们将实现 `FromStr`，
// 并在出错时返回错误而不是回退到默认值。此外，实现 FromStr 后，
// 你可以对字符串使用 `parse` 方法来生成实现者类型的对象。
// 你可以在这里阅读更多内容：https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// 执行 `rustlings hint from_str` 或在 watch 模式下使用 `hint` 子命令以获取提示。

use std::num::ParseIntError;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// 在 FromStr 实现中使用下面的错误类型。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // 输入字符串为空。
    Empty,
    // 字段数量不正确。
    BadLen,
    // 姓名字段为空。
    NoName,
    // 包装 parse::<usize>() 返回的错误。
    ParseInt(ParseIntError),
}


// 步骤：
// 1. 如果传入的字符串长度为 0，则返回错误。
// 2. 按逗号拆分传入的字符串。
// 3. 拆分结果必须恰好包含两个元素，否则返回错误。
// 4. 取出拆分后的第一部分，作为姓名。
// 5. 取出另一部分，使用类似 `"4".parse::<usize>()` 的写法，
//    将其解析为 usize，作为年龄。
// 6. 如果提取姓名或年龄的过程中出错，则返回错误。
// 如果一切顺利，则返回包含 Person 实例的 Result。
//
// 补充说明：`Box<dyn Error>` 实现了 `From<&'_ str>`。
// 因此，如果使用它作为错误类型，就可以通过
// `return Err("我的错误信息".into())` 返回字符串错误信息。
// 这是通用说明；本题使用的错误类型是上面的 ParsePersonError。

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty(){
            return Err(ParsePersonError::Empty);
        }

        let mut it = s.split(',');
        match (it.next(),it.next(),it.next()){
            (Some(name),Some(age),None) =>{
                if name.is_empty(){
                    return Err(ParsePersonError::NoName)
                }
                match age.parse::<usize>(){
                    Ok(age) =>Ok(Person{name:name.to_string(),age:age}),
                    Err(e) =>Err(ParsePersonError::ParseInt(e)),
                }
            }
            _ => Err(ParsePersonError::BadLen)
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
