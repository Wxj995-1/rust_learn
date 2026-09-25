use std::io;
use std::net::UdpSocket;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8000")?; // 本地端口 8000
    socket.connect("127.0.0.1:8080")?;                // 设置默认对端（服务器）

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        socket.send(input.as_bytes())?;               // 发送给服务器

        let mut buf = [0u8; 1500];
        let amt = socket.recv(&mut buf)?;             // 收响应，返回字节数
        println!("recv = {}", String::from_utf8_lossy(&buf[..amt]));
    }
}