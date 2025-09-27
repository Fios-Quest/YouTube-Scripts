use std::pin::Pin;
use std::task::{Context, Poll, Waker};

struct ExampleFuture;

impl Future for ExampleFuture {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready("The future ran")
    }
}

fn main() {
    let mut example = ExampleFuture;
    let example = Pin::new(&mut example);

    let mut context = Context::from_waker(Waker::noop());

    let poll_response = example.poll(&mut context);

    match poll_response {
        Poll::Ready(output) => assert_eq!(output, "The future ran"),
        Poll::Pending => panic!("This shouldn't happen!"),
    }
}
