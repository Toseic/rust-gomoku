use game_client::client::{*};
use game_client::terminal::{reset, hide_cursor, set_mode, get_dimensions};

use std::thread;
use std::time::Duration;
use tokio::net::TcpStream;
// use game_server::webserver;



#[tokio::main]
async fn _game() {
    // 连接到服务器
    // return run_client().await;
    // return run_bot_client();
    // return launch_multi_client(256).await;
    let dim =  get_dimensions().unwrap();
    let map_x = dim.x;
    let map_y = dim.y ;
    // let map_y = 4;

    // let map_size = 15;
    let mut matrix: Vec::<Vec::<u8>> = vec![vec![0; map_x]; map_y];
    // init: set to 0
    for i in 0..map_y {
        for j in 0..map_y {
            matrix[i][j] = 0;
        }
    }

    set_mode(false);
    reset();
    hide_cursor();
    let sleep_duration = Duration::from_secs(2);

    // 使当前线程休眠2秒

    let size_ = map_x * map_y;
    let idx = 0;

    matrix[0][idx] = 1;
    matrix[0][idx+1] = 1;
    let mut game_map: Map = Map::new(size_ , map_x, map_y, map_x, map_y);
    convert_map_into_buffer(&matrix, & mut game_map);
    
    fresh_game_map(&game_map);


    println!("test");
    println!("rows: {}, cols: {}", dim.x, dim.y);

    // fresh_game_map(&buffer);
    thread::sleep(sleep_duration);
    // set_mode(true);
    // reset();

}


#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".to_string(); // 替换为服务器的 IP 地址
    let mut socket: TcpStream =TcpStream::connect(addr).await.unwrap();

    set_mode(false);
    reset();
    hide_cursor();

    println!("Connected to the server!");
    let sleep_duration = Duration::from_secs(2);
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
        } else {
            println!("update");
            receive_update_map(& mut socket, & mut matrix).await;
        }
        // generate map
        convert_map_into_buffer(&matrix, & mut game_map);
        fresh_game_map(&game_map);   
        
        // get and send input to server

        let input = get_user_cursor(&mut matrix, &mut game_map, map_x, map_y);
        // println!("input_x: {}, input_y: {}", input_x, input_y);
        send_input_to_server(& mut socket, & input).await;
        
        thread::sleep(sleep_duration);
    }
}