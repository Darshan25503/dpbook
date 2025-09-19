use crate::domain::repositories::{ContactRepositorySync, RepositoryError};
use crate::domain::value_objects::ContactId;
use std::sync::Arc;

/// Use case for deleting a contact
/// Follows Single Responsibility Principle - only handles contact deletion
pub struct DeleteContactUseCase {
    repository: Arc<dyn ContactRepositorySync>,
}

impl DeleteContactUseCase {
    pub fn new(repository: Arc<dyn ContactRepositorySync>) -> Self {
        Self { repository }
    }

    /// Execute the delete contact use case
    pub fn execute(&self, request: DeleteContactRequest) -> Result<DeleteContactResponse, RepositoryError> {
        // Check if contact exists
        if !self.repository.exists(&request.contact_id)? {
            return Err(RepositoryError::ContactNotFound(request.contact_id.to_string()));
        }

        // Delete the contact
        self.repository.delete(&request.contact_id)?;

        Ok(DeleteContactResponse {
            contact_id: request.contact_id,
            message: "Contact deleted successfully".to_string(),
        })
    }
}

/// Request DTO for deleting a contact
#[derive(Debug)]
pub struct DeleteContactRequest {
    pub contact_id: ContactId,
}

/// Response DTO for deleting a contact
#[derive(Debug)]
pub struct DeleteContactResponse {
    pub contact_id: ContactId,
    pub message: String,
}
