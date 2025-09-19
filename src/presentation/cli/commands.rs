use clap::{Parser, Subcommand};

/// Phonebook CLI Application
#[derive(Parser)]
#[command(name = "phonebook")]
#[command(about = "A CLI phonebook application")]
#[command(version = "1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to the contacts file
    #[arg(short, long, default_value = "contacts.json")]
    pub file: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new contact
    Add {
        /// First name
        #[arg(short, long)]
        first_name: String,

        /// Last name
        #[arg(short, long)]
        last_name: String,

        /// Phone numbers (can be specified multiple times)
        #[arg(short, long)]
        phone: Vec<String>,

        /// Email addresses (can be specified multiple times)
        #[arg(short, long)]
        email: Vec<String>,

        /// Notes
        #[arg(short, long)]
        notes: Option<String>,

        /// Tags (can be specified multiple times)
        #[arg(short, long)]
        tag: Vec<String>,
    },

    /// Find a contact by ID
    Find {
        /// Contact ID
        id: String,
    },

    /// List all contacts
    List {
        /// Page number (0-based)
        #[arg(long, default_value = "0")]
        page: usize,

        /// Number of contacts per page
        #[arg(long, default_value = "10")]
        page_size: usize,

        /// Sort by field
        #[arg(long, default_value = "last-name")]
        sort_by: SortField,

        /// Reverse sort order
        #[arg(long)]
        reverse: bool,
    },

    /// Search contacts
    Search {
        /// Search query
        query: String,
    },

    /// Update a contact
    Update {
        /// Contact ID
        id: String,

        /// New first name
        #[arg(long)]
        first_name: Option<String>,

        /// New last name
        #[arg(long)]
        last_name: Option<String>,

        /// Add phone numbers
        #[arg(long)]
        add_phone: Vec<String>,

        /// Remove phone numbers
        #[arg(long)]
        remove_phone: Vec<String>,

        /// Add email addresses
        #[arg(long)]
        add_email: Vec<String>,

        /// Remove email addresses
        #[arg(long)]
        remove_email: Vec<String>,

        /// Set notes
        #[arg(long)]
        notes: Option<String>,

        /// Add tags
        #[arg(long)]
        add_tag: Vec<String>,

        /// Remove tags
        #[arg(long)]
        remove_tag: Vec<String>,
    },

    /// Delete a contact
    Delete {
        /// Contact ID
        id: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },

    /// Show statistics
    Stats,
}

#[derive(Clone, Debug)]
pub enum SortField {
    FirstName,
    LastName,
    FullName,
}

impl std::str::FromStr for SortField {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "first-name" | "firstname" => Ok(SortField::FirstName),
            "last-name" | "lastname" => Ok(SortField::LastName),
            "full-name" | "fullname" => Ok(SortField::FullName),
            _ => Err(format!("Invalid sort field: {}", s)),
        }
    }
}

impl std::fmt::Display for SortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortField::FirstName => write!(f, "first-name"),
            SortField::LastName => write!(f, "last-name"),
            SortField::FullName => write!(f, "full-name"),
        }
    }
}
