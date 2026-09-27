use futures::{future::FutureExt, pin_mut, select};
use tokio::runtime::Runtime;
use std::io::Result;
use std::time::Duration;

async fn func1() -> Result<()> {
    tokio::time::sleep(Duration::from_secs(2)).await;   // 异步睡眠
    println!("func1 finished!");
    Ok(())
}

async fn func2() -> Result<()> {
    println!("func2 finished!");
    Ok(())
}

async fn async_main() {
    let f1 = func1().fuse();     // ① 加一层 Fuse
    let f2 = func2().fuse();     // ①

    pin_mut!(f1, f2);            // ② 栈上钉住

    select! {                    // ③ 竞速
        _ = f1 => println!("func1 finished!!!!!"),
        _ = f2 => println!("func2 finished!!!!!"),
    }
}

fn main() {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async_main());
    println!("Hello, world!");
}