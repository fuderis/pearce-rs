use crate::prelude::*;
use std::path::Path;

/// The IPC listener
pub struct IpcListener {
    pub inner: tokio::net::UnixListener,
}

impl IpcListener {
    pub fn bind(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        if path.exists() {
            let _ = std::fs::remove_file(path);
        }

        let inner = tokio::net::UnixListener::bind(path)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = std::fs::Permissions::from_mode(0o660);
            let _ = std::fs::set_permissions(path, permissions);
        }

        Ok(Self { inner })
    }

    pub async fn accept(&mut self) -> Result<tokio::net::UnixStream> {
        let (stream, _) = self.inner.accept().await?;
        Ok(stream)
    }
}
