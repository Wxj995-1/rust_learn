use std::sync::Mutex;
use std::thread;
use std::rc::Rc;
use std::sync::Arc;
fn main() {
    // Rc不是线程安全的
    // Arc是线程安全的
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let cnt = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = cnt.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles
    {
        handle.join().unwrap();
    }

    println!("result = {}", *counter.lock().unwrap());
    println!("Hello, world!");
}
