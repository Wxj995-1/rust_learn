use std::time::Duration;
use tokio::runtime::Runtime;
use std::io::Result;
async fn func1() -> Result<()>{
    tokio::time::sleep(Duration::from_secs(1)).await;   // 异步睡眠（非阻塞）
    println!("func1 finished!");
    Ok(())
}

async fn func2() -> Result<()>{
    println!("func2 finished!");
    Ok(())
}

async fn async_main() {
    let f1 = func1();
    let f2 = func2();

    if let Err(_) = futures::try_join!(f1, f2)   // 并发推进 f1、f2
    {
        println!("Rrr!");
    }
}

fn main() {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async_main());   // 得到 Future 再驱动
    println!("Hello, world!");
}