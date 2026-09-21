// T: PartialOrd  支持 > < 比较
// T: Copy        可以拷贝，这样 list[0] 可以直接复制给 larger
// - list 只是参数名，随便叫。
// - &[T] 才是类型，它由三部分构成：& + [ ] + T
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut larger = list[0];
    for &item in list.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}

fn main() {
    let number_list = vec![1, 2, 23, 34, 8, 100];
    let max_number = largest(&number_list);
    println!("max_number = {}", max_number);

    let char_list = vec!['a', 'y', 'b'];
    let max_char = largest(&char_list);
    println!("max_char = {}", max_char);

    println!("Hello, world!");
}
