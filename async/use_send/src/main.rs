use std::rc::Rc;

#[derive(Default)]
struct NoSend(Rc<()>);

async fn bar() {}

/*
async fn foo() {
    let x = NoSend::default();   // 把值绑定给 x
    bar().await;                 // 暂停点，x 还活着 → 必须存进状态机
    // x 在这里才 drop
}
这时状态机里存了 NoSend，于是整个 Future 变成 !Send：

*/

/*
Send 是一个标记 trait，含义是：
这个类型的值可以安全地在线程之间传递。
#[derive] 不需要它，它是自动实现的：一个类型的所有字段都是 Send，它就是 Send。

*/

async fn foo() {
    NoSend::default();   // 临时值，本语句结束就丢弃
    // let x = NoSend::default();
    
    bar().await;         // await 在这里
} 

fn required_send(_: impl Send) {}

fn main() {
    required_send(foo());   // 能通过！
    println!("Hello, world!");
}