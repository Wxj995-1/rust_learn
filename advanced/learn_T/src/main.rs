// 1. 泛型：类型占位符，消除重复代码，编译单态化无性能损耗
use std::fmt::Display;

// ========== 2、函数中使用泛型 ==========
// T: PartialOrd 约束：T必须支持大小比较
fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max {
            max = item;
        }
    }
    max
}

// ==========3、结构体中的泛型 ==========
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// ==========5、方法中的泛型（impl块带泛型） ==========
impl<T, U> Point<T, U> {
    // 泛型方法
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &U {
        &self.y
    }

    // 方法可以自己定义新泛型V,W，和结构体的T,U无关
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// 单独为 Point<f64,f64> 实现方法，只针对浮点类型
impl Point<f64, f64> {
    fn print_float_point(&self) {
        println!("浮点数点 x={}, y={}", self.x, self.y);
    }
}

// ==========4、枚举中的泛型 ==========
#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

fn main() {
    // 2.测试泛型函数
    let num_list = [12, 34, 8, 99];
    println!("i32数组最大值：{}", largest(&num_list));
    let char_list = ['b', 'z', 'a'];
    println!("char数组最大值：{}", largest(&char_list));

    //3.测试泛型结构体
    let p1 = Point { x: 10, y: 3.14 };
    println!("Point x={}, y={}", p1.get_x(), p1.get_y());
    let p_float = Point { x:1.1, y:2.2 };
    p_float.print_float_point();

    // 测试方法内新泛型 
    let p2 = Point{x:"hello", y:true};
    let p_mix = p1.mixup(p2);
    println!("mix之后 {:?}", p_mix);

    //4.泛型枚举
    let some_str = MyOption::Some(String::from("泛型枚举"));
    let none_val: MyOption<i32> = MyOption::None;
    println!("MyOption:{:?}", some_str);
    println!("MyOption:{:?}", none_val);

    // ==========6、单态化说明（代码层面看不到，编译期自动完成）==========
    /*
    编译时Rust自动生成：
    fn largest_i32(list:&[i32])->&i32 { ... }
    fn largest_char(list:&[char])->&char { ... }
    运行时没有泛型，和手写多份函数性能完全一致，无开销
    */
}
