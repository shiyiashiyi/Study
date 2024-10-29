use crate::common;

pub fn gcd_of_strings_main(){
    let mut word1 = String::new();
    let mut word2 = String::new();

    common::putinto(&mut word1,&mut word2);
    let result = gcd_of_strings(word1.clone(), word2.clone());
    println! ("{}", result);
}

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let mut s1 = str1.clone();
    s1 += &str2;
    let mut s2 = str2.clone();
    s2 += &str1;
    //判断是否没有公因式
    if s1 != s2{
        return String::from("");
    }
    let n = gcd(str1.len(), str2.len());
    str1.get(0..n).unwrap().to_string()
}

pub fn gcd(a:usize, b:usize) -> usize {
    if b == 0 { a } else {gcd(b, a%b) }
}