use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::Duration;

pub struct ThreadTimer {
    duration: Duration,
    join_handle: Option<JoinHandle<()>>,
    waker: Arc<Mutex<Waker>>,
    is_complete: Arc<Mutex<bool>>,
}

impl ThreadTimer {
    pub fn new(duration: Duration) -> ThreadTimer {
        Self {
            duration,
            join_handle: None,
            waker: Arc::new(Mutex::new(Waker::noop().clone())),
            is_complete: Arc::new(Mutex::new(false)),
        }
    }
}

impl Future for ThreadTimer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let timer = self.get_mut();
        *timer.waker.lock().unwrap() = cx.waker().clone();

        if timer.join_handle.is_none() {
            let duration = timer.duration;
            let waker = timer.waker.clone();
            let is_complete = timer.is_complete.clone();
            timer.join_handle = Some(spawn(move || {
                sleep(duration);
                *is_complete.lock().unwrap() = true;
                waker.lock().unwrap().wake_by_ref();
            }));
        }

        match *timer.is_complete.lock().unwrap() {
            true => Poll::Ready(()),
            false => Poll::Pending,
        }
    }
}
