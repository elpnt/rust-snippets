use futures::{future, SinkExt, StreamExt};
use tokio::io;
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .expect("you need one specific socket address");

    let mut stream = TcpStream::connect(&addr).await?;
    let (tx, rx) = stream.split();

    let mut sink = FramedWrite::new(rx, BytesCodec::new());
    let mut stream = FramedRead::new(tx, BytesCodec::new())
        .filter_map(|i| match i {
            Ok(i) => future::ready(Some(i.freeze())),
            Err(e) => {
                println!("failed to read from socket: error={}", e);
                future::ready(None)
            }
        })
        .map(Ok);

    let mut stdin =
        FramedRead::new(io::stdin(), BytesCodec::new()).map(|i| i.map(|bytes| bytes.freeze()));
    let mut stdout = FramedWrite::new(io::stdout(), BytesCodec::new());
    match future::join(sink.send_all(&mut stdin), stdout.send_all(&mut stream)).await {
        (Err(e), _) | (_, Err(e)) => Err(e.into()),
        _ => Ok(()),
    }
}
