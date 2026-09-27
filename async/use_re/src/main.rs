use std::future::Future;
use std::pin::Pin;
use futures::executor;

async fn first() {}
async fn second() {}

async fn foo() {
    first().await;
    second().await;
}

// 递归 async：返回 Pin<Box<dyn Future>> 提供间接层，并加终止条件
fn re(n: u32) -> Pin<Box<dyn Future<Output = ()>>> {
    Box::pin(async move {
        if n == 0 {
            return;                     // 终止条件
        }
        println!("re({n})");
        re(n - 1).await;                // 规模递减
    })
}

fn main() {
    executor::block_on(foo());
    executor::block_on(re(3));
    println!("Hello, world!");
}