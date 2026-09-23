fn main() {
    // ==========1. 字面量模式==========
    let x = 5;
    match x {
        1 => println!("匹配到1"),
        5 => println!("匹配字面量5"), // 命中
        _ => println!("其他"), // 必须匹配所有的情况
    }

    // ==========2. 解构：元组==========
    let t = (10, 20);
    let (a, b) = t; // 解构元组，a=10,b=20
    println!("a={},b={}", a, b);

    // ==========解构：结构体==========
    #[derive(Debug)]
    struct Point {    
        x: i32,
        y: i32,
    }
    let p = Point { x: 1, y: 2 };
    let Point { x: px, y: py } = p;
    println!("px={},py={}", px, py);

    // ==========解构：枚举 Option==========
    let num = Some(42);
    if let Some(v) = num {
        println!("枚举解构拿到v={}", v);
    }

    // ==========3. 变量模式==========
    let val = 99; // val就是变量模式，绑定99

    // ==========4. 通配符 _ ==========
    let m = 100;
    match m {
        10 => println!("10"),
        _ => println!("其他值"), // _匹配任何东西，不存储
    }

    // ==========5. 占位符 .. 忽略剩余==========
    let arr = [1,2,3,4,5];
    let [first, ..] = arr; // 只取第一个，剩下全部忽略
    println!("first = {}", first);

    // ==========6. 函数的参数也是模式 ================
    let p = (3, 5);
    print_point(&p);
}

// 模式在使用他的地方并不都是相同的，模式存在可反驳和不可反驳
fn print_point(&(x, y): &(i32, i32))
{
    println!("x = {}, y = {}", x, y);
}