use std::io;
use crate::common;


pub fn  merge_string_main() {
    let mut word1 = String::new();
    let mut word2 = String::new();

    common::putinto(&mut word1, &mut word2);
    let mut result = merge_string(word1.clone(), word2.clone());
    println!("{}", result);

    let mut handle_input = || {
        println!("按回车键退出，输入空格重新输入字符串...");
        loop {
            let mut dummy = String::new();
            io::stdin().read_line(&mut dummy).unwrap();
             
            if &dummy == " \r\n" {
                common::putinto(&mut word1, &mut word2);
                result = merge_string(word1.clone(), word2.clone());
                println!("当前结果: {}", result);
            } else if &dummy == "\r\n" {
                break;
            } else {
                result = merge_string(result.clone(), dummy.trim().to_string());
                println!("当前结果: {}", result);
            }
        }
    };

    handle_input();
}



pub fn merge_string(word1: String, word2: String) -> String {
    let mut item1 = word1.chars();
    let mut item2 = word2.chars();
    let mut s = String::new();

    loop {
        match (item1.next(), item2.next()) {
            (Some(c1), Some(c2)) => {
                s.push(c1);
                s.push(c2);
            }
            (Some(c1), None) => {
                s.push(c1);
            }
            (None, Some(c2)) => {
                s.push(c2);
            }
            (None, None) => {
                break;
            }
        }
    }

    s
}
