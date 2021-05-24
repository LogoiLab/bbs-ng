use std::io::{Read, Write};
use std::net::TcpStream;
pub fn splash(stream: &mut TcpStream) -> bool {
    let mut buf: Vec<u8> = vec![];
    let mut char_buf: [u8;1] = [0];
    stream.write(super::constants::MOTD.as_bytes()).unwrap();
    stream.write(b"\r\nWould you like to register or login? (r/l): ").unwrap();
    loop {
        match stream.read(&mut char_buf) {
            Ok(o) => {
                if o == 0 {
                    return false;
                }
                match char_buf[0] {
                    0xFF => (), 0xFC => (), 0xFD => (), 0x01 => (), 0x0A => (),
                    0x0D => {
                        for charac in &buf {
                            if charac.clone() as char == 'r' {
                                return true;
                            }
                            if charac.clone() as char == 'l' {
                                return false;
                            }
                        }
                        stream.write(b"\r\nInvalid option.").unwrap();
                        stream.write(b"\r\nWould you like to register or login? (r/l): ").unwrap();
                    },
                    /*0x0A => {
                        session.account.password = String::from(String::from_utf8_lossy(buf.as_slice()).trim());
                        break;
                    },*/
                    _ => {
                        buf.push(char_buf[0]);
                    }
                }
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            Err(_) => break,
        };
        char_buf = [0];
    }
    return false;
}
