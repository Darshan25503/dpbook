use crate::application::use_cases::*;
use crate::domain::repositories::ContactRepositorySync;
use std::sync::Arc;

/// Application service that orchestrates use cases
/// Follows the Facade pattern to provide a simplified interface
pub struct ContactService {
    add_contact_use_case: AddContactUseCase,
    find_contact_use_case: FindContactUseCase,
    update_contact_use_case: UpdateContactUseCase,
    delete_contact_use_case: DeleteContactUseCase,
    list_contacts_use_case: ListContactsUseCase,
    search_contacts_use_case: SearchContactsUseCase,
}

impl ContactService {
    pub fn new(repository: Arc<dyn ContactRepositorySync>) -> Self {
        Self {
            add_contact_use_case: AddContactUseCase::new(repository.clone()),
            find_contact_use_case: FindContactUseCase::new(repository.clone()),
            update_contact_use_case: UpdateContactUseCase::new(repository.clone()),
            delete_contact_use_case: DeleteContactUseCase::new(repository.clone()),
            list_contacts_use_case: ListContactsUseCase::new(repository.clone()),
            search_contacts_use_case: SearchContactsUseCase::new(repository),
        }
    }

    pub fn add_contact(&self, request: AddContactRequest) -> Result<AddContactResponse, crate::domain::repositories::RepositoryError> {
        self.add_contact_use_case.execute(request)
    }

    pub fn find_contact(&self, request: FindContactRequest) -> Result<FindContactResponse, crate::domain::repositories::RepositoryError> {
        self.find_contact_use_case.execute(request)
    }

    pub fn update_contact(&self, request: UpdateContactRequest) -> Result<UpdateContactResponse, crate::domain::repositories::RepositoryError> {
        self.update_contact_use_case.execute(request)
    }

    pub fn delete_contact(&self, request: DeleteContactRequest) -> Result<DeleteContactResponse, crate::domain::repositories::RepositoryError> {
        self.delete_contact_use_case.execute(request)
    }

    pub fn list_contacts(&self, request: ListContactsRequest) -> Result<ListContactsResponse, crate::domain::repositories::RepositoryError> {
        self.list_contacts_use_case.execute(request)
    }

    pub fn search_contacts(&self, request: SearchContactsRequest) -> Result<SearchContactsResponse, crate::domain::repositories::RepositoryError> {
        self.search_contacts_use_case.execute(request)
    }
}
