use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

fn main() {
    // 构造套接字地址
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    // 真正绑定端口，开始监听
    let listener = TcpListener::bind(addr).unwrap();
    println!("服务监听在 127.0.0.1:8080");

    // 等待客户端连接
    for stream in listener.incoming() {
        match stream {
            Ok(_conn) => println!("收到新连接"),
            Err(e) => eprintln!("连接失败: {}", e),
        }
    }
}
