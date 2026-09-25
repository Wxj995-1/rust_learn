use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(Serialize, Deserialize)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

fn handle_client(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    println!("incoming connection from: {}", stream.peer_addr()?);

    let mut stream = BufReader::new(stream);
    let mut data = Vec::new();

    loop {
        data.clear();
        let bytes_read = stream.read_until(b'\n', &mut data)?;
        if bytes_read == 0 {
            return Ok(());
        }

        let input: Point3D = serde_json::from_slice(&data)?;
        let value = input.x.pow(2) + input.y.pow(2) + input.z.pow(2);

        let out = serde_json::to_vec(&f64::from(value).sqrt())?;
        stream.get_mut().write_all(&out)?;
        stream.get_mut().write_all(b"\n")?;
        stream.get_mut().flush()?;
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    println!("listening on 0.0.0.0:8080");

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("accept error = {e}"),
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("error = {e}");
                    }
                });
            }
        }
    }

    unreachable!("incoming() 永不结束")
}