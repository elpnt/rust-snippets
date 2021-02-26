//! `cargo run --bin echo-server`
//! or specify the address manually
//! `cargo run --bin echo-server 127.0.0.1:8080`

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8888".into());

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (mut socket, cli_addr) = listener.accept().await?;
        println!("connected {}", cli_addr);

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    println!("close {}", cli_addr);
                    return;
                }

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
                println!("{}: {}", cli_addr, String::from_utf8(buf.clone()).unwrap());
            }
        });
    }
}
