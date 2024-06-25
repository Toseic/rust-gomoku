

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;


pub enum MessageType {
    Init,
    Update,
    Win,
    Lose,
}



pub async fn collect_socket(socket_vec: & mut Vec<TcpStream>) -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;

    for _ in 0..2 { // todo: fix this
        let (socket, _) = listener.accept().await?;
        println!("receive a connection from {}", socket.peer_addr()?);
        socket_vec.push(socket);
    
    }
    Ok(())

}

// pub const MAP_X: usize = 100;
// pub const MAP_Y: usize = 40;



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

pub fn check_if_any_winner(
    _matrix: &Vec<Vec<u8>>,
    user_id: usize
) -> i32 {
    let rows = _matrix.len();
    let cols = _matrix[0].len();

    for i in 0..rows {
        let mut row_line_len = 0;
        for j in 0..cols {
            if j > 0 && _matrix[i][j] == user_id as u8 && _matrix[i][j-1] != user_id as u8 {
                row_line_len = 0;
            } else if _matrix[i][j] == user_id as u8 {
                row_line_len += 1;
            }
            if row_line_len >= 4 {
                return user_id as i32;
            }
        }
    }
    for j in 0..cols {
        let mut col_line_len = 0;
        for i in 0..rows {
            if i > 0 && _matrix[i][j] == user_id as u8 && _matrix[i-1][j] != user_id as u8 {
                col_line_len = 0;
            } else if _matrix[i][j] == user_id as u8 {
                col_line_len += 1;
            }
            if col_line_len >= 4 {
                return user_id as i32;
            }
        }
    }
    for i in 0..rows {
        for j in 0..cols {
            if i > 0 && j > 0 && _matrix[i][j] == user_id as u8 && _matrix[i-1][j-1] == user_id as u8 {
                let mut diag_line_len = 2;
                let mut x = j as i32 - 1;
                let mut y = i as i32 - 1;
                while x >= 0 && y >= 0 && _matrix[y as usize][x as usize] == user_id as u8 {
                    diag_line_len += 1;
                    x -= 1;
                    y -= 1;
                }
                x = j as i32 + 1;
                y = i as i32 + 1;
                while x < cols as i32 && y < rows as i32 && _matrix[y as usize][x as usize] == user_id as u8 {
                    diag_line_len += 1;
                    x += 1;
                    y += 1;
                }
                if diag_line_len >= 6 {
                    return user_id as i32;
                }
            }
            if i > 0 && j < cols-1 && _matrix[i][j] == user_id as u8 && _matrix[i-1][j+1] == user_id as u8 {
                let mut diag_line_len = 2;
                let mut x = j as i32 + 1;
                let mut y = i as i32 - 1;
                while x < cols as i32 && y >= 0 && _matrix[y as usize][x as usize] == user_id as u8 {
                    diag_line_len += 1;
                    x += 1;
                    y -= 1;
                }
                x = j as i32 - 1;
                y = i as i32 + 1;
                while x >= 0 && y < rows as i32 && _matrix[y as usize][x as usize] == user_id as u8 {
                    diag_line_len += 1;
                    x -= 1;
                    y += 1;
                }
                if diag_line_len >= 6 {
                    return user_id as i32;
                }
            }
        }
    }
    -1
}
