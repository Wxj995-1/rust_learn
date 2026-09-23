//1 匹配字面值
/*
fn main() {
    let x = 1;
    match x 
    {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("xx"),
    }

    println!("Hello, world!");
} */

//2 匹配命名变量
/*
fn main()
{
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("50"),
        Some(y) => println!("value = {}",y),
        _ => println!("other"),
    }

    println!("x = {:?}, y = {:?}", x, y);

} */

//3 多个模式
/*
fn main()
{
    let x = 1;
    match x 
    {
        1|2=> println!("1 or 2"),
        2 => println!("two"),
        _ => println!("xx"),
    }
} */

//4 通过.. 匹配
fn main()
{
    let x = 5;
    match x 
    {
        1..= 5 => println!("1 to 5"), // 1|2|3|4|5
        2 => println!("two"),
        _ => println!("xx"),
    }
}
