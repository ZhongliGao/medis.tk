use futures::{future::OptionFuture, Future};
use mini_redis::client;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;
use std::{pin::Pin, time::Instant};
use tokio::stream::Stream;
use tokio::stream::StreamExt;
use tokio::sync::mpsc;

// Iteration
// #[tokio::main]
// async fn main() {
//     let (mut tx, mut rx) = mpsc::channel(10);

//     tokio::spawn(async move {
//         tx.send(1).await.unwrap();
//         tx.send(2).await.unwrap();
//         tx.send(3).await.unwrap();
//     });

//     while let Some(v) = rx.next().await {
//         println!("GOT = {:?}", v);
//     }
// }

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Publish some data
    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;
    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    let messages = subscriber
        .into_stream()
        .filter(|msg| match msg {
            Ok(msg) if msg.content.len() == 1 => true,
            _ => false,
        })
        .map(|msg| msg.unwrap().content)
        .take(3);

    // the stream is pinned to the stack
    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    tokio::spawn(async { publish().await });

    subscribe().await?;

    Ok(())
}

// The Stream trait is very similar to the Future trait.
// pub trait Stream {
//     type Item;

//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;

//     fn size_hint(&self) -> (usize, Option<usize>) {
//         (0, None)
//     }
// }

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            // Get a handle to the waker for the current task
            let waker = cx.waker().clone();
            let when = self.when;

            // Spawn a timer thread.
            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                waker.wake();
            });

            Poll::Pending
        }
    }
}

struct Interval {
    rem: usize,
    delay: Delay,
}

impl Stream for Interval {
    type Item = ();
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rem == 0 {
            // No more delays
            return Poll::Ready(None);
        }

        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                let when = self.delay.when + Duration::from_millis(10);
                self.delay = Delay { when };
                self.rem -= 1;
                Poll::Ready(Some(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

// use async_stream::stream;
// use std::time::{Duration, Instant};

// stream! {
//     let mut when = Instant::now();
//     for _ in 0..3 {
//         let delay = Delay { when };
//         delay.await;
//         yield ();
//         when += Duration::from_millis(10);
//     }
// }
