use crate::application::validation::Validator;
use crate::domain::entities::Contact;
use crate::domain::repositories::{ContactRepositorySync, RepositoryError};
use crate::domain::value_objects::{Email, PhoneNumber};
use std::sync::Arc;

/// Use case for adding a new contact to the phonebook
/// Follows Single Responsibility Principle - only handles contact creation
pub struct AddContactUseCase {
    repository: Arc<dyn ContactRepositorySync>,
}

impl AddContactUseCase {
    pub fn new(repository: Arc<dyn ContactRepositorySync>) -> Self {
        Self { repository }
    }

    /// Execute the add contact use case
    pub fn execute(&self, request: AddContactRequest) -> Result<AddContactResponse, RepositoryError> {
        // Validate input using domain validation
        Validator::validate_name_component(&request.first_name, "First name")
            .map_err(|e| RepositoryError::ValidationError(e.to_string()))?;

        Validator::validate_name_component(&request.last_name, "Last name")
            .map_err(|e| RepositoryError::ValidationError(e.to_string()))?;

        Validator::validate_contact_methods(&request.phone_numbers, &request.emails)
            .map_err(|e| RepositoryError::ValidationError(e.to_string()))?;

        // Create contact entity
        let contact = Contact::new(
            request.first_name,
            request.last_name,
            request.phone_numbers,
            request.emails,
        );

        // Save to repository
        self.repository.save(contact.clone())?;

        Ok(AddContactResponse {
            contact_id: contact.id().clone(),
            message: "Contact added successfully".to_string(),
        })
    }
}

/// Request DTO for adding a contact
#[derive(Debug)]
pub struct AddContactRequest {
    pub first_name: String,
    pub last_name: String,
    pub phone_numbers: Vec<PhoneNumber>,
    pub emails: Vec<Email>,
}

/// Response DTO for adding a contact
#[derive(Debug)]
pub struct AddContactResponse {
    pub contact_id: crate::domain::value_objects::ContactId,
    pub message: String,
}
