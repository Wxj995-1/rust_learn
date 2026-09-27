use futures::executor;

async fn move_block() {
    let my_string = "my_string".to_string();

    let f = async move {
        println!("string = {}", my_string);
    };

    // println!("after move, string = {}", my_string); // 取消注释会报错：值已被移动

    f.await;
}

fn main() {
    executor::block_on(move_block());
    println!("Hello, world!");
}