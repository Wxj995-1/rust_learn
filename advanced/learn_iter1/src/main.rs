//1、迭代器负责遍历序列中的每一项和决定序列何时结束的逻辑。
//2、创建迭代器：迭代器是惰性的，意思就是在调用方法使用迭代器之前，不会有任何效果
//3、每个迭代器都实现了 Iterator trait 定义在标准库中
trait Iterator {
    // 关联类型：迭代器产出的元素类型
    type Item;
    // 核心方法：返回下一个元素，Option包裹
    fn next(&mut self) -> Option<Self::Item>;
}
/*
- .iter()：迭代器产出&T，借用原集合，不转移所有权
- .iter_mut()：迭代器产出&mut T，可变借用，可以修改元素
- .into_iter()：迭代器产出T，拿走所有权，原 vector 失效
*/

fn main() {
    let v = vec![1,2,3];
    // 创建迭代器：iter()，此时只是生成迭代器对象，没有遍历（惰性）
    let iter = v.iter();

    // for循环会自动调用迭代器的next()，消耗迭代器
    for val in iter {
        println!("{}", val);
    }

    // ========== 手动调用next，直观理解Iterator trait ==========
    // 当迭代器结束的时候 返回None
    let mut iter2 = v.iter();
    println!("next 1: {:?}", iter2.next()); // Some(&1)
    println!("next 2: {:?}", iter2.next()); // Some(&2)
    println!("next 3: {:?}", iter2.next()); // Some(&3)
    println!("next 4: {:?}", iter2.next()); // None，迭代结束

    // 迭代可变引用
    let mut v2 = vec![1,2,3,4];
    let mut v2_iter = v2.iter_mut();
    if let Some(v) = v2_iter.next()
    {
        *v = 3;
    }
    println!("v2 = {:?}",v2);

    // 消费适配器
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: u32 = v1_iter.sum();
    println!("tatal = {}",total);

    // 迭代适配器
    let v1 = vec![1,2,3];
    let v2: Vec<_> = v1.iter().map(|x| x +1 ).collect();
    println!("v2 = {:?}",v2);

    // 迭代适配器
    let v1 = vec![1,2,3,12,34];
    let v2: Vec<_> = v1.into_iter().filter(|x| *x > 5 ).collect();
    println!("v2 = {:?}",v2);
}
