use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

enum Poll<T> { Ready(T), Pending }

// Waker：唤醒 = 往 channel 发信号，把任务放回“就绪队列”
#[derive(Clone)]
struct Waker {
    tx: mpsc::Sender<()>,
}

impl Waker {
    fn wake(&self) {
        let _ = self.tx.send(());   // 相当于 waker.wake()
    }
}

trait SimpleFuture {
    type Output;
    fn poll(&mut self, waker: &Waker) -> Poll<Self::Output>;
}

struct MySleeper {
    done: Arc<AtomicBool>,
    registered: bool,
}

impl SimpleFuture for MySleeper {
    type Output = ();
    fn poll(&mut self, waker: &Waker) -> Poll<()> {
        if self.done.load(Ordering::SeqCst) {
            return Poll::Ready(());
        }
        if !self.registered {
            self.registered = true;            // 只注册一次
            let done = self.done.clone();
            let waker = waker.clone();
            thread::spawn(move || {            // 模拟 reactor 的定时器
                thread::sleep(Duration::from_secs(5));
                done.store(true, Ordering::SeqCst);
                waker.wake();                  // 事件到了 → 唤醒任务
            });
        }
        Poll::Pending
    }
}

fn block_on<F: SimpleFuture>(mut fut: F) {
    let (tx, rx) = mpsc::channel();
    let waker = Waker { tx };
    loop {
        match fut.poll(&waker) {
            Poll::Ready(_) => { println!("my future is ok"); break; }
            Poll::Pending => {
                rx.recv().unwrap();  // 阻塞等待被唤醒，不做忙等
            }
        }
    }
}

fn main() {
    let fut = MySleeper {
        done: Arc::new(AtomicBool::new(false)),
        registered: false,
    };
    block_on(fut);
}