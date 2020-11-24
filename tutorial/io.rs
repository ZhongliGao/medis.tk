// use tokio::fs::File;
// use tokio::io::{self, AsyncReadExt};

// async fn red
// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut f = File::open("foo.txt").await?;
//     let mut buffer = [0; 10];

//     // read up to 10 bytes
//     let n = f.read(&mut buffer[..]).await?;

//     println!("The bytes: {:?}", &buffer[..n]);
//     Ok(())
// }

// async fn read_to_end()
// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut f = File::open("foo.txt").await?;
//     let mut buffer = Vec::new();

//     // read the whole file
//     f.read_to_end(&mut buffer).await?;
//     Ok(())
// }

// async fn write()
// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut file = File::create("foo.txt").await?;

//     // Writes some prefix of the byte string, but not necessarily all of it.
//     let n = file.write(b"some bytes").await?;

//     println!("Wrote the first {} bytes of 'some bytes'.", n);
//     Ok(())
// }

// async fn write_all()
// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut buffer = File::create("foo.txt").await?;

//     buffer.write_all(b"some bytes").await?;
//     Ok(())
// }

// helper
// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut reader: &[u8] = b"Hello";
//     let mut file = File::create("foo.txt").await?;

//     io::copy(&mut reader, &mut file).await?;
//     Ok(())
// }

// echo server
// splitting a reader + writer
// use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpStream;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let socket = TcpStream::connect("127.0.0.1:6142").await?;
//     let (mut rd, mut wr) = io::split(socket);

//     // write data in the background
//     let write_task = tokio::spawn(async move {
//         wr.write_all(b"hello\r\n").await?;
//         wr.write_all(b"world\r\n").await?;

//         Ok::<_, io::Error>(())
//     });

//     let mut buf = vec![0; 128];

//     loop {
//         let n = rd.read(&mut buf).await?;

//         if n == 0 {
//             break;
//         }
//         println!("GOT {:?}", &buf[..n]);
//     }

//     Ok(())
// }

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// manual copying
#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // Return value of `Ok(0)` signifies that the remote has
                    // closed
                    Ok(0) => return,
                    Ok(n) => {
                        // Copy the data back to socket
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // Unexpected socket error. There isn't much we can
                            // do here so just stop processing.
                            return;
                        }
                    }
                    Err(_) => {
                        // Unexpected socket error. There isn't much we can do
                        // here so just stop processing.
                        return;
                    }
                }
            }
        });
    }
}
