fn get_str() -> &'static str {
    // 字符串字面量，'static，完全可以返回
    "static string"
}

fn main() {
    let s: &'static str = "Hello, world!";
    println!("{}", s);
    let res = get_str();
    println!("{}", res);
}
