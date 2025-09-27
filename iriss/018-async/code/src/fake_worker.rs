use std::pin::Pin;
use std::task::{Context, Poll};

pub struct FakeWorker {
    pub work_remaining: u8,
}

impl Future for FakeWorker {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.work_remaining {
            0 => Poll::Ready("All done!"),
            _ => {
                self.get_mut().work_remaining -= 1;
                Poll::Pending
            }
        }
    }
}
