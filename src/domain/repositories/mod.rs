pub mod contact_repository;
pub mod errors;

pub use contact_repository::{ContactRepository, ContactRepositorySync};
pub use errors::RepositoryError;
