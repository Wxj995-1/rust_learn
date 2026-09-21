// 函数的生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    let s1 = String::from("long string");
    let s2 = String::from("string");
    let res = longest(&s1, &s2);
    println!("{}", res);
    println!("Hello, world!");
}
