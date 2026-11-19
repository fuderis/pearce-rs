#![allow(unused_imports, dead_code)]
pub use crate::error::Error;

pub type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub use atoman::{Receiver, Sender, SharedItem, SharedMap, State};
pub use macron::*;

pub use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

pub use serde::{Deserialize, Serialize};
pub use serde_json::{self as json, Value as JsonValue};
