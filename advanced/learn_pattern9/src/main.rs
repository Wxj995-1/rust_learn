/* 
fn main() {
    let num = Some(4);
    match num 
    {
        Some(x) if x < 5 => println!("< 5"),
        Some(x) => println!("x:{}", x),
        None => (),
    }




    println!("Hello, world!");
}
*/


fn main()
{
    let num = Some(4);
    let y = 4;

    match num 
    {
        Some(x) if x == y => println!("num == y"),
        Some(x) => println!("x:{}", x),
        None => (),
    }

    let x = 4;
    let y = true;
    match x
    {
        4|5|6 if y => println!("1"),
        _ => println!("2"),
    }
    println!("Hello, world!");

}