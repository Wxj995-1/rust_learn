// 忽略模式中的值

/* 
fn foo(_: i32, y: i32)
{
    println!("y = {}", y);

}


trait A
{
    fn bar(x: i32, y:i32);
}

struct B{
    // a: i32,
}


impl A for B{
    fn bar(_: i32, y: i32)
    {
        println!("y = {}", y);
    }


}
*/

/*
fn main() {
    foo(1, 2);
    let numbers = (1, 2, 3, 4);
    match numbers{
        (one, _, three, _) => {
            println!("oen = {}, three = {}",one, three);
        }
    }
    println!("Hello, world!");
}
 */


fn main()
{
    let _x = 5;
    let _y = 6;

    let s = Some(String::from("hello"));
    if let Some(_) = s
    {
        println!("fount a string");
    }

    println!("s = {:?}", s);
    println!("Hello, world!");
}



