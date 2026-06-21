#![allow(unused_imports)]
pub use std::result::Result as StdResult;
pub type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = StdResult<T, DynError>;

pub(crate) use atoman::*;
pub(crate) use macron::*;

pub(crate) use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub(crate) use serde::{Deserialize, Serialize};
