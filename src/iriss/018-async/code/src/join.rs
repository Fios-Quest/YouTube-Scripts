use std::pin::Pin;
use std::task::{Context, Poll};

pub mod collapsable_future;

use crate::join::collapsable_future::CollapsableFuture;

pub struct Join<F1: Future, F2: Future>(
    Pin<Box<CollapsableFuture<F1>>>,
    Pin<Box<CollapsableFuture<F2>>>,
);

impl<F1: Future, F2: Future> Join<F1, F2> {
    pub fn new(future1: F1, future2: F2) -> Self {
        Self(
            Box::pin(CollapsableFuture::new(future1)),
            Box::pin(CollapsableFuture::new(future2)),
        )
    }
}

impl<F1: Future, F2: Future> Future for Join<F1, F2> {
    type Output = (F1::Output, F2::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = self.get_mut();

        let p1 = inner.0.as_mut().poll(cx);
        let p2 = inner.1.as_mut().poll(cx);

        match (p1, p2) {
            (Poll::Ready(_), Poll::Ready(_)) => Poll::Ready((inner.0.extract(), inner.1.extract())),
            _ => Poll::Pending,
        }
    }
}
