use game_client::client::{*};
use game_client::terminal::{reset, hide_cursor, set_mode, get_dimensions};

use tokio::net::TcpStream;


#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".to_string(); // todo: fix this
    let mut socket: TcpStream =TcpStream::connect(addr).await.unwrap();

    set_mode(false);
    reset();
    hide_cursor();

    println!("Connected to the server!");
    let mut message_type = MessageType::Init;
    let map_x = 64;
    let map_y = 32;
    let dim =  get_dimensions().unwrap();
    let term_x = dim.x;
    let term_y = dim.y ;

    let mut game_map: Map = Map::new(term_x * term_y ,map_x, map_y, term_x, term_y);
    let mut matrix: Vec::<Vec::<u8>> = vec![vec![0; 64]; 32];

    loop {
        // receive message type
        receive_message_type(& mut socket, & mut message_type).await;


        // receive server's map
        if message_type == MessageType::Init {
            println!("init");
            receive_init_map(& mut socket, & mut matrix).await;
        } else if message_type == MessageType::Update {
            println!("update");
            receive_update_map(& mut socket, & mut matrix).await;
        } else if message_type == MessageType::Win ||
        message_type == MessageType::Lose {
            break;
        }
        // generate map
        convert_map_into_buffer(&matrix, & mut game_map);
        fresh_game_map(&game_map);   
        
        // get and send input to server

        let input = get_user_cursor(&mut matrix, &mut game_map, map_x, map_y);
        send_input_to_server(& mut socket, & input).await;
        
        // thread::sleep(sleep_duration);
    }
    set_mode(true);
    reset();

    if message_type == MessageType::Win {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}