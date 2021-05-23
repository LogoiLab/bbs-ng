use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn new_connection(stream: TcpStream) {
    stream.set_nonblocking(true).unwrap();

    let mut buf: Vec<u8> = vec![];
    let mut reader = BufReader::new(stream);

    loop {
        //let mut iter_buf: [u8;16] = [0;16];
        match reader.read_until(0x0D, &mut buf) {
            Ok(o) => {
                if o == 0 {
                    break;
                }
                println!("{}", String::from_utf8_lossy(buf.as_slice()).trim());
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            Err(_) => break,
        };
        buf = vec![];
    }
}
