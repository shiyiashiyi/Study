use std::io;


pub fn putinto(word1: &mut String, word2: &mut String) {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("请输入第一个字符串:");
    io::stdin().read_line(&mut input1).expect("读取失败");
    *word1 = input1.trim().to_string();

    println!("请输入第二个字符串:");
    io::stdin().read_line(&mut input2).expect("读取失败");
    *word2 = input2.trim().to_string();
}