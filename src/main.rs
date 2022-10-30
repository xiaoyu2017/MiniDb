use std::collections::HashMap;
use std::io;

use crate::pojo::stringdata::StringData;
// 引入属性
use crate::tool::command::MiniDb;

// 需要加载的代码
pub mod tool;
pub mod pojo;

fn main() {
    let mut string_data = StringData::new();
    'input: loop {
        let mut command = String::new();

        io::stdin().read_line(&mut command).expect("读取命令失败！");
        // 获取命令信息
        let option: Option<String> = match command.trim().parse() {
            Ok(i) => Option::Some(i),
            Err(i) => continue
        };

        if let Some(i) = option {
            let vec = MiniDb::command_to_vec(&i);
            if vec[0] == "set" {
                &mut string_data.set(vec[1].to_string(), vec[2].to_string());
                println!("success");
            } else if vec[0] == "get" {
                println!("{}", &mut string_data.get(vec[1].to_string()));
            } else {
                println!("End of the program！");
                break 'input;
            }
        };
    }
}
