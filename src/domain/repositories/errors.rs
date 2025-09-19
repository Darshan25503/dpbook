use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Contact not found with ID: {0}")]
    ContactNotFound(String),
    
    #[error("Contact already exists with ID: {0}")]
    ContactAlreadyExists(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}
