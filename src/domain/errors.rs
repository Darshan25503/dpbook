use thiserror::Error;

/// Domain-level errors
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Business rule violation: {0}")]
    BusinessRule(String),
    
    #[error("Entity not found: {0}")]
    EntityNotFound(String),
    
    #[error("Duplicate entity: {0}")]
    DuplicateEntity(String),
}

/// Application-level errors
#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    
    #[error("Repository error: {0}")]
    Repository(#[from] crate::domain::repositories::RepositoryError),
    
    #[error("Use case error: {0}")]
    UseCase(String),
    
    #[error("Service error: {0}")]
    Service(String),
}

/// Infrastructure-level errors
#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("File system error: {0}")]
    FileSystem(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
}

/// Presentation-level errors
#[derive(Error, Debug)]
pub enum PresentationError {
    #[error("CLI error: {0}")]
    Cli(String),
    
    #[error("Input validation error: {0}")]
    InputValidation(String),
    
    #[error("Output formatting error: {0}")]
    OutputFormatting(String),
    
    #[error("Application error: {0}")]
    Application(#[from] ApplicationError),
}

/// Top-level application error that can be returned from main
#[derive(Error, Debug)]
pub enum PhonebookError {
    #[error("Presentation error: {0}")]
    Presentation(#[from] PresentationError),
    
    #[error("Application error: {0}")]
    Application(#[from] ApplicationError),
    
    #[error("Infrastructure error: {0}")]
    Infrastructure(#[from] InfrastructureError),
    
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    
    #[error("System error: {0}")]
    System(String),
}
