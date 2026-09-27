use futures::executor;

async fn async_function1() {
    println!("async function1 ++++!");
}

async fn async_function2() {
    println!("async function2 ++++!");
}

async fn async_main() {
    let f1 = async_function1();
    let f2 = async_function2();

    let f = async move {
        f1.await;
        f2.await;
    };

    f.await;
}

fn main() {
    executor::block_on(async_main());
    println!("Hello, world!");
}

// ---------------------------------------------//
async fn async_put_data_to_buf(buf: &mut [u8]) {
    buf[0] = 42;   // 往 buf 写数据
}

async fn async_main() {
    let f = async {
        let mut x = [0u8; 128];
        let async_put = async_put_data_to_buf(&mut x);
        async_put.await;
        println!("x[0] = {}", x[0]);
    };

    f.await;   // 驱动 f，否则什么都不发生
}







/*

// 一个状态机结构体，字段 = 进度 + 需要保存的局部值
struct AsyncMainFuture {
    state: u8,            // 当前跑到哪个状态
    f1: Option<Fn1Future>,
    f2: Option<Fn2Future>,
    f:  Option<ComboFuture>,
}

impl Future for AsyncMainFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<()> {
        loop {
            match self.state {
                // 状态 0：造 f1，进入等待 f1
                0 => {
                    self.f1 = Some(async_function1());
                    self.state = 1;
                }
                // 状态 1：poll f1；没好就 Pending，好了就进下一步
                1 => {
                    match self.f1.as_mut().unwrap().poll(cx) {
                        Poll::Ready(_) => self.state = 2,
                        Poll::Pending  => return Poll::Pending,   // 挂起
                    }
                }
                // 状态 2：造 f2
                2 => {
                    self.f2 = Some(async_function2());
                    self.state = 3;
                }
                // 状态 3：poll f2；没好就 Pending，好了就进下一步
                3 => {
                    match self.f2.as_mut().unwrap().poll(cx) {
                        Poll::Ready(_) => self.state = 4,
                        Poll::Pending  => return Poll::Pending,
                    }
                }
                _ => return Poll::Ready(()),
            }
        }
    }
}
*/