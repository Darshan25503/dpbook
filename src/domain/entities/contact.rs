use crate::domain::value_objects::{ContactId, Email, PhoneNumber};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Contact entity representing a person in the phonebook
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    id: ContactId,
    first_name: String,
    last_name: String,
    phone_numbers: Vec<PhoneNumber>,
    emails: Vec<Email>,
    notes: Option<String>,
    tags: Vec<String>,
    metadata: HashMap<String, String>,
}

impl Contact {
    /// Create a new contact
    pub fn new(
        first_name: String,
        last_name: String,
        phone_numbers: Vec<PhoneNumber>,
        emails: Vec<Email>,
    ) -> Self {
        Self {
            id: ContactId::new(),
            first_name,
            last_name,
            phone_numbers,
            emails,
            notes: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Create a contact with existing ID (for loading from storage)
    pub fn with_id(
        id: ContactId,
        first_name: String,
        last_name: String,
        phone_numbers: Vec<PhoneNumber>,
        emails: Vec<Email>,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            phone_numbers,
            emails,
            notes: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    // Getters
    pub fn id(&self) -> &ContactId {
        &self.id
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn phone_numbers(&self) -> &[PhoneNumber] {
        &self.phone_numbers
    }

    pub fn emails(&self) -> &[Email] {
        &self.emails
    }

    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    // Setters
    pub fn set_first_name(&mut self, first_name: String) {
        self.first_name = first_name;
    }

    pub fn set_last_name(&mut self, last_name: String) {
        self.last_name = last_name;
    }

    pub fn set_notes(&mut self, notes: Option<String>) {
        self.notes = notes;
    }

    pub fn add_phone_number(&mut self, phone: PhoneNumber) {
        if !self.phone_numbers.contains(&phone) {
            self.phone_numbers.push(phone);
        }
    }

    pub fn remove_phone_number(&mut self, phone: &PhoneNumber) {
        self.phone_numbers.retain(|p| p != phone);
    }

    pub fn add_email(&mut self, email: Email) {
        if !self.emails.contains(&email) {
            self.emails.push(email);
        }
    }

    pub fn remove_email(&mut self, email: &Email) {
        self.emails.retain(|e| e != email);
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn remove_metadata(&mut self, key: &str) {
        self.metadata.remove(key);
    }

    /// Check if contact matches search query
    pub fn matches_search(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        
        self.first_name.to_lowercase().contains(&query_lower)
            || self.last_name.to_lowercase().contains(&query_lower)
            || self.phone_numbers.iter().any(|p| p.value().contains(&query_lower))
            || self.emails.iter().any(|e| e.value().contains(&query_lower))
            || self.notes.as_ref().map_or(false, |n| n.to_lowercase().contains(&query_lower))
            || self.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
    }
}
