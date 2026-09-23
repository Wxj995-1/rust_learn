// Rc 只能向下共享，但真实树需要"子节点能回头找到父节点"。父持有子用 Rc，子指向父用 Weak（否则父子互指又成环）。
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,        // 指向父：弱，避免环
    children: RefCell<Vec<Rc<Node>>>,   // 持有子：强
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),   // 先占位
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None，还没有父

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // 给 leaf 补上父指针（弱引用 branch）
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // 现在能通过弱引用升级回父节点
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // Some(Node { value: 5, ... })
}