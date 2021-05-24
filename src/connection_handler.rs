use std::io::{Read, Write};
use std::net::TcpStream;

pub fn new_connection(stream: &mut TcpStream) {
    stream.set_nonblocking(true).unwrap();
    stream.set_nodelay(true).unwrap();
    stream.write(super::constants::MOTD.as_bytes()).unwrap();
    let mut session = super::user_handler::login(stream);
    while session.is_none() {
        session = super::user_handler::login(stream);
    }
    println!("{:?}", session);
    stream.write(&[0x07 as u8]).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    stream.write(&[0x07 as u8]).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    stream.write(&[0x07 as u8]).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    stream.write(format!("\r\nHELLO {}", session.unwrap().account.username).as_bytes()).unwrap();
}
