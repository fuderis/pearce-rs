use std::net::{IpAddr as StdIpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

/// The server IP address wrapper
#[derive(Debug)]
pub struct IpAddr(pub SocketAddr);

impl From<([u8; 4], u16)> for IpAddr {
    fn from(value: ([u8; 4], u16)) -> Self {
        Self(value.into())
    }
}

impl From<([u16; 8], u16)> for IpAddr {
    fn from(value: ([u16; 8], u16)) -> Self {
        Self(value.into())
    }
}

impl From<([u8; 16], u16)> for IpAddr {
    fn from(value: ([u8; 16], u16)) -> Self {
        Self(value.into())
    }
}
impl From<u16> for IpAddr {
    fn from(value: u16) -> Self {
        Self(([127, 0, 0, 1], value).into())
    }
}

impl From<SocketAddr> for IpAddr {
    fn from(addr: SocketAddr) -> Self {
        Self(addr)
    }
}

impl From<Ipv4Addr> for IpAddr {
    fn from(ip: Ipv4Addr) -> Self {
        Self(SocketAddr::new(StdIpAddr::V4(ip), 0))
    }
}

impl From<Ipv6Addr> for IpAddr {
    fn from(ip: Ipv6Addr) -> Self {
        Self(SocketAddr::new(StdIpAddr::V6(ip), 0))
    }
}

impl From<StdIpAddr> for IpAddr {
    fn from(ip: StdIpAddr) -> Self {
        Self(SocketAddr::new(ip, 0))
    }
}
