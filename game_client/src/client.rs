

use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use ansi_term::Colour;
use ansi_term::Style;

#[derive(Clone)]
pub struct MapUnit {
    ch: char,
    color: Colour,
    pub bold: bool,
}

pub struct Map {
    pub buffer: Vec<MapUnit>,
    pub map_x: usize,
    pub map_y: usize,
    pub term_x: usize,
    pub term_y: usize,
    pub highlight_x: usize,
    pub highlight_y: usize,
}

impl MapUnit {
    pub fn new() -> MapUnit {
        MapUnit {
            ch: ' ',
            color: Colour::White,
            bold: false,
        }
    }
}

impl Map {
    pub fn new(size_: usize, x: usize, y: usize, term_x: usize, term_y: usize) -> Map {
        if x > term_x || y > term_y {
            panic!("map size is larger than terminal size");
        }
        Map {
            buffer: vec![MapUnit::new(); size_],
            highlight_x: 0,
            highlight_y: 0,
            map_x: x,
            map_y: y,
            term_x: term_x,
            term_y: term_y,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MessageType {
    Init,
    Update,
    Win,
    Lose,
}

pub fn fresh_game_map(map: &Map) {
    use std::io::Write;
    let buffer = &map.buffer;
    let mut stdout_buffer = Vec::<u8>::new();
    clearscreen::clear().expect("failed to clear screen");
    for i in 0..buffer.len() {
        let paint_str = match buffer[i].bold {
            true => Style::new().bold().fg(buffer[i].color).paint(buffer[i].ch.to_string()),
            false => Style::new().fg(buffer[i].color).paint(buffer[i].ch.to_string()),
        };
        // insert into stdout_buffer
        for j in 0..paint_str.to_string().len() {
            stdout_buffer.push(paint_str.to_string().as_bytes()[j]);
        }
    }
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(stdout_buffer.as_slice()).unwrap();
    stdout.flush().unwrap();
}

pub fn receive_buffer(buffer: &mut [u8]) {
    for i in 0..1024*32 {
        buffer[i] = 'a' as u8;
    }

}


pub fn convert_map_into_buffer(
    matrix: & Vec::<Vec::<u8>> ,
    map: &mut Map
) {
    let mut idx = 0;
    let buffer = &mut map.buffer;

    // up border
    for _ in 0..map.map_x+2 {
        buffer[idx].ch = '@';
        idx += 1;
    }
    if idx % map.term_x != 0 {
        loop {
            buffer[idx].ch = ' ';
            idx += 1;
            if idx % map.term_x == 0 {
                break;
            }
        }
    }

    for i in 0..map.map_y {
        buffer[idx].ch = '@';
        idx += 1;
        for j in 0..map.map_x {
            match matrix[i][j] {
                0 => buffer[idx].ch = ' ',
                // 0 => buffer[idx].ch = '○',
                1 => buffer[idx].ch = 'X',
                2 => buffer[idx].ch = '#',
                _ => buffer[idx].ch = '?',
            };
            if (map.highlight_y == i) && (map.highlight_x == j) {
                if  buffer[idx].ch == ' ' {
                    buffer[idx].ch = '○';
                }
            } 
            idx += 1;
        }

        buffer[idx].ch = '@';
        idx += 1;

        if idx % map.term_x == 0 {
            continue;
        }
        loop {
            buffer[idx].ch = ' ';
            idx += 1;
            if idx % map.term_x == 0 {
                break;
            }
        }
        
    }

    for _ in 0..map.map_x+2 {
        buffer[idx].ch = '@';
        idx += 1;
    }
    if idx % map.term_x != 0 {
        loop {
            buffer[idx].ch = ' ';
            idx += 1;
            if idx % map.term_x == 0 {
                break;
            }
        }
    }

}

pub fn set_unit_as_highlight(x_idx: usize, y_idx: usize, game_map: &mut Map, map_x: usize) {
    let idx = game_map.highlight_y * map_x + game_map.highlight_x;
    game_map.buffer[idx].bold = false;
    game_map.highlight_x = x_idx;
    game_map.highlight_y = y_idx;
    let idx = game_map.highlight_y * map_x + game_map.highlight_x;
    game_map.buffer[idx].bold = true;

    
}

pub fn get_user_cursor(
    matrix: & mut Vec::<Vec::<u8>> ,
    game_map: &mut Map, 
    map_x: usize,
    map_y: usize,

) -> [u8; 2]
{
    use std::io::Read;
    let mut stdin = std::io::stdin();
    let mut buffer = [0u8; 1];
    let mut x_idx = 1;
    let mut y_idx = 1;
    set_unit_as_highlight(x_idx, y_idx, game_map, map_x);
    // user input
    loop {
        stdin.read_exact(&mut buffer).unwrap();
        let c = buffer[0] as char;
        match c {
            'w' => {
                y_idx = (y_idx + map_y - 1) % map_y;
            },
            's' => {
                y_idx = (y_idx + 1) % map_y;
            },
            'a' => {
                x_idx = (x_idx + map_x - 1) % map_x;
            },
            'd' => {
                x_idx = (x_idx + 1) % map_x;
            },
            '\n' => {
                break;
            },
            _ => {
                continue;
            }
        }
        set_unit_as_highlight(x_idx, y_idx, game_map, map_x);
        convert_map_into_buffer(&matrix, game_map);
        fresh_game_map(&game_map);
    }
    [x_idx as u8, y_idx as u8]
}


pub async fn comm_with_server(input: &[u8; 2]) -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080".to_string(); 
    let socket = TcpStream::connect(addr).await?;

    println!("Connected to the server!");

    let (mut rd, mut wr) = io::split(socket);

    wr.write_all(&input.to_vec()).await.expect("Failed to write to socket");

    let mut buf = vec![0; 1024]; // TODO: fix it

    let n = rd.read(&mut buf).await?;


    println!("Received from server: {:?}", &buf[..n]);

    Ok(())
}



pub async fn conn_socket(socket: & mut TcpStream) -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080".to_string();
    *socket = TcpStream::connect(addr).await.unwrap();

   Ok(())
}



pub async fn receive_message_type(socket: &mut tokio::net::TcpStream, 
        _type: &mut MessageType){
    let mut buffer = [0; 1];
    socket.read(&mut buffer).await.unwrap();

    *_type = match buffer[0] {
        0 => MessageType::Init,
        1 => MessageType::Update,
        2 => MessageType::Win,
        3 => MessageType::Lose,
        _ => MessageType::Init,
    }
}

pub async fn receive_init_map(socket: &mut tokio::net::TcpStream, 
        matrix: &mut Vec<Vec<u8>>) {
    // todo: cant always be 64, 32
    let mut _buffer: [u8; 64*32] = [0; 64*32];
    socket.read(&mut _buffer).await.unwrap();
    for i in 0..32 {
        for j in 0..64 {
            let idx = i * 64 + j;
            matrix[i][j] = _buffer[idx] ;
        }
    }
}

pub async fn receive_update_map(socket: &mut tokio::net::TcpStream, 
        matrix: &mut Vec<Vec<u8>>) {
    let mut _buffer: [u8; 128] = [0; 128]; // todo: if more than 128
    socket.read(&mut _buffer).await.unwrap();
    let update_num = _buffer[0] as usize;
    // print!("receive: ");
    for update in 0..update_num {
        let x = _buffer[update * 3 + 1] as usize;
        let y = _buffer[update * 3 + 2] as usize;
        matrix[y][x] = _buffer[update * 3 + 3];
        // print!("({}, {}, {}), ", x, y, matrix[y][x]);
    }
    // println!("");
    

    
}

pub async fn send_input_to_server(socket: &mut tokio::net::TcpStream, 
        input: &[u8; 2]) {
    socket.write_all(&input.to_vec()).await.unwrap();
}