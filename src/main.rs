pub mod connection_handler;

use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:885").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    connection_handler::new_connection(stream);
                });
            }
            Err(_) => { /* connection failed */ }
        }
    }
}
