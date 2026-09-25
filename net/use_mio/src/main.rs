use std::collections::HashMap;
use std::io::{self, Read, Write};

use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn main() -> io::Result<()> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    let addr = "127.0.0.1:8080".parse().unwrap();

    // 服务器监听器
    let mut server = TcpListener::bind(addr)?;
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    // 客户端连接
    let mut client = TcpStream::connect(addr)?;
    poll.registry().register(
        &mut client,
        CLIENT,
        Interest::READABLE | Interest::WRITABLE,
    )?;

    // 已接受的服务端连接：Token -> 流
    let mut connections: HashMap<Token, TcpStream> = HashMap::new();
    let mut next_token = 2usize;

    let mut client_sent = false;

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                // 监听器可读：有新连接
                SERVER => loop {
                    match server.accept() {
                        Ok((mut conn, _addr)) => {
                            let token = Token(next_token);
                            next_token += 1;
                            poll.registry().register(
                                &mut conn,
                                token,
                                Interest::READABLE,
                            )?;
                            println!("server: accepted as {token:?}");
                            connections.insert(token, conn);
                        }
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                        Err(e) => return Err(e),
                    }
                },

                // 客户端：先发一次，再读回声
                CLIENT => {
                    if !client_sent && event.is_writable() {
                        client.write_all(b"hello from client\n")?;
                        // 发完只关注可读，避免“一直可写”导致忙循环
                        poll.registry().reregister(
                            &mut client,
                            CLIENT,
                            Interest::READABLE,
                        )?;
                        println!("client: sent message");
                        client_sent = true;
                    }
                    if event.is_readable() {
                        let mut buf = [0u8; 1024];
                        match client.read(&mut buf) {
                            Ok(0) => println!("client: server closed"),
                            Ok(n) => {
                                print!("client: echo -> ");
                                io::stdout().write_all(&buf[..n])?;
                            }
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
                            Err(e) => return Err(e),
                        }
                    }
                }

                // 服务端其它连接：读多少回多少
                token => {
                    let mut closed = false;
                    if let Some(conn) = connections.get_mut(&token) {
                        if event.is_readable() {
                            let mut buf = [0u8; 1024];
                            match conn.read(&mut buf) {
                                Ok(0) => closed = true,
                                Ok(n) => conn.write_all(&buf[..n])?, // 回声
                                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    if closed {
                        connections.remove(&token);
                    }
                }
            }
        }
    }
}