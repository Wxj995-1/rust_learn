use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};


fn main() {
    let v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    assert_eq!("127.0.0.1".parse(),Ok(v4));
    assert_eq!("::1".parse(),Ok(v6));
    assert_eq!(v4.is_loopback(),true);
    assert_eq!(v6.is_loopback(),true);

    println!("Hello, world!");
}
