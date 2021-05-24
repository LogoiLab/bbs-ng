use std::io::{Read, Write};
use std::net::TcpStream;
use super::user_handler::Session;
pub fn new_connection(stream: &mut TcpStream) {
    stream.set_nonblocking(true).unwrap();
    stream.set_nodelay(true).unwrap();
    let mut session: Option<Session>;
    if super::menu_handler::splash(stream) {
        session = super::user_handler::register(stream);
        while session.is_none() {
            stream.write(b"\r\nPasswords did not match... Try again.").unwrap();
            super::user_handler::register(stream);
        }
    } else {
        session = super::user_handler::login(stream);
        while session.is_none() {
            session = super::user_handler::login(stream);
        }
    }
    println!("{:?}", session);
    stream.write(&[0x07 as u8]).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    stream.write(&[0x07 as u8]).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    stream.write(&[0x07 as u8]).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    let verified_session = session.unwrap();
    stream.write(format!("\r\nHELLO {}", verified_session.account.username).as_bytes()).unwrap();
}
