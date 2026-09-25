use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 1024];
    loop {
        let n = stream.read(&mut buf)?;      // 读对端发来的字节
        if n == 0 {
            return Ok(());                    // 读到 0 = 对端关闭连接
        }
        stream.write_all(&buf[..n])?;         // 回声：原样发回
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("listening on {}", listener.local_addr()?);   // 打印监听地址

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {       // 每个连接一个线程（分离，不收集句柄）
                    if let Err(e) = handle_client(stream) {
                        eprintln!("client error: {e}");
                    }
                });
            }
            Err(e) => eprintln!("accept failed: {e}"),
        }
    }

    unreachable!("incoming() never returns")
}