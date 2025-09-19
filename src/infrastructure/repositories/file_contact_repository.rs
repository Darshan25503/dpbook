use crate::domain::entities::Contact;
use crate::domain::repositories::{ContactRepositorySync, RepositoryError};
use crate::domain::value_objects::ContactId;
use crate::infrastructure::persistence::FileStorage;
use std::collections::HashMap;
use std::sync::Mutex;

/// File-based implementation of ContactRepository
/// Uses JSON file for persistence with in-memory caching
pub struct FileContactRepository {
    storage: FileStorage,
    cache: Mutex<Option<HashMap<ContactId, Contact>>>,
}

impl FileContactRepository {
    pub fn new(file_path: String) -> Self {
        Self {
            storage: FileStorage::new(file_path),
            cache: Mutex::new(None),
        }
    }

    /// Load contacts into cache if not already loaded
    fn ensure_cache_loaded(&self) -> Result<(), RepositoryError> {
        let mut cache = self.cache.lock().unwrap();
        if cache.is_none() {
            let contacts = self.storage.load_contacts()?;
            *cache = Some(contacts);
        }
        Ok(())
    }

    /// Get a reference to the cached contacts
    fn with_cache<F, R>(&self, f: F) -> Result<R, RepositoryError>
    where
        F: FnOnce(&HashMap<ContactId, Contact>) -> R,
    {
        self.ensure_cache_loaded()?;
        let cache = self.cache.lock().unwrap();
        let contacts = cache.as_ref().unwrap();
        Ok(f(contacts))
    }

    /// Modify the cached contacts and save to file
    fn modify_cache<F>(&self, f: F) -> Result<(), RepositoryError>
    where
        F: FnOnce(&mut HashMap<ContactId, Contact>) -> Result<(), RepositoryError>,
    {
        self.ensure_cache_loaded()?;
        let mut cache = self.cache.lock().unwrap();
        let contacts = cache.as_mut().unwrap();
        f(contacts)?;
        self.storage.save_contacts(contacts)?;
        Ok(())
    }
}

impl ContactRepositorySync for FileContactRepository {
    fn save(&self, contact: Contact) -> Result<(), RepositoryError> {
        let contact_id = contact.id().clone();
        self.modify_cache(|contacts| {
            if contacts.contains_key(&contact_id) {
                return Err(RepositoryError::ContactAlreadyExists(contact_id.to_string()));
            }
            contacts.insert(contact_id, contact);
            Ok(())
        })
    }

    fn find_by_id(&self, id: &ContactId) -> Result<Option<Contact>, RepositoryError> {
        self.with_cache(|contacts| contacts.get(id).cloned())
    }

    fn find_all(&self) -> Result<Vec<Contact>, RepositoryError> {
        self.with_cache(|contacts| contacts.values().cloned().collect())
    }

    fn update(&self, contact: Contact) -> Result<(), RepositoryError> {
        let contact_id = contact.id().clone();
        self.modify_cache(|contacts| {
            if !contacts.contains_key(&contact_id) {
                return Err(RepositoryError::ContactNotFound(contact_id.to_string()));
            }
            contacts.insert(contact_id, contact);
            Ok(())
        })
    }

    fn delete(&self, id: &ContactId) -> Result<(), RepositoryError> {
        self.modify_cache(|contacts| {
            if contacts.remove(id).is_none() {
                return Err(RepositoryError::ContactNotFound(id.to_string()));
            }
            Ok(())
        })
    }

    fn search(&self, query: &str) -> Result<Vec<Contact>, RepositoryError> {
        self.with_cache(|contacts| {
            contacts
                .values()
                .filter(|contact| contact.matches_search(query))
                .cloned()
                .collect()
        })
    }

    fn exists(&self, id: &ContactId) -> Result<bool, RepositoryError> {
        self.with_cache(|contacts| contacts.contains_key(id))
    }

    fn count(&self) -> Result<usize, RepositoryError> {
        self.with_cache(|contacts| contacts.len())
    }
}
