#![allow(unused_imports)]
pub use crate::error::Error;

pub use std::result::Result as StdResult;
pub type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = StdResult<T, DynError>;

pub use atoman::{Receiver, Sender, SharedItem, SharedMap, State};
pub use macron::*;

pub use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

pub use serde::{Deserialize, Serialize};
pub use serde_json::{self as json, Value as JsonValue};
