use std::pin::Pin;
use std::task::{Context, Poll};

#[tokio::main]
async fn main() {
    // let mut cx = Context::from_waker(futures::task::noop_waker_ref());
    // let ret = poll_fut(&mut cx);
    let fut = MyFut::new(42);
    println!("Final Result: {:?}", fut.await);
}

#[allow(dead_code)]
fn poll_fut(cx: &mut Context<'_>) -> Poll<usize> {
    let mut fut = MyFut::new(42);
    let fut = Pin::new(&mut fut);
    my_ready!(fut.poll(cx))
}

struct MyFut {
    polled: bool,
    v: usize,
}

impl MyFut {
    fn new(v: usize) -> Self {
        Self { polled: false, v }
    }
}

impl Future for MyFut {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(self.v)
        } else {
            self.polled = true;
            //wake up the waker
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

//my_ready! => Poll::Ready / Poll::Pending
#[macro_export]
macro_rules! my_ready {
    ($expr:expr) => {
        match $expr {
            std::task::Poll::Ready(val) => std::task::Poll::Ready(val),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    };
}
