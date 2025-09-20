use async_rust::thread_executor::block_thread_on;
use async_rust::thread_timer::ThreadTimer;
use core::fmt;
use std::time::{Duration, Instant, SystemTime, SystemTimeError, UNIX_EPOCH};

fn main() {
    // async fn this_function_returns_a_future() -> String {
    //     String::from("Inside an async function")
    // }

    // let this_block_is_a_future = async { String::from("Inside an async block") };

    // let string_from_block = block_thread_on(this_block_is_a_future);
    // let string_from_fn = block_thread_on(this_function_returns_a_future());

    // println!("{string_from_block}");
    // println!("{string_from_fn}");

    // // ---

    // let future_with_value = async {
    //     ThreadTimer::new(Duration::from_secs(1)).await;
    //     "This future returns a string".to_string()
    // };

    // let future_that_uses_that_value = async {
    //     let value = future_with_value.await;
    //     println!("Received: {value}");
    // };

    // block_thread_on(future_that_uses_that_value);

    // // ---

    // result_example();

    // // ---

    // let future = async {
    //     println!("Starting future"); // Only prints once
    //     ThreadTimer::new(Duration::from_secs(2)).await;
    //     ThreadTimer::new(Duration::from_secs(1)).await;
    //     println!("Ending future"); // Only prints once
    // };

    // block_thread_on(future);

    // ---

    let future = async {
        let now = Instant::now();
        ThreadTimer::new(Duration::from_secs(2)).await;
        ThreadTimer::new(Duration::from_secs(1)).await;
        now.elapsed().as_secs()
    };

    let time_taken = block_thread_on(future);
    println!("Time taken {time_taken} seconds");
}

#[derive(Debug)]
enum MyFutureError {
    TimeError,
    SecondsError,
}

impl fmt::Display for MyFutureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyFutureError::TimeError => write!(f, "Time went backwards"),
            MyFutureError::SecondsError => write!(f, "Something totally unexpected happened 😜"),
        }
    }
}

impl From<SystemTimeError> for MyFutureError {
    fn from(_: SystemTimeError) -> Self {
        MyFutureError::TimeError
    }
}

fn result_example() {
    async fn this_future_could_fail() -> Result<u64, MyFutureError> {
        let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        time.is_multiple_of(2)
            .then_some(time)
            .ok_or(MyFutureError::SecondsError)
    }

    async fn use_fallible_future() -> Result<(), MyFutureError> {
        let time = this_future_could_fail().await?;
        println!("{time} secs have passed since the Unix Epoch");
        Ok(())
    }

    match block_thread_on(use_fallible_future()) {
        Ok(()) => {}
        Err(message) => println!("Error: {message}"),
    }
}
