// from_into.rs
//
// From 特征用于值到值的转换。实现目标类型的 From<来源类型> 后，
// 来源类型会自动获得转换到该目标类型的 Into 实现。更多信息请参阅：
// https://doc.rust-lang.org/std/convert/trait.From.html
//
// 执行 `rustlings hint from_into` 或在 watch 模式下使用 `hint` 子命令以获取提示。

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// 实现 Default 特征，以便在传入的字符串无法转换为 Person 时，
// 使用默认人物作为备用结果。
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// 你的任务是补全下面的实现，让 `let p = Person::from("Mark,20")` 能够编译。
// 注意：需要使用类似 `"4".parse::<usize>()` 的写法，
// 将年龄部分解析为 usize，并妥善处理解析结果。
//
// 步骤：
// 1. 如果传入的字符串长度为 0，则返回 Person 的默认值。
// 2. 按逗号拆分传入的字符串。
// 3. 取出拆分后的第一部分，作为姓名。
// 4. 如果姓名为空，则返回 Person 的默认值。
// 5. 取出另一部分，将其解析为 usize，作为年龄。
// 如果解析年龄时出错，则返回 Person 的默认值；
// 否则，使用得到的姓名和年龄构造并返回一个 Person 实例。


impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let mut it=s.split(',');
        match(it.next(),it.next(),it.next()){
            (Some(name),Some(age),None)if !name.is_empty() => {
                 match age.parse::<usize>(){
                    Ok(age) => Person {
                         name: (name.to_string()), age:(age)
                         },
                    Err(_) => Person::default(),
                 }
            },
            _ => Person::default()
        }
    }
}

fn main() {
    // 使用 from 函数进行转换。
    let p1 = Person::from("Mark,20");
    // 已经为 Person 实现了 From，因此也可以使用 Into 进行同方向的转换。
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // 检查默认人物是否为 30 岁的 John。
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // 检查传入无效字符串时，是否返回默认人物 John。
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // 检查是否能够正确转换 "Mark,20"。
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // 检查 "Mark,twenty" 是否因年龄解析失败而返回默认人物。
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
