use crate::domain::entities::Contact;
use crate::domain::repositories::RepositoryError;
use crate::domain::value_objects::ContactId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// File-based storage implementation
/// Handles serialization and persistence of contacts to JSON file
#[derive(Debug)]
pub struct FileStorage {
    file_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ContactsData {
    contacts: HashMap<String, Contact>,
}

impl FileStorage {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    /// Load all contacts from file
    pub fn load_contacts(&self) -> Result<HashMap<ContactId, Contact>, RepositoryError> {
        if !Path::new(&self.file_path).exists() {
            return Ok(HashMap::new());
        }

        let content = fs::read_to_string(&self.file_path)
            .map_err(|e| RepositoryError::IoError(format!("Failed to read file: {}", e)))?;

        if content.trim().is_empty() {
            return Ok(HashMap::new());
        }

        let data: ContactsData = serde_json::from_str(&content)
            .map_err(|e| RepositoryError::SerializationError(format!("Failed to deserialize: {}", e)))?;

        let mut contacts = HashMap::new();
        for (id_str, contact) in data.contacts {
            let id = uuid::Uuid::parse_str(&id_str)
                .map_err(|e| RepositoryError::SerializationError(format!("Invalid UUID: {}", e)))?;
            contacts.insert(ContactId::from_uuid(id), contact);
        }

        Ok(contacts)
    }

    /// Save all contacts to file
    pub fn save_contacts(&self, contacts: &HashMap<ContactId, Contact>) -> Result<(), RepositoryError> {
        // Create directory if it doesn't exist
        if let Some(parent) = Path::new(&self.file_path).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| RepositoryError::IoError(format!("Failed to create directory: {}", e)))?;
        }

        let mut data = ContactsData {
            contacts: HashMap::new(),
        };

        for (id, contact) in contacts {
            data.contacts.insert(id.to_string(), contact.clone());
        }

        let json = serde_json::to_string_pretty(&data)
            .map_err(|e| RepositoryError::SerializationError(format!("Failed to serialize: {}", e)))?;

        fs::write(&self.file_path, json)
            .map_err(|e| RepositoryError::IoError(format!("Failed to write file: {}", e)))?;

        Ok(())
    }

    /// Get the file path
    pub fn file_path(&self) -> &str {
        &self.file_path
    }
}
