use crate::domain::entities::Contact;
use crate::domain::repositories::RepositoryError;
use crate::domain::value_objects::ContactId;
use std::future::Future;
use std::pin::Pin;

/// Repository trait for contact persistence operations
/// Follows the Repository pattern and Dependency Inversion Principle
pub trait ContactRepository: Send + Sync {
    /// Save a new contact
    fn save(&self, contact: Contact) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + '_>>;
    
    /// Find a contact by ID
    fn find_by_id(&self, id: &ContactId) -> Pin<Box<dyn Future<Output = Result<Option<Contact>, RepositoryError>> + Send + '_>>;
    
    /// Find all contacts
    fn find_all(&self) -> Pin<Box<dyn Future<Output = Result<Vec<Contact>, RepositoryError>> + Send + '_>>;
    
    /// Update an existing contact
    fn update(&self, contact: Contact) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + '_>>;
    
    /// Delete a contact by ID
    fn delete(&self, id: &ContactId) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + '_>>;
    
    /// Search contacts by query
    fn search(&self, query: &str) -> Pin<Box<dyn Future<Output = Result<Vec<Contact>, RepositoryError>> + Send + '_>>;
    
    /// Check if a contact exists
    fn exists(&self, id: &ContactId) -> Pin<Box<dyn Future<Output = Result<bool, RepositoryError>> + Send + '_>>;
    
    /// Get total count of contacts
    fn count(&self) -> Pin<Box<dyn Future<Output = Result<usize, RepositoryError>> + Send + '_>>;
}

// For synchronous implementations, we'll also provide a sync version
pub trait ContactRepositorySync: Send + Sync {
    /// Save a new contact
    fn save(&self, contact: Contact) -> Result<(), RepositoryError>;
    
    /// Find a contact by ID
    fn find_by_id(&self, id: &ContactId) -> Result<Option<Contact>, RepositoryError>;
    
    /// Find all contacts
    fn find_all(&self) -> Result<Vec<Contact>, RepositoryError>;
    
    /// Update an existing contact
    fn update(&self, contact: Contact) -> Result<(), RepositoryError>;
    
    /// Delete a contact by ID
    fn delete(&self, id: &ContactId) -> Result<(), RepositoryError>;
    
    /// Search contacts by query
    fn search(&self, query: &str) -> Result<Vec<Contact>, RepositoryError>;
    
    /// Check if a contact exists
    fn exists(&self, id: &ContactId) -> Result<bool, RepositoryError>;
    
    /// Get total count of contacts
    fn count(&self) -> Result<usize, RepositoryError>;
}
