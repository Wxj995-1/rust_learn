//1、Rust中每一个引用都有其生命周期，也就是引用保持有效的作用域。大部分时候生命周期是隐含并可以推断的，正如大部分时候类型可以推断一样。
//2、生命周期的主要目标是避免悬垂引用。
//3、Rust编译器使用借用检查器来检查生命周期是否有效。


// 'a 生命周期标注：返回的引用生命周期，是x和y中较短的那一个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("long string");
    let s2 = String::from("short");
    let res = longest(&s1, &s2);
    println!("{}", res);
}
