#[derive(Debug)]
struct A<'a>{
    name :&'a str,
}

// 生命周期省略
fn  get_a_str(s: &str) -> &str
{
    s
}
fn main() {
    let n  =String::from("hello, world!");
    let a = A{name : &n};

    let s = get_a_str(&n);
    println!("s = {:#?}",s);
    println!("Hello, world!");
}
