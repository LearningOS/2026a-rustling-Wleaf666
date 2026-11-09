// quiz2.rs
//
// 这是针对以下章节的测验：
// - 字符串 (Strings)
// - 向量 (Vecs)
// - 移动语义 (Move semantics)
// - 模块 (Modules)
// - 枚举 (Enums)
//
// 让我们以函数的形式构建一个小机器。我们将输入一个由字符串和命令组成的列表。
// 这些命令决定对字符串执行什么操作：
// - 将字符串转为大写
// - 修剪字符串首尾空白
// - 将 "bar" 附加到字符串后指定次数
// 具体的格式如下：
// - 输入是一个由 2 元元组组成的 Vector，元组第一个元素是字符串，第二个是命令。
// - 输出是一个由处理后的字符串组成的 Vector。
//
// 这次没有提示！


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input:  Vec<(String,Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        for (mut string, command) in input {
            // TODO: Complete the function body. You can do it!
            match command{
                Command::Uppercase =>output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().to_string()),
                Command::Append(n) =>{
                    for _ in 0..n{
                        string.push_str("bar");
                    }
                    output.push(string);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
