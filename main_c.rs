use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 连接到服务器
    let addr = "127.0.0.1:8080".to_string(); // 替换为服务器的 IP 地址
    let mut socket = TcpStream::connect(addr).await?;

    println!("Connected to the server!");

    let (mut rd, mut wr) = io::split(socket);

    tokio::spawn(async move {
        let mut stdin = io::stdin();
        let mut buf = vec![0; 1024];

        loop {
            let n = stdin.read(&mut buf).await.expect("Failed to read from stdin");
            if n == 0 {
                return;
            }

            wr.write_all(&buf[0..n]).await.expect("Failed to write to socket");
        }
    });

    let mut buf = vec![0; 1024];
    loop {
        let n = rd.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }

        println!("Received from server: {:?}", &buf[..n]);
    }
}
