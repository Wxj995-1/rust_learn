//1、Rust中一个实现消息传递并发的主要工具是通道。通道由两部分组成，一个是发送端，一个是接收端，发送端用来发送消息，接收端用来接收消息。发送者或者接收者任一被丢弃时就可以认为通道被关闭了。
//2、通道介绍
//（1）通过mpsc::channel，创建通道，mpsc是多个生产者，单个消费者；
//（2）通过spmc::channel，创建通道，spmc是一个生产者，多个消费者；
//（3）创建通道后返回的是发送者和消费者，示例：
//let (tx, rx) = mpsc::channel();
//let (tx, rx) = spmc::channel();
use std::sync::mpsc;
use std::thread;
 /* 
fn main() {
    let (tx, rx) = mpsc::channel();   // 返回 (Sender<T>, Receiver<T>)

    thread::spawn(move || {
        tx.send(String::from("hello")).unwrap();  // 发送
    });

    let received = rx.recv().unwrap();           // 阻塞接收
    println!("receive: {}", received);              // 收到: 你好
}*/



fn main() {
    let (tx, rx) = mpsc::channel();

    for id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("来自线程 {id}")).unwrap();
        });
    }

    drop(tx);   // 丢弃原始发送端，否则 rx 永远不会收到关闭信号

    for msg in rx {          // 也可以直接迭代 Receiver
        println!("{msg}");
    }
}