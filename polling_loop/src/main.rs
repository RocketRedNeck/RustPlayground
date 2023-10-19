use async_std::task;
use async_std::task::Poll;
use std::time::{Duration, Instant};
use std::thread;
use futures_lite::future::FutureExt;

fn main() {
    let start_time = Instant::now();

    demo_waiting_for_two_async_fns();

    println!("Program finished in {} ms", start_time.elapsed().as_millis());
}

fn demo_waiting_for_two_async_fns() {
    // Create context (if one does not exit naturally)
    let waker = futures::task::noop_waker_ref();
    let mut cx = std::task::Context::from_waker(waker);

    // Create a future
    let sleeper1 = first_sleeper();
    let sleeper2 = second_sleeper();

    // Move it around as needed
    // Pin it
    let mut pin1 = Box::pin(sleeper1);
    let mut pin2 = Box::pin(sleeper2);
    // Start polling

    let mut one_done = 0;
    let mut two_done = 0;
    loop {
        if 0 == one_done {
            match pin1.poll(&mut cx) {
                Poll::Ready(()) => {
                    one_done = 1;
                }
                _ =>{}
            }
        }
        if 0 == two_done {
            match pin2.poll(&mut cx) {
                Poll::Ready(()) => {
                    two_done = 1;
                }
                _ =>{}
            }
        }
        if one_done + two_done < 2 {
            println!("Waiting on sleepers: x = {}", one_done + two_done);
        }
        else
        {
            break;
        }
    }
}


async fn first_sleeper() {
    sleep_and_print(1, 2000).await;
}

async fn second_sleeper() {
    sleep_and_print(2, 1000).await;
}

/// This utility function simply goes to sleep for a specified time
/// and then prints a message when it is done.
async fn sleep_and_print(future_number: u32, sleep_millis: u64) {
    let sleep_duration = Duration::from_millis(sleep_millis);
    // Note we are using async-std's `task::sleep` here, not
    // thread::sleep. We must not block the thread!
    task::sleep(sleep_duration).await;
    println!("Future {} slept for {} ms on {:?}", future_number, sleep_millis, thread::current().id());
}