use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("已连接服务器，输入内容回车发送（Ctrl-D 退出）");

    loop {
        print!("> ");
        io::stdout().flush()?;              // 让提示立刻显示

        let mut input = String::new();
        let n = io::stdin().read_line(&mut input)?;
        if n == 0 {
            break;                          // Ctrl-D：输入结束
        }
        if input.trim().is_empty() {
            continue;                       // 空行跳过
        }

        stream.write_all(input.as_bytes())?;      // 发送给服务器

        let mut buf = [0u8; 1024];
        let len = stream.read(&mut buf)?;         // 读取回显（读一次）
        if len == 0 {
            println!("服务器已关闭连接");
            break;
        }
        print!("echo: {}", String::from_utf8_lossy(&buf[..len]));
    }

    Ok(())
}