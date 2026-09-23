use std::sync::Mutex;

// Mutex<T> 是一个智能指针，lock调用返回一个叫做MutexGuard的智能指针
// Mutex<T> 内部提供了drop方法，实现当MutexGuard离开作用域的时候自动释放锁
fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }  // 离开作用域 锁自动释放

    println!("m = {:?}",m);
    println!("Hello, world!");
}
