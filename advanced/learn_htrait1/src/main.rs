
/*
pub trait Iterator
{
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
} */


pub trait Iterator1<T> {
    fn next(&mut self) -> Option<T>;
}

struct A {
    value: i32,
}

impl Iterator1<i32> for A {
    fn next(&mut self) -> Option<i32> {
        println!("in i32");
        if self.value > 3 {
            self.value += 1;
            Some(self.value)
        } else {
            None
        }
    }
}

impl Iterator1<String> for A {
    fn next(&mut self) -> Option<String> {
        println!("in String");
        if self.value > 3 {
            self.value += 1;
            Some(String::from("hello"))
        } else {
            None
        }
    }
}

fn main() {
    let mut a = A { value: 3 };

    // 用限定语法指定具体实现，消除歧义
    let _ = <A as Iterator1<i32>>::next(&mut a);
    let _ = <A as Iterator1<String>>::next(&mut a);

    println!("Hello, world!");
}