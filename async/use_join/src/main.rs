use std::time::Duration;
use tokio::runtime::Runtime;

async fn func1() {
    tokio::time::sleep(Duration::from_secs(1)).await;   // 异步睡眠（非阻塞）
    println!("func1 finished!");
}

async fn func2() {
    println!("func2 finished!");
}

async fn async_main() {
    let f1 = func1();
    let f2 = func2();

    futures::join!(f1, f2);   // 并发推进 f1、f2
}

fn main() {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async_main());   // 得到 Future 再驱动
    println!("Hello, world!");
}