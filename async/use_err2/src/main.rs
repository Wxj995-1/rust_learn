async fn foo() -> Result<(), String> {
    Ok(())
}

async fn func() -> Result<(), String> {
    let fut = async {
        foo().await?;        // 调用 foo()，await，再用 ? 传播错误
        Ok(())
    };
    fut.await                // 返回 Result<(), String>
}

fn main() {
    futures::executor::block_on(func()).unwrap();
    println!("Hello, world!");
}