use crate::prelude::*;

use atoman::net;
use std::path::Path;

pub struct IpcListener {
    #[cfg(unix)]
    pub inner: atoman::net::UnixListener,

    #[cfg(windows)]
    pub inner: tokio::net::windows::named_pipe::NamedPipeServer,
}

impl IpcListener {
    /// Binds IPC listener (`Unix Domain Sockets` on Unix, `Named Pipes` on Windows).
    pub fn bind(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();

        #[cfg(unix)]
        {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            if path.exists() {
                let _ = std::fs::remove_file(path);
            }

            let inner = net::UnixListener::bind(path)?;

            use std::os::unix::fs::PermissionsExt;
            let permissions = std::fs::Permissions::from_mode(0o660);
            let _ = std::fs::set_permissions(path, permissions);

            Ok(Self { inner })
        }

        #[cfg(windows)]
        {
            let path_str = path.to_str().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid pipe path")
            })?;

            // let’s sure the path starts with \\.\pipe\
            let pipe_name = if path_str.starts_with(r"\\.\pipe\") {
                path_str.to_string()
            } else {
                format!(
                    r"\\.\pipe\{}",
                    path_str.trim_start_matches(|c| c == '/' || c == '\\')
                )
            };

            let inner = net::windows::named_pipe::ServerOptions::new()
                .first_pipe_instance(true)
                .create(&pipe_name)?;

            Ok(Self { inner })
        }
    }
}
