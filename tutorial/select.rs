use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::{net::TcpListener, process};

struct MySelect {
    rx1: oneshot::Receiver<&'static str>,
    rx2: oneshot::Receiver<&'static str>,
}

impl Future for MySelect {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
            println!("rx1 completed first with {:?}", val);
            return Poll::Ready(());
        }

        if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2 completed first with {:?}", val);
            return Poll::Ready(());
        }

        Poll::Pending
    }
}

// #[tokio::main]
// async fn main() {
//     let (tx1, rx1) = oneshot::channel();
//     let (tx2, rx2) = oneshot::channel();

//     // use tx1 and tx2

//     MySelect { rx1, rx2 }.await;
// }

// #[tokio::main]
// async fn main() {
//     let (tx, rx) = oneshot::channel();

//     // spawn a task taht sends a message over the oneshot
//     tokio::spawn(async move {
//         tx.send("done").unwrap();
//     });

//     tokio::select! {
//         socket = TcpStream::connect("localhost:3465") => {
//             println!("Socket connected {:?}", socket);
//         }
//         msg = rx => {
//             println!("received message first {:?}", msg);
//         }
//     }
// }

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let (tx, rx) = oneshot::channel();

//     tokio::spawn(async move {
//         tx.send(()).unwrap();
//     });

//     let mut listener = TcpListener::bind("localhost:3465").await?;

//     tokio::select! {
//         _ = async {
//             loop {
//                 let (socket, _) = listener.accept().await?;
//                 tokio::spawn(async move { process(socket) });
//             }

//             // Help the rust type inferencer out
//             Ok::<_, io::Error>(())
//         } => {}
//         _ = rx => {
//             println!("terminating accept loop");
//         }
//     }

//     Ok(())
// }

// #[tokio::main]
// async fn main() {
//     let (mut tx1, mut rx1) = mpsc::channel(128);
//     let (mut tx2, mut rx2) = mpsc::channel(128);

//     tokio::spawn(async move {
//         // Do something w/ `tx1` and `tx2`
//     });

//     tokio::select! {
//         Some(v) = rx1.recv() => {
//             println!("Got {:?} from rx1", v);
//         }
//         Some(v) = rx2.recv() => {
//             println!("Got {:?} from rx2", v);
//         }
//         else => {
//             println!("Both channels closed");
//         }
//     }
// }

// async fn race(data: &[u8], addr1: SocketAddr, addr2: SocketAddr) -> io::Result<()> {
//     tokio::select! {
//         Ok(_) = async {
//             let mut socket = TcpStream::connect(addr1).await?;
//             socket.write_all(data).await?;
//             Ok::<_, io::Error>(())
//         } => {}
//         Ok(_) = async {
//             let mut socket = TcpStream::connect(addr2).await?;
//             socket.write_all(data).await?;
//             Ok::<_, io::Error>(())
//         } => {}
//         else => {}
//     };

//     Ok(())
// }

// #[tokio::main]
// async fn main() {
//     let (tx1, mut rx1) = mpsc::channel(128);
//     let (tx2, mut rx2) = mpsc::channel(128);
//     let (tx3, mut rx3) = mpsc::channel(128);

//     loop {
//         let msg = tokio::select! {
//             Some(msg) = rx1.recv() => msg,
//             Some(msg) = rx2.recv() => msg,
//             Some(msg) = rx3.recv() => msg,
//             else => { break }
//         };

//         println!("Got {:?}", msg);
//     }

//     println!("All channels have been closed.");
// }

// async fn action(input: Option<i32>) -> Option<String> {
//     // If the input is `None`, return `None`.
//     // This could also be written as `let i = input?;`
//     let i = match input {
//         Some(input) => input,
//         None => return None,
//     };
//     // async logic here
// }

// #[tokio::main]
// async fn main() {
//     let (mut tx, mut rx) = tokio::sync::mpsc::channel(128);

//     let mut done = false;
//     let operation = action(None);
//     tokio::pin!(operation);

//     tokio::spawn(async move {
//         let _ = tx.send(1).await;
//         let _ = tx.send(3).await;
//         let _ = tx.send(2).await;
//     });

//     loop {
//         tokio::select! {
//             res = &mut operation, if !done => {
//                 done = true;

//                 if let Some(v) = res {
//                     println!("GOT = {}", v);
//                     return;
//                 }
//             }
//             Some(v) = rx.recv() => {
//                 if v % 2 == 0 {
//                     // `.set` is a method on `Pin`.
//                     operation.set(action(Some(v)));
//                     done = false;
//                 }
//             }
//         }
//     }
// }

fn main() {}
