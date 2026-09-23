// 对象 封装 继承

struct Dog{
    name: String,
}

impl Dog
{
    fn print_name(&self) -> &String
    {
        println!("Dog name = {}",&self.name);
        &self.name
    }
}
fn main() {
    let m = Dog{name: String::from("wangcai")};
    m.print_name();
    println!("Hello, world!");
}
