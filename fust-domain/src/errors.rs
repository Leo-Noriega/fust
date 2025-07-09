//! Domain-specific error types

use thiserror::Error;

/// Domain error types
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },

    #[error("Entity not found: {entity_type} with id {id}")]
    EntityNotFound { entity_type: String, id: String },

    #[error("Business rule violation: {rule}")]
    BusinessRuleViolation { rule: String },

    #[error("Validation error: {field} - {message}")]
    ValidationError { field: String, message: String },

    #[error("Internal domain error: {message}")]
    Internal { message: String },
}

impl DomainError {
    /// Create a new invalid input error
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }

    /// Create a new entity not found error
    pub fn entity_not_found(entity_type: impl Into<String>, id: impl Into<String>) -> Self {
        Self::EntityNotFound {
            entity_type: entity_type.into(),
            id: id.into(),
        }
    }

    /// Create a new business rule violation error
    pub fn business_rule_violation(rule: impl Into<String>) -> Self {
        Self::BusinessRuleViolation {
            rule: rule.into(),
        }
    }

    /// Create a new validation error
    pub fn validation_error(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ValidationError {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a new internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }
} 