use std::env;

use pnet::datalink::{self, Channel, Config};
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;

fn main() {
    // 命令行第 1 个参数：网卡名，例如 cargo run -- eth0
    let interface_name = env::args().nth(1).expect("用法: cargo run -- <网卡名>");

    // 列举本机网卡，按名字找到目标网卡
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name)
        .expect("找不到指定网卡，请用 `ip link` 查看名称");

    // 打开数据链路层通道
    let config = Config::default();
    let (_tx, mut rx) = match datalink::channel(&interface, config) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => {
            eprintln!("不支持的通道类型");
            return;
        }
        Err(e) => {
            eprintln!("无法打开通道: {e}（Linux 上通常需要 sudo）");
            return;
        }
    };

    println!("正在监听 {} ...（Ctrl-C 退出）", interface.name);

    loop {
        match rx.next() {
            Ok(frame) => {
                // 解析以太网帧
                let ethernet = match EthernetPacket::new(frame) {
                    Some(e) => e,
                    None => continue,
                };
                if ethernet.get_ethertype() != EtherTypes::Ipv4 {
                    continue; // 只看 IPv4
                }

                // 解析 IPv4
                let ip = match Ipv4Packet::new(ethernet.payload()) {
                    Some(ip) => ip,
                    None => continue,
                };
                if ip.get_next_level_protocol() != IpNextHeaderProtocols::Tcp {
                    continue; // 只看 TCP
                }

                // 解析 TCP
                if let Some(tcp) = TcpPacket::new(ip.payload()) {
                    println!(
                        "TCP {}:{} -> {}:{}",
                        ip.get_source(),
                        tcp.get_source(),
                        ip.get_destination(),
                        tcp.get_destination()
                    );
                }
            }
            Err(e) => eprintln!("接收错误: {e}"),
        }
    }
}