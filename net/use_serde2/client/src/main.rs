use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

#[derive(Serialize, Deserialize)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    loop {
        // 读一行输入，格式如：3,4,0
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        if line.trim().is_empty() {
            continue; // 空行跳过
        }

        let parts: Vec<&str> = line.trim().split(',').collect();
        let point = Point3D {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        };

        // 序列化并发送（带换行）
        let json = serde_json::to_string(&point)?;
        stream.write_all(json.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;

        // 读取服务器返回的一行
        let mut buffer = Vec::new();
        BufReader::new(&stream).read_until(b'\n', &mut buffer)?;

        let response = String::from_utf8_lossy(&buffer);
        if response.trim().is_empty() {
            eprintln!("empty response");
            continue;
        }
        println!("response = {}", response.trim());
    }
}