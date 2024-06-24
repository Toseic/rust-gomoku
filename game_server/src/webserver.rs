

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
// use std::sync::{Arc, Mutex};

pub async fn server() -> Result<(), Box<dyn Error>> {
    // 绑定到局域网地址和端口
    let addr = "0.0.0.0:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;

    println!("Server running on {}", addr);
    // let user_input_vector = Arc::new(Mutex::new(Vec::<u8>::new()));
    // let shared_vector1 = Arc::clone(&user_input_vector);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Error handling client: {:?}", e);
            }
        });
    }
}



pub async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
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

pub async fn collect_socket(socket_vec: & mut Vec<TcpStream>) -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;

    for _ in 0..2 {
        let (socket, _) = listener.accept().await?;
        println!("receive a connection from {}", socket.peer_addr()?);
        socket_vec.push(socket);
    
    }
    Ok(())

}


pub async fn send_id_to_user() -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;

    println!("Server running on {}", addr);

    for idx in 0..2 {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1];
            buf[0] = idx;
            // 回显接收到的数据
            let _ = socket.write_all(&buf[..1]).await;
            
        });
    }
    Ok(())
}

pub fn get_user_input() {

}



// use webserver::webserver::server;
pub const MAP_X: usize = 100;
pub const MAP_Y: usize = 40;

pub enum MessageType {
    Init,
    Update,
    Win,
    Lose,
}

pub async fn send_message_type(socket: &mut tokio::net::TcpStream, message_type: MessageType) {
    let buffer: [u8; 1] = match message_type {
        MessageType::Init => [0],
        MessageType::Update => [1],
        MessageType::Win => [2],
        MessageType::Lose => [3],
    };
    socket.write_all(&buffer).await.unwrap();
}

pub async fn send_init_map(
    socket: &mut tokio::net::TcpStream, _matrix: &Vec<Vec<u8>>, 
    map_x: usize, map_y: usize,
    user_idx: i32
) {
    let mut matrix: Vec<u8> = vec![0; map_x * map_y];
    for i in 0..map_y {
        for j in 0..map_x {
            let idx = i * map_x + j;
            if _matrix[i][j] == user_idx as u8 {
                matrix[idx] = 1;
            } else if _matrix[i][j] == 0 {
                matrix[idx] = 0;
            } else {
                matrix[idx] = 2;
            }
        }
    }
    
    socket.write_all(&matrix).await.unwrap();
}

pub async fn send_update_vec(
    socket: &mut tokio::net::TcpStream, _matrix: &Vec<Vec<u8>>, 
    user_input_vec: &Vec<(u8, u8, u8)>,
    user_num: i32,
    user_idx: i32
) {
    let mut buffer: Vec<u8> = vec![0; 128];
    // extract last `user_num` items
    let vec_len = user_input_vec.len();
    buffer[0] = 3 * user_num as u8 + 1;
    let mut idx = 0;
    for i in vec_len-user_num as usize..vec_len {
        let (x, y, uid) = user_input_vec[i];
        buffer[3*idx as usize + 1] = x;
        buffer[3*idx as usize + 2] = y;
        if user_idx == uid.into() {
            buffer[3*idx as usize + 3] = 1;
        } else {
            buffer[3*idx as usize + 3] = 2;
        }
        idx += 1;
    }

    for user in  0..user_num {
        print!("({},{},{})", buffer[3*user as usize + 1], buffer[3*user as usize + 2], buffer[3*user as usize + 3]);

    }
    println!();
    
    socket.write_all(&buffer).await.unwrap();
}


pub async fn receive_user_input(socket: &mut tokio::net::TcpStream) -> [u8; 2] {
    let mut buffer = [0; 2];
    socket.read(&mut buffer).await.unwrap();
    buffer
}