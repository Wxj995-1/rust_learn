// 解构并分解值
// 解构元组、结构体、枚举、引用

// 解构结构体
struct Point
{
    x: i32,
    y: i32,
}

/*
fn main() {
    let p = Point{x: 1, y: 2};
    // 变量 x 和 y 匹配 a 和 b
    let Point{x: a, y: b} = p;

    let Point{x, y} = p;
    assert_eq!(1, x);
    assert_eq!(2, y);

    println!("Hello, world!");
} */


fn main() {
    let p = Point{x: 1, y: 0};
    match p
    {
        Point{x, y: 0} => println!("x axis"),
        Point{x: 0, y} => println!("y axis"),
        Point{x, y} => println!("other"),
    }

    println!("Hello, world!");
}