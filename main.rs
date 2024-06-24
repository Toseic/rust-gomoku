use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 绑定到局域网地址和端口
    let addr = "0.0.0.0:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;

    println!("Server running on {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Error handling client: {:?}", e);
            }
        });
    }
}

async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; 1024];

    loop {
        let n = socket.read(&mut buf).await?;

        if n == 0 {
            return Ok(());
        }

        // 打印接收到的数据
        println!("Received: {:?}", &buf[..n]);

        // 回显接收到的数据
        socket.write_all(&buf[..n]).await?;
    }
}
