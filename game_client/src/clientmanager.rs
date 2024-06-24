
pub mod base1 {
    use tokio::net::TcpStream;
    use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
    use std::error::Error;
    use tokio::time::{self, Duration};


    pub async fn run_client() -> Result<(), Box<dyn Error>> {
        // 连接到服务器
        let addr = "127.0.0.1:8080".to_string(); // 替换为服务器的 IP 地址
        let socket = TcpStream::connect(addr).await?;
    
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
    


    pub async fn _run_bot_client() -> Result<(), Box<dyn Error>> {
        // 连接到服务器
        let addr = "127.0.0.1:8080".to_string(); // 替换为服务器的 IP 地址
        let socket = TcpStream::connect(addr).await?;
    
        println!("Connected to the server!");
    
        let (mut rd, mut wr) = io::split(socket);
    
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(1)); // 每5秒发送一次
            let message = "Hello from client!";
    
            loop {
                interval.tick().await;
                if let Err(_e) = wr.write_all(message.as_bytes()).await {
                    // eprintln!("Failed to write to socket: {:?}", e);
                    eprint!("x");
                    return;
                }
                print!("*");
            }
        });
    
        let mut buf = vec![0; 128];
        loop {
            let n = rd.read(&mut buf).await?;
            if n == 0 {
                return Ok(());
            }
    
            // println!("Received from server: {:?}", &buf[..n]);
        }
    }

    // pub async fn run_bot_client() -> Result<(), Box<dyn Error>> {
    //     let result = async {
    //         _run_bot_client()?;
    //         Ok(())
    //     }.await;
    //     result
        
    // }

    pub async fn launch_multi_client(client_num: i32) -> Result<(), Box<dyn Error>> {
        let mut handles = vec![];

        // 创建 10 个并发任务
        for i in 0..client_num {
            let handle = tokio::spawn(async move {
                if let Err(e) = _run_bot_client().await {
                    eprintln!("Client {} encountered an error: {:?}", i, e);
                }
            });
            handles.push(handle);
        }
    
        // 等待所有任务完成
        for handle in handles {
            handle.await?;
        }
    
        Ok(())
    }
}
