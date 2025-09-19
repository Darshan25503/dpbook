use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PhoneNumberError {
    #[error("Invalid phone number format: {0}")]
    InvalidFormat(String),
    #[error("Phone number cannot be empty")]
    Empty,
}

/// Value object representing a phone number
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhoneNumber {
    value: String,
}

impl PhoneNumber {
    /// Create a new phone number with validation
    pub fn new(value: String) -> Result<Self, PhoneNumberError> {
        if value.trim().is_empty() {
            return Err(PhoneNumberError::Empty);
        }

        let cleaned = Self::clean_phone_number(&value);
        if !Self::is_valid_format(&cleaned) {
            return Err(PhoneNumberError::InvalidFormat(value));
        }

        Ok(Self { value: cleaned })
    }

    /// Get the phone number value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Clean phone number by removing non-digit characters except +
    fn clean_phone_number(phone: &str) -> String {
        phone
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '+')
            .collect()
    }

    /// Validate phone number format
    fn is_valid_format(phone: &str) -> bool {
        let regex = Regex::new(r"^(\+\d{1,3})?\d{10,15}$").unwrap();
        regex.is_match(phone)
    }

    /// Format phone number for display
    pub fn formatted(&self) -> String {
        if self.value.starts_with('+') {
            self.value.clone()
        } else if self.value.len() == 10 {
            format!(
                "({}) {}-{}",
                &self.value[0..3],
                &self.value[3..6],
                &self.value[6..10]
            )
        } else {
            self.value.clone()
        }
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatted())
    }
}

impl TryFrom<String> for PhoneNumber {
    type Error = PhoneNumberError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        PhoneNumber::new(value)
    }
}

impl TryFrom<&str> for PhoneNumber {
    type Error = PhoneNumberError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        PhoneNumber::new(value.to_string())
    }
}


