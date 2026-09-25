use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    loop {
        let mut buf = [0u8; 1500];
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("size = {}", amt);

        buf[..amt].reverse();              // 反转本次收到的有效字节
        socket.send_to(&buf[..amt], src)?; // 原路发回
    }
}