#![deny(missing_docs)]
//! Sentinel

///
pub mod save_data;
/// File scanner for searching app files
pub mod scanner;
pub use anyhow::anyhow as err_msg;
pub use anyhow::Result;
