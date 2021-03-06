use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};

use bcrypt::{DEFAULT_COST};
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Session {
    pub ip_addr: SocketAddr,
    pub account: Account
}

#[derive(Debug, Default)]
pub struct Account {
    pub id: Option<u32>,
    pub username: String,
    pub password: String,
    pub signature: Option<String>
}

pub fn register(stream: &mut TcpStream) -> Option<Session> {
    let mut session: Session = Session{
        ip_addr: stream.peer_addr().unwrap(),
        account: Account::default()
    };

    let mut buf: Vec<u8> = vec![];
    let mut char_buf: [u8;1] = [0];

    stream.write(b"\r\nUsername: ").unwrap();
    stream.write(&[0xff as u8, 0xfc as u8, 0x01 as u8]).unwrap();
    loop {
        match stream.read(&mut char_buf) {
            Ok(o) => {
                if o == 0 {
                    return None;
                }
                match char_buf[0] {
                    0xFA ..= 0xFF => (), 0x01 => (), 0x0A => (),
                    0x0D => {
                        let username = String::from(String::from_utf8_lossy(buf.as_slice()).trim());
                        if username.len() == 0 {
                            stream.write(b"\r\nYou must enter a username.").unwrap();
                            return None;
                        }
                        session.account.username = username;
                        buf = vec![];
                        break;
                    },
                    /*0x0A => {
                        session.account.username = String::from(String::from_utf8_lossy(buf.as_slice()).trim());
                        buf = vec![];
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

    stream.write(b"\r\nPassword: ").unwrap();
    stream.write(&[0xff as u8, 0xfb as u8, 0x01 as u8]).unwrap();
    let mut password: String = "".to_string();
    loop {
        match stream.read(&mut char_buf) {
            Ok(o) => {
                if o == 0 {
                    return None;
                }
                match char_buf[0] {
                    0xFF => (), 0xFC => (), 0xFD => (), 0x01 => (), 0x0A => (),
                    0x0D => {
                        password = String::from(String::from_utf8_lossy(buf.as_slice()).trim());
                        if password.len() == 0 {
                            stream.write(b"\r\nYou must enter a password.\r\n").unwrap();
                            return None;
                        }
                        //let mut salt = [0u8; 16];
                        //thread_rng().fill(&mut salt[..]);
                        //password = bcrypt::hash_with_salt(password, DEFAULT_COST, &salt).unwrap().to_string();
                        //session.account.password = password;
                        break;
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
    stream.write(&[0xff as u8, 0xfc as u8, 0x01 as u8]).unwrap();

    stream.write(b"\r\nConfirm Password: ").unwrap();
    stream.write(&[0xff as u8, 0xfb as u8, 0x01 as u8]).unwrap();
    let mut password2: String = "".to_string();
    loop {
        match stream.read(&mut char_buf) {
            Ok(o) => {
                if o == 0 {
                    return None;
                }
                match char_buf[0] {
                    0xFF => (), 0xFC => (), 0xFD => (), 0x01 => (), 0x0A => (),
                    0x0D => {
                        password2 = String::from(String::from_utf8_lossy(buf.as_slice()).trim());
                        if password2.len() == 0 {
                            stream.write(b"\r\nYou must enter a password.\r\n").unwrap();
                            return None;
                        }
                        if password != password2 {
                            return None;
                        }

                        let mut salt = [0u8; 16];
                        thread_rng().fill(&mut salt[..]);
                        session.account.password = bcrypt::hash_with_salt(password2, DEFAULT_COST, &salt).unwrap().to_string();
                        break;
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

    stream.write(&[0xff as u8, 0xfc as u8, 0x01 as u8]).unwrap();

    stream.write(b"\r\nWould you like to create a signature[y/(n)]: ").unwrap();
    let mut choice: String = "".to_string();
    loop {
        match stream.read(&mut char_buf) {
            Ok(o) => {
                if o == 0 {
                    return None;
                }
                match char_buf[0] {
                    0xFF => (), 0xFC => (), 0xFD => (), 0x01 => (), 0x0A => (),
                    0x0D => {
                        choice = String::from(String::from_utf8_lossy(buf.as_slice()).trim());
                        if choice.contains("y") {
                            stream.write(b"\r\nSignature: ").unwrap();
                            let mut sig: String = "".to_string();
                            loop {
                                match stream.read(&mut char_buf) {
                                    Ok(o) => {
                                        if o == 0 {
                                            return None;
                                        }
                                        match char_buf[0] {
                                            0xFF => (), 0xFC => (), 0xFD => (), 0x01 => (), 0x0A => (),
                                            0x0D => {
                                                sig = String::from(String::from_utf8_lossy(buf.as_slice()).trim());
                                                if sig.len() > 0 {
                                                    session.account.signature = Some(sig);
                                                } else {
                                                    session.account.signature = None;
                                                }
                                                break;
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
                        } else {
                            session.account.signature = None;
                        }
                        break;
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
    return Some(session);
}

pub fn login(stream: &mut TcpStream) -> Option<Session> {
    let mut session: Session = Session{
        ip_addr: stream.peer_addr().unwrap(),
        account: Account::default()
    };
    let mut buf: Vec<u8> = vec![];
    let mut char_buf: [u8;1] = [0];

    stream.write(b"\r\nUsername: ").unwrap();
    stream.write(&[0xff as u8, 0xfc as u8, 0x01 as u8]).unwrap();
    loop {
        match stream.read(&mut char_buf) {
            Ok(o) => {
                if o == 0 {
                    return None;
                }
                match char_buf[0] {
                    0xFA ..= 0xFF => (), 0x01 => (), 0x0A => (),
                    0x0D => {
                        let username = String::from(String::from_utf8_lossy(buf.as_slice()).trim());
                        if username.len() == 0 {
                            stream.write(b"\r\nYou must enter a username.").unwrap();
                            return None;
                        }
                        session.account.username = username;
                        buf = vec![];
                        break;
                    },
                    /*0x0A => {
                        session.account.username = String::from(String::from_utf8_lossy(buf.as_slice()).trim());
                        buf = vec![];
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
    stream.write(b"\r\nPassword: ").unwrap();
    stream.write(&[0xff as u8, 0xfb as u8, 0x01 as u8]).unwrap();
    loop {
        match stream.read(&mut char_buf) {
            Ok(o) => {
                if o == 0 {
                    return None;
                }
                match char_buf[0] {
                    0xFF => (), 0xFC => (), 0xFD => (), 0x01 => (), 0x0A => (),
                    0x0D => {
                        let mut password = String::from(String::from_utf8_lossy(buf.as_slice()).trim());
                        if password.len() == 0 {
                            stream.write(b"\r\nYou must enter a password.\r\n").unwrap();
                            return None;
                        }
                        let mut salt = [0u8; 16];
                        thread_rng().fill(&mut salt[..]);
                        password = bcrypt::hash_with_salt(password, DEFAULT_COST, &salt).unwrap().to_string();
                        session.account.password = password;
                        break;
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
    stream.write(&[0xff as u8, 0xfc as u8, 0x01 as u8]).unwrap();
    return Some(session);
}

impl Account {
    pub fn read_signature(&self) -> Option<String> {
        return self.signature.clone();
    }
    pub fn update_signature(&mut self, new_sig: String) {
        self.signature = Some(new_sig);
    }
    pub fn delete_signature(&mut self) {
        self.signature = None;
    }

}
