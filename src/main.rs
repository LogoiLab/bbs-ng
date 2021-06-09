pub mod board_handler;
pub mod connection_handler;
pub mod constants;
pub mod menu_handler;
pub mod user_handler;

use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:885").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    connection_handler::new_connection(&mut stream);
                });
            }
            Err(_) => { /* connection failed */ }
        }
    }
}
