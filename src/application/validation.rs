use crate::domain::errors::DomainError;
use crate::domain::value_objects::{Email, PhoneNumber};

/// Validation utilities for application layer
pub struct Validator;

impl Validator {
    /// Validate that a string is not empty or whitespace-only
    pub fn validate_non_empty_string(value: &str, field_name: &str) -> Result<(), DomainError> {
        if value.trim().is_empty() {
            return Err(DomainError::Validation(format!(
                "{} cannot be empty",
                field_name
            )));
        }
        Ok(())
    }

    /// Validate that a string meets minimum length requirements
    pub fn validate_min_length(value: &str, min_length: usize, field_name: &str) -> Result<(), DomainError> {
        if value.trim().len() < min_length {
            return Err(DomainError::Validation(format!(
                "{} must be at least {} characters long",
                field_name, min_length
            )));
        }
        Ok(())
    }

    /// Validate that a string doesn't exceed maximum length
    pub fn validate_max_length(value: &str, max_length: usize, field_name: &str) -> Result<(), DomainError> {
        if value.len() > max_length {
            return Err(DomainError::Validation(format!(
                "{} cannot exceed {} characters",
                field_name, max_length
            )));
        }
        Ok(())
    }

    /// Validate a collection of phone numbers
    pub fn validate_phone_numbers(phone_strings: &[String]) -> Result<Vec<PhoneNumber>, DomainError> {
        let mut phone_numbers = Vec::new();
        
        for phone_str in phone_strings {
            match PhoneNumber::try_from(phone_str.as_str()) {
                Ok(phone) => phone_numbers.push(phone),
                Err(e) => {
                    return Err(DomainError::Validation(format!(
                        "Invalid phone number '{}': {}",
                        phone_str, e
                    )));
                }
            }
        }
        
        Ok(phone_numbers)
    }

    /// Validate a collection of email addresses
    pub fn validate_emails(email_strings: &[String]) -> Result<Vec<Email>, DomainError> {
        let mut emails = Vec::new();
        
        for email_str in email_strings {
            match Email::try_from(email_str.as_str()) {
                Ok(email) => emails.push(email),
                Err(e) => {
                    return Err(DomainError::Validation(format!(
                        "Invalid email '{}': {}",
                        email_str, e
                    )));
                }
            }
        }
        
        Ok(emails)
    }

    /// Validate that at least one contact method is provided
    pub fn validate_contact_methods(phone_numbers: &[PhoneNumber], emails: &[Email]) -> Result<(), DomainError> {
        if phone_numbers.is_empty() && emails.is_empty() {
            return Err(DomainError::BusinessRule(
                "At least one phone number or email address is required".to_string(),
            ));
        }
        Ok(())
    }

    /// Validate name components
    pub fn validate_name_component(name: &str, component_name: &str) -> Result<(), DomainError> {
        Self::validate_non_empty_string(name, component_name)?;
        Self::validate_min_length(name, 1, component_name)?;
        Self::validate_max_length(name, 100, component_name)?;
        
        // Check for invalid characters (basic validation)
        if name.chars().any(|c| c.is_control() && c != '\t') {
            return Err(DomainError::Validation(format!(
                "{} contains invalid characters",
                component_name
            )));
        }
        
        Ok(())
    }

    /// Validate search query
    pub fn validate_search_query(query: &str) -> Result<(), DomainError> {
        Self::validate_non_empty_string(query, "Search query")?;
        Self::validate_min_length(query, 1, "Search query")?;
        Self::validate_max_length(query, 200, "Search query")?;
        Ok(())
    }

    /// Validate pagination parameters
    pub fn validate_pagination(_page: usize, page_size: usize) -> Result<(), DomainError> {
        if page_size == 0 {
            return Err(DomainError::Validation(
                "Page size must be greater than 0".to_string(),
            ));
        }
        
        if page_size > 100 {
            return Err(DomainError::Validation(
                "Page size cannot exceed 100".to_string(),
            ));
        }
        
        Ok(())
    }
}
