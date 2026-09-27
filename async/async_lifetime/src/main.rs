use futures::executor;
use std::future::Future;

async fn foo(x: &u8) -> u8 {
    *x
}

fn foo_expand<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move { *x }
}

fn bad_fixed_1() -> impl Future<Output = u8> {
    let x = 5;
    async move { x }                 // 用 async move 把 x 搬进 Future
}

fn bad_fixed_2() -> impl Future<Output = u8> {
    let x = 5;
    async move { foo_expand(&x).await }   // x 归 Future 所有，内部再借用
}

fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;                   // x 创建在 Future 内部
        foo_expand(&x).await
    }
}

fn main() {
    let x = 5;
    let f = foo(&x);
    println!("{}", executor::block_on(f));          // 5

    println!("{}", executor::block_on(foo_expand(&x)));  // 5
    println!("{}", executor::block_on(bad_fixed_1()));   // 5
    println!("{}", executor::block_on(bad_fixed_2()));   // 5
    println!("{}", executor::block_on(good()));          // 5
}