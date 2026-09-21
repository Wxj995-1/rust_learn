// 方法中的生命周期
#[derive(Debug)]
struct StuA<'a>
{
    name : &'a str,
}

impl<'a> StuA<'a>
{
    fn do_something(&self) -> i32
    {
        3
    }
}
fn main() {
    let s = String::from("hello");
    let a = StuA{name: &s};
    println!("do_something = {}",a.do_something());
    println!("{:#?}",a);
    println!("Hello, world!");
}
