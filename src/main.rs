use std::collections::HashMap;
use std::io;

fn main() {
    let mut map: HashMap<&str, &str> = HashMap::new();

    let mut command = String::new();

    // 循环获得用户输入
    loop {
        // 用户输入的整行信息
        io::stdin().read_line(&mut command).expect("读取命令失败！");

        // 获取命令信息
        let option: Option<String> = match command.trim().parse() {
            Ok(i) => Option::Some(i),
            Err(i) => continue
        };

        let mut info: Vec<String> = Vec::new();

        match option {
            Some(x) => {
                for item in x.split_whitespace() {
                    info.push(item.to_string());
                }
            }
            _ => ()
        }
        let x1 = info[0];

        match info[0] {
            "set".to_string() => {
                let x = &info[1];
                map.insert(&info[1], &info[2]);
            }
            _ => ()
        }

        println!("end map:{:?}", map);
    }
}
//
// fn command_info(str: Option<String>) -> Vec<String> {
//     match str {
//         Some(x) => {
//             let mut vec: Vec<String> = Vec::new();
//             for item in x.split_whitespace() {
//                 vec.push(item.to_string());
//             }
//             vec
//         }
//         _ => vec![]
//     }
// }