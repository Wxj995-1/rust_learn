use ipnet::{Ipv4Net, Ipv6Net};
use std::net::{Ipv4Addr, Ipv6Addr};

fn main() -> std::io::Result<()> {
    // 构造
    let v4 = Ipv4Net::new(Ipv4Addr::new(10, 1, 1, 0), 24).expect("前缀长度非法");
    let v6 = Ipv6Net::new(Ipv6Addr::new(0xfd00, 0, 0, 0, 0, 0, 0, 0), 24)
        .expect("前缀长度非法");

    // 解析
    let v4: Ipv4Net = "10.1.1.0/24".parse().unwrap();
    let v6: Ipv6Net = "fd00::/24".parse().unwrap();

    println!("v4 = {}", v4);
    println!("v6 = {}", v6);
    Ok(())
}