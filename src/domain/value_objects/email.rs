use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Invalid email format: {0}")]
    InvalidFormat(String),
    #[error("Email cannot be empty")]
    Empty,
}

/// Value object representing an email address
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email {
    value: String,
}

impl Email {
    /// Create a new email with validation
    pub fn new(value: String) -> Result<Self, EmailError> {
        let trimmed = value.trim().to_lowercase();
        
        if trimmed.is_empty() {
            return Err(EmailError::Empty);
        }

        if !Self::is_valid_format(&trimmed) {
            return Err(EmailError::InvalidFormat(value));
        }

        Ok(Self { value: trimmed })
    }

    /// Get the email value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Validate email format using regex
    fn is_valid_format(email: &str) -> bool {
        let regex = Regex::new(
            r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
        ).unwrap();
        regex.is_match(email)
    }

    /// Get the domain part of the email
    pub fn domain(&self) -> &str {
        self.value.split('@').nth(1).unwrap_or("")
    }

    /// Get the local part of the email
    pub fn local_part(&self) -> &str {
        self.value.split('@').next().unwrap_or("")
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryFrom<String> for Email {
    type Error = EmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Email::new(value)
    }
}

impl TryFrom<&str> for Email {
    type Error = EmailError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Email::new(value.to_string())
    }
}


