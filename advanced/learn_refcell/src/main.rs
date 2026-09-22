//1、内部可变性：允许在使用不可变引用时改变数据。
//2、通过RefCell<T>在运行时检查借用规则（通常情况下，是在编译时检查借用规则），RefCell<T>代表其数据的唯一所有权。
//类似于Rc<T>，RefCell<T>只能用于单线程场景。
//3、选择Box<T>、Rc<T>或RefCell<T>的理由：
//Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
//Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
//因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。


use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let data = Rc::new(RefCell::new(100));
    let data2 = Rc::clone(&data); // 多所有者 Rc clone
    // 
    // 单线程场景
    // 
    *data.borrow_mut() += 10; // 修改内部数据
    println!("data = {}", data2.borrow()); // 110
}
