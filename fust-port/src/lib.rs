//! Port interfaces and abstractions
//! 
//! This crate defines the interfaces (ports) that the application layer
//! uses to communicate with the infrastructure layer.

pub mod repositories;
pub mod services;

pub use repositories::*;
pub use services::*; 