use std::net;

pub const BROADCAST_PORT: u16 = 56700;
pub const BROADCAST_IP: net::Ipv4Addr =
    net::Ipv4Addr::new(255, 255, 255, 255);
pub const BROADCAST_ADDRESS: net::SocketAddrV4 =
    net::SocketAddrV4::new(BROADCAST_IP, BROADCAST_PORT);
