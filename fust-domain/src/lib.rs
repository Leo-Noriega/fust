//! Core domain logic and business entities
//! 
//! This crate contains the heart of the business logic, including:
//! - Domain entities and value objects
//! - Business rules and validation
//! - Domain services
//! - Error types

pub mod entities;
pub mod errors;
pub mod metadata;
pub mod services;
pub mod value_objects;

pub use entities::*;
pub use errors::*;
pub use metadata::*;
pub use services::*;
pub use value_objects::*; 