use crate::domain::entities::Contact;
use crate::domain::repositories::{ContactRepositorySync, RepositoryError};
use std::sync::Arc;

/// Use case for searching contacts
/// Follows Single Responsibility Principle - only handles contact searching
pub struct SearchContactsUseCase {
    repository: Arc<dyn ContactRepositorySync>,
}

impl SearchContactsUseCase {
    pub fn new(repository: Arc<dyn ContactRepositorySync>) -> Self {
        Self { repository }
    }

    /// Execute the search contacts use case
    pub fn execute(&self, request: SearchContactsRequest) -> Result<SearchContactsResponse, RepositoryError> {
        if request.query.trim().is_empty() {
            return Err(RepositoryError::ValidationError(
                "Search query cannot be empty".to_string(),
            ));
        }

        let contacts = self.repository.search(&request.query)?;
        let count = contacts.len();

        Ok(SearchContactsResponse {
            contacts,
            query: request.query,
            count,
        })
    }
}

/// Request DTO for searching contacts
#[derive(Debug)]
pub struct SearchContactsRequest {
    pub query: String,
}

/// Response DTO for searching contacts
#[derive(Debug)]
pub struct SearchContactsResponse {
    pub contacts: Vec<Contact>,
    pub query: String,
    pub count: usize,
}
