use game_server::webserver::{*};
use tokio::net::TcpStream;


#[tokio::main]
async fn main() {
    let mut socket_vec: Vec<TcpStream> = Vec::new();
    collect_socket(& mut socket_vec).await.unwrap();


    //  init map
    let mut matrix: Vec::<Vec::<u8>> = vec![vec![0; 64]; 32];
    for i in 0..32 {
        for j in 0..64 {
            matrix[i][j] = 0;
        }
    }


    let mut idx = 0;
    let mut winner = -1;
    let mut change_vec = Vec::<(u8, u8, u8)>::new();
    let user_num = socket_vec.len();
    loop {
        let mut user_id = 0;
        for socket in &mut socket_vec {
            // send message type to user
            if idx == 0 {
                println!("send init message to user: {}", user_id);
                send_message_type(socket, MessageType::Init).await;
                // send map to user
                send_init_map(socket, &matrix, 64, 32, user_id+1).await; // todo: fix this
                println!("send init map to user: {}", user_id);
            } else {
                send_message_type(socket, MessageType::Update).await;
                send_update_vec(socket, &matrix, &change_vec, user_num as i32, user_id as i32).await;
                println!("send update map to user: {}", user_id);
                
            }

            // receive user's input
            let user_input = receive_user_input(socket).await;
            println!("user input: {:?}", user_input);
            // modify map
            // todo: check input is valid or not
            matrix[user_input[1] as usize][user_input[0] as usize] = 1 + user_id as u8;
            change_vec.push((user_input[0], user_input[1], user_id as u8));
            

            // user win or continue
            // check if any user win or not 
            winner = check_if_any_winner(&matrix,1 + user_id as usize);
            if winner != -1 {
                println!("user {} win", user_id);
                break;
            }

            user_id += 1;

        }
        if winner != -1 {
            break;
        }
        idx += 1;
    }
    let mut user_id = 1;
    // send messages to all users
    for socket in &mut socket_vec {
        // send message type to user
        if user_id == winner {
            println!("send win message to user: {}", user_id);
            send_message_type(socket, MessageType::Win).await;
        } else {
            println!("send lose message to user: {}", user_id);
            send_message_type(socket, MessageType::Lose).await;
        }

        user_id += 1;

    }
}