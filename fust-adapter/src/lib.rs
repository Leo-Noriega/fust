//! External adapters and implementations
//!
//! This crate contains implementations of the ports (interfaces) that
//! connect to external systems like file system, git, etc.

pub mod filesystem_adapter;
pub mod repositories;
pub mod services;

pub use filesystem_adapter::*;
pub use repositories::*;
pub use services::*;
