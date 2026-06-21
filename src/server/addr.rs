use crate::prelude::*;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

/// The server endpoint wrapper
#[derive(Debug)]
pub enum Addr {
    Ip(SocketAddr),
    Path(PathBuf),
}

impl From<([u8; 4], u16)> for Addr {
    fn from(value: ([u8; 4], u16)) -> Self {
        Self::Ip(value.into())
    }
}

impl From<([u16; 8], u16)> for Addr {
    fn from(value: ([u16; 8], u16)) -> Self {
        Self::Ip(value.into())
    }
}

impl From<([u8; 16], u16)> for Addr {
    fn from(value: ([u8; 16], u16)) -> Self {
        Self::Ip(value.into())
    }
}
impl From<u16> for Addr {
    fn from(value: u16) -> Self {
        Self::Ip(([127, 0, 0, 1], value).into())
    }
}

impl From<SocketAddr> for Addr {
    fn from(addr: SocketAddr) -> Self {
        Self::Ip(addr)
    }
}

impl From<Ipv4Addr> for Addr {
    fn from(ip: Ipv4Addr) -> Self {
        Self::Ip(SocketAddr::new(IpAddr::V4(ip), 0))
    }
}

impl From<Ipv6Addr> for Addr {
    fn from(ip: Ipv6Addr) -> Self {
        Self::Ip(SocketAddr::new(IpAddr::V6(ip), 0))
    }
}

impl From<IpAddr> for Addr {
    fn from(ip: IpAddr) -> Self {
        Self::Ip(SocketAddr::new(ip, 0))
    }
}

impl From<&str> for Addr {
    fn from(path: &str) -> Self {
        Self::Path(PathBuf::from(path))
    }
}

impl From<String> for Addr {
    fn from(path: String) -> Self {
        Self::Path(PathBuf::from(path))
    }
}

impl From<&Path> for Addr {
    fn from(path: &Path) -> Self {
        Self::Path(path.to_path_buf())
    }
}

impl From<PathBuf> for Addr {
    fn from(path: PathBuf) -> Self {
        Self::Path(path)
    }
}

impl From<&PathBuf> for Addr {
    fn from(path: &PathBuf) -> Self {
        Self::Path(path.clone())
    }
}
