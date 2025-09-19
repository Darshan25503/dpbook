use crate::domain::entities::Contact;
use crate::domain::repositories::{ContactRepositorySync, RepositoryError};
use crate::domain::value_objects::{ContactId, Email, PhoneNumber};
use std::sync::Arc;

/// Use case for updating an existing contact
/// Follows Single Responsibility Principle - only handles contact updates
pub struct UpdateContactUseCase {
    repository: Arc<dyn ContactRepositorySync>,
}

impl UpdateContactUseCase {
    pub fn new(repository: Arc<dyn ContactRepositorySync>) -> Self {
        Self { repository }
    }

    /// Execute the update contact use case
    pub fn execute(&self, request: UpdateContactRequest) -> Result<UpdateContactResponse, RepositoryError> {
        // Check if contact exists
        let mut contact = self.repository
            .find_by_id(&request.contact_id)?
            .ok_or_else(|| RepositoryError::ContactNotFound(request.contact_id.to_string()))?;

        // Update fields if provided
        if let Some(first_name) = request.first_name {
            if first_name.trim().is_empty() {
                return Err(RepositoryError::ValidationError(
                    "First name cannot be empty".to_string(),
                ));
            }
            contact.set_first_name(first_name);
        }

        if let Some(last_name) = request.last_name {
            if last_name.trim().is_empty() {
                return Err(RepositoryError::ValidationError(
                    "Last name cannot be empty".to_string(),
                ));
            }
            contact.set_last_name(last_name);
        }

        if let Some(notes) = request.notes {
            contact.set_notes(if notes.trim().is_empty() { None } else { Some(notes) });
        }

        // Handle phone number updates
        for phone in request.add_phone_numbers {
            contact.add_phone_number(phone);
        }

        for phone in request.remove_phone_numbers {
            contact.remove_phone_number(&phone);
        }

        // Handle email updates
        for email in request.add_emails {
            contact.add_email(email);
        }

        for email in request.remove_emails {
            contact.remove_email(&email);
        }

        // Handle tag updates
        for tag in request.add_tags {
            contact.add_tag(tag);
        }

        for tag in request.remove_tags {
            contact.remove_tag(&tag);
        }

        // Validate that contact still has at least one phone or email
        if contact.phone_numbers().is_empty() && contact.emails().is_empty() {
            return Err(RepositoryError::ValidationError(
                "Contact must have at least one phone number or email".to_string(),
            ));
        }

        // Save updated contact
        self.repository.update(contact.clone())?;

        Ok(UpdateContactResponse {
            contact,
            message: "Contact updated successfully".to_string(),
        })
    }
}

/// Request DTO for updating a contact
#[derive(Debug, Default)]
pub struct UpdateContactRequest {
    pub contact_id: ContactId,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub notes: Option<String>,
    pub add_phone_numbers: Vec<PhoneNumber>,
    pub remove_phone_numbers: Vec<PhoneNumber>,
    pub add_emails: Vec<Email>,
    pub remove_emails: Vec<Email>,
    pub add_tags: Vec<String>,
    pub remove_tags: Vec<String>,
}

/// Response DTO for updating a contact
#[derive(Debug)]
pub struct UpdateContactResponse {
    pub contact: Contact,
    pub message: String,
}
