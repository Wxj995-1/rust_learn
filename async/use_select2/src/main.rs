use futures::future;
use futures::select;

async fn count() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break,
            default => unreachable!(),   // 不会执行（future 立即完成）
        }
    }

    assert_eq!(total, 10);
}

fn main() {
    futures::executor::block_on(count());
    println!("Hello, world!");
}