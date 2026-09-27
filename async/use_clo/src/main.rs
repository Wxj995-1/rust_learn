use std::future::Future;

fn my_function() -> impl Future<Output = u8> {
    async move {
        let closure = |x: u8| x;
        closure(5)
    }
}

fn main() {
    let result = futures::executor::block_on(my_function());
    println!("{result}");
}