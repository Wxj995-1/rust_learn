fn main() {

    // bool
    let is_true: bool = true;
    println!("is_true = {}",is_true);
    let is_false: bool = false;
    println!("is_false = {}",is_false);

    println!("is_false_true = {}, {}",is_false, is_true);

    // char 在rust里面 char是32为的
    let a:char = 'a';
    println!("a = {}",a);
   
    let b:char = '你';
    println!("b = {}",b);

    // i8 i32 i64 u8 u18 u32 u64 f32 f64
    let c: i8 = -111;
    println!("c = {}",c);

    let d: f32 = 0.008;
    println!("d = {}",d);

    // 自适应类型 isize usize 
    println!("max = {}", usize::max_value());

    // 数组 [type; size] size也是数组函数的一部分
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    println!("arr[0] = {}",arr[0]);
    show(arr);

    // 元组
    let tup: (i32, f32, char) = (-3,3.46, '好');
    println!("{}",tup.0);
    println!("{}",tup.1);
    println!("{}",tup.2);

    let (_x, _y, _z) = tup;
    println!("{}",_x);


}


fn show(arr:[u32;5])
{
    println!("----------------------------");
    for i in &arr{
        println!("{}", i);
    }
    println!("----------------------------");
}





