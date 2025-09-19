use crate::domain::entities::Contact;
use crate::domain::repositories::{ContactRepositorySync, RepositoryError};
use std::sync::Arc;

/// Use case for listing all contacts
/// Follows Single Responsibility Principle - only handles contact listing
pub struct ListContactsUseCase {
    repository: Arc<dyn ContactRepositorySync>,
}

impl ListContactsUseCase {
    pub fn new(repository: Arc<dyn ContactRepositorySync>) -> Self {
        Self { repository }
    }

    /// Execute the list contacts use case
    pub fn execute(&self, request: ListContactsRequest) -> Result<ListContactsResponse, RepositoryError> {
        let mut contacts = self.repository.find_all()?;

        // Apply sorting
        match request.sort_by {
            SortBy::FirstName => contacts.sort_by(|a, b| a.first_name().cmp(b.first_name())),
            SortBy::LastName => contacts.sort_by(|a, b| a.last_name().cmp(b.last_name())),
            SortBy::FullName => contacts.sort_by(|a, b| a.full_name().cmp(&b.full_name())),
        }

        if request.reverse {
            contacts.reverse();
        }

        // Apply pagination
        let total_count = contacts.len();
        let start_index = request.page * request.page_size;
        let end_index = std::cmp::min(start_index + request.page_size, contacts.len());

        let paginated_contacts = if start_index < contacts.len() {
            contacts[start_index..end_index].to_vec()
        } else {
            Vec::new()
        };

        Ok(ListContactsResponse {
            contacts: paginated_contacts,
            total_count,
            page: request.page,
            page_size: request.page_size,
            has_more: end_index < total_count,
        })
    }
}

/// Request DTO for listing contacts
#[derive(Debug)]
pub struct ListContactsRequest {
    pub page: usize,
    pub page_size: usize,
    pub sort_by: SortBy,
    pub reverse: bool,
}

impl Default for ListContactsRequest {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: 10,
            sort_by: SortBy::LastName,
            reverse: false,
        }
    }
}

/// Sorting options for contacts
#[derive(Debug, Clone)]
pub enum SortBy {
    FirstName,
    LastName,
    FullName,
}

/// Response DTO for listing contacts
#[derive(Debug)]
pub struct ListContactsResponse {
    pub contacts: Vec<Contact>,
    pub total_count: usize,
    pub page: usize,
    pub page_size: usize,
    pub has_more: bool,
}
