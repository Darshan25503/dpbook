use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Unique identifier for a contact
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContactId(Uuid);

impl ContactId {
    /// Create a new unique contact ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create a contact ID from a UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the inner UUID
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ContactId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ContactId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ContactId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<ContactId> for Uuid {
    fn from(id: ContactId) -> Self {
        id.0
    }
}
