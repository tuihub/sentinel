#![deny(missing_docs)]
//! Sentinel

///
pub mod save_data;
/// File scanner for searching app files
pub mod scanner;
// Used by binaries. Not public API.
#[doc(hidden)]
#[path = "private/mod.rs"]
pub mod __private;

pub use anyhow::{anyhow as err_msg, Result};
