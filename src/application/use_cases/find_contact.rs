use crate::domain::entities::Contact;
use crate::domain::repositories::{ContactRepositorySync, RepositoryError};
use crate::domain::value_objects::ContactId;
use std::sync::Arc;

/// Use case for finding a contact by ID
/// Follows Single Responsibility Principle - only handles contact retrieval
pub struct FindContactUseCase {
    repository: Arc<dyn ContactRepositorySync>,
}

impl FindContactUseCase {
    pub fn new(repository: Arc<dyn ContactRepositorySync>) -> Self {
        Self { repository }
    }

    /// Execute the find contact use case
    pub fn execute(&self, request: FindContactRequest) -> Result<FindContactResponse, RepositoryError> {
        let contact = self.repository.find_by_id(&request.contact_id)?;

        match contact {
            Some(contact) => Ok(FindContactResponse {
                contact: Some(contact),
                found: true,
            }),
            None => Ok(FindContactResponse {
                contact: None,
                found: false,
            }),
        }
    }
}

/// Request DTO for finding a contact
#[derive(Debug)]
pub struct FindContactRequest {
    pub contact_id: ContactId,
}

/// Response DTO for finding a contact
#[derive(Debug)]
pub struct FindContactResponse {
    pub contact: Option<Contact>,
    pub found: bool,
}
