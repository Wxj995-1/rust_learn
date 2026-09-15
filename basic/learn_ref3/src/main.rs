// 参数：&str 切片，返回值：&str切片
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] // 没有空格，返回整个字符串
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s); // String自动转为&str切片
    println!("{}", word);
}
