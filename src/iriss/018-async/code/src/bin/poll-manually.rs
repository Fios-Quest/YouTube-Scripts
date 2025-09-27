use async_rust::fake_worker::FakeWorker;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

fn main() {
    let mut example = FakeWorker { work_remaining: 3 };
    let mut example = Pin::new(&mut example);

    let mut context = Context::from_waker(Waker::noop());

    assert_eq!(example.as_mut().poll(&mut context), Poll::Pending);
    assert_eq!(example.as_mut().poll(&mut context), Poll::Pending);
    assert_eq!(example.as_mut().poll(&mut context), Poll::Pending);
    assert_eq!(
        example.as_mut().poll(&mut context),
        Poll::Ready("All done!")
    );
}
