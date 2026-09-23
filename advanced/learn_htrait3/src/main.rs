trait A{
    fn print(&self);
}

trait B{
    fn print(&self);
}

struct MyType;

impl A for MyType
{
    fn print(&self)
    {
        println!("A trait for MyType");
    }
}

impl B for MyType
{
    fn print(&self)
    {
        println!("B trait for MyType");
    }
}

impl MyType
{
    fn print(&self)
    {
        println!("MyType");
    }
}


fn main() {
    let m_type = MyType;
    m_type.print(); // MyType::print(&m_type);
    A::print(&m_type);
    B::print(&m_type);
    println!("Hello, world!");
}
