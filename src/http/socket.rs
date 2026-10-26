use serde::{Deserialize, Serialize};
use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    path::{Path, PathBuf},
};

/// Server address / socket configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Socket {
    /// TCP protocol (standard HTTP/IP endpoint).
    Ip(SocketAddr),

    /// Cross-platform IPC path:
    /// - Unix: Unix Domain Socket (UDS) via filesystem path.
    /// - Windows: Named Pipes (NP).
    Path(PathBuf),
}

impl From<([u8; 4], u16)> for Socket {
    fn from(value: ([u8; 4], u16)) -> Self {
        Self::Ip(value.into())
    }
}

impl From<([u16; 8], u16)> for Socket {
    fn from(value: ([u16; 8], u16)) -> Self {
        Self::Ip(value.into())
    }
}

impl From<([u8; 16], u16)> for Socket {
    fn from(value: ([u8; 16], u16)) -> Self {
        Self::Ip(value.into())
    }
}

impl From<u16> for Socket {
    fn from(value: u16) -> Self {
        Self::Ip(([127, 0, 0, 1], value).into())
    }
}

impl From<SocketAddr> for Socket {
    fn from(addr: SocketAddr) -> Self {
        Self::Ip(addr)
    }
}

impl From<Ipv4Addr> for Socket {
    fn from(ip: Ipv4Addr) -> Self {
        Self::Ip(SocketAddr::new(IpAddr::V4(ip), 0))
    }
}

impl From<Ipv6Addr> for Socket {
    fn from(ip: Ipv6Addr) -> Self {
        Self::Ip(SocketAddr::new(IpAddr::V6(ip), 0))
    }
}

impl From<IpAddr> for Socket {
    fn from(ip: IpAddr) -> Self {
        Self::Ip(SocketAddr::new(ip, 0))
    }
}

impl From<&str> for Socket {
    fn from(path: &str) -> Self {
        Self::Path(path.into())
    }
}

impl From<String> for Socket {
    fn from(path: String) -> Self {
        Self::Path(path.into())
    }
}

impl From<&Path> for Socket {
    fn from(path: &Path) -> Self {
        Self::Path(path.to_path_buf())
    }
}

impl From<PathBuf> for Socket {
    fn from(path: PathBuf) -> Self {
        Self::Path(path)
    }
}

/// IPC socket guard.
#[cfg(feature = "server")]
pub(crate) struct SocketGuard {
    pub path: std::path::PathBuf,
}

#[cfg(feature = "server")]
impl Drop for SocketGuard {
    fn drop(&mut self) {
        if self.path.exists() {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}
