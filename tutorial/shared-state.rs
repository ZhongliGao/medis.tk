// use std::sync::Mutex;

// struct CanIncrement {
//     mutex: Mutex<i32>,
// }
// impl CanIncrement {
//     // This function is not marked async.
//     fn increment(&self) {
//         let mut lock = self.mutex.lock().unwrap();
//         *lock += 1;
//     }
// }

// async fn increment_and_do_stuff(can_incr: &CanIncrement) {
//     can_incr.increment();
//     do_something_async().await;
// }

fn main() {}

// The primary feature of the Tokio mutex is that it can be held across an .await without any issues.
// That said, an asynchronous mutex is more expensive than an ordinary mutex, and it is typically better to use one of the two other approaches.

// use tokio::sync::Mutex;

// async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
//     let mut lock = mutex.lock().await;
//     *lock += 1;

//     do_something_async().await;
// }
