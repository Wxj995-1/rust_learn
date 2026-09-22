struct Dog
{
    name:String,
    // count : i32,
}

impl Drop for Dog
{
    fn drop(&mut self)
    {
        println!("{} leave", &self.name);
    }
}
// rust 提供了std::mem::drop()
fn main() {
    let a = Dog{name:String::from("wangcai")};
    let b= Dog{name:String::from("dahuang")};
    drop(b);

    println!("Hello, world!");
}
