use std::fmt;

/*
Rust 的孤儿规则（orphan rule）：
impl 时，trait 和类型至少要有一个是本 crate 自己定义的。
*/

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.0.join(","))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    println!("Hello, world!");
}