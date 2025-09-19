use crate::application::services::ContactService;
use crate::application::use_cases::{
    AddContactRequest, DeleteContactRequest, FindContactRequest,
    ListContactsRequest, SearchContactsRequest, UpdateContactRequest
};
use crate::domain::value_objects::{ContactId, Email, PhoneNumber};
use crate::infrastructure::repositories::FileContactRepository;
use crate::presentation::cli::{commands::*, formatters::ContactFormatter};
use clap::Parser;
use std::io::{self, Write};
use std::sync::Arc;
use uuid::Uuid;

/// Main CLI application
pub struct PhonebookApp {
    contact_service: ContactService,
}

impl PhonebookApp {
    /// Create a new phonebook app with file-based storage
    pub fn new(file_path: String) -> Self {
        let repository = Arc::new(FileContactRepository::new(file_path));
        let contact_service = ContactService::new(repository);

        Self { contact_service }
    }

    /// Run the CLI application
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let cli = Cli::parse();
        let app = PhonebookApp::new(cli.file);

        match cli.command {
            Commands::Add {
                first_name,
                last_name,
                phone,
                email,
                notes: _,
                tag: _,
            } => app.handle_add(first_name, last_name, phone, email),

            Commands::Find { id } => app.handle_find(id),

            Commands::List {
                page,
                page_size,
                sort_by,
                reverse,
            } => app.handle_list(page, page_size, sort_by, reverse),

            Commands::Search { query } => app.handle_search(query),

            Commands::Update {
                id,
                first_name,
                last_name,
                add_phone,
                remove_phone,
                add_email,
                remove_email,
                notes: _,
                add_tag: _,
                remove_tag: _,
            } => app.handle_update(
                id,
                first_name,
                last_name,
                add_phone,
                remove_phone,
                add_email,
                remove_email,
            ),

            Commands::Delete { id, yes } => app.handle_delete(id, yes),

            Commands::Stats => app.handle_stats(),
        }
    }

    fn handle_add(
        &self,
        first_name: String,
        last_name: String,
        phone_strings: Vec<String>,
        email_strings: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Parse phone numbers
        let mut phone_numbers = Vec::new();
        for phone_str in phone_strings {
            match PhoneNumber::try_from(phone_str.as_str()) {
                Ok(phone) => phone_numbers.push(phone),
                Err(e) => {
                    eprintln!("Invalid phone number '{}': {}", phone_str, e);
                    return Ok(());
                }
            }
        }

        // Parse emails
        let mut emails = Vec::new();
        for email_str in email_strings {
            match Email::try_from(email_str.as_str()) {
                Ok(email) => emails.push(email),
                Err(e) => {
                    eprintln!("Invalid email '{}': {}", email_str, e);
                    return Ok(());
                }
            }
        }

        let request = AddContactRequest {
            first_name,
            last_name,
            phone_numbers,
            emails,
        };

        match self.contact_service.add_contact(request) {
            Ok(response) => {
                println!("✓ {}", response.message);
                println!("Contact ID: {}", response.contact_id);
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        Ok(())
    }

    fn handle_find(&self, id_str: String) -> Result<(), Box<dyn std::error::Error>> {
        let id = match Uuid::parse_str(&id_str) {
            Ok(uuid) => ContactId::from_uuid(uuid),
            Err(_) => {
                eprintln!("Invalid contact ID format");
                return Ok(());
            }
        };

        let request = FindContactRequest { contact_id: id };

        match self.contact_service.find_contact(request) {
            Ok(response) => {
                if response.found {
                    if let Some(contact) = response.contact {
                        println!("{}", ContactFormatter::format_contact(&contact));
                    }
                } else {
                    println!("Contact not found");
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        Ok(())
    }

    fn handle_list(
        &self,
        page: usize,
        page_size: usize,
        sort_by: SortField,
        reverse: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sort_by = match sort_by {
            SortField::FirstName => crate::application::use_cases::list_contacts::SortBy::FirstName,
            SortField::LastName => crate::application::use_cases::list_contacts::SortBy::LastName,
            SortField::FullName => crate::application::use_cases::list_contacts::SortBy::FullName,
        };

        let request = ListContactsRequest {
            page,
            page_size,
            sort_by,
            reverse,
        };

        match self.contact_service.list_contacts(request) {
            Ok(response) => {
                if response.contacts.is_empty() {
                    println!("No contacts found");
                    return Ok(());
                }

                println!("{}", ContactFormatter::format_list_header());
                println!("{}", ContactFormatter::format_separator());

                for contact in &response.contacts {
                    println!("{}", ContactFormatter::format_contact_compact(contact));
                }

                println!("{}", ContactFormatter::format_separator());
                println!(
                    "{}",
                    ContactFormatter::format_pagination_info(
                        response.page,
                        response.page_size,
                        response.total_count,
                        response.has_more
                    )
                );
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        Ok(())
    }

    fn handle_search(&self, query: String) -> Result<(), Box<dyn std::error::Error>> {
        let request = SearchContactsRequest { query };

        match self.contact_service.search_contacts(request) {
            Ok(response) => {
                println!(
                    "{}",
                    ContactFormatter::format_search_summary(&response.query, response.count)
                );

                if !response.contacts.is_empty() {
                    println!("{}", ContactFormatter::format_list_header());
                    println!("{}", ContactFormatter::format_separator());

                    for contact in &response.contacts {
                        println!("{}", ContactFormatter::format_contact_compact(contact));
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        Ok(())
    }

    fn handle_update(
        &self,
        id_str: String,
        first_name: Option<String>,
        last_name: Option<String>,
        add_phone_strings: Vec<String>,
        remove_phone_strings: Vec<String>,
        add_email_strings: Vec<String>,
        remove_email_strings: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let id = match Uuid::parse_str(&id_str) {
            Ok(uuid) => ContactId::from_uuid(uuid),
            Err(_) => {
                eprintln!("Invalid contact ID format");
                return Ok(());
            }
        };

        // Parse phone numbers to add
        let mut add_phone_numbers = Vec::new();
        for phone_str in add_phone_strings {
            match PhoneNumber::try_from(phone_str.as_str()) {
                Ok(phone) => add_phone_numbers.push(phone),
                Err(e) => {
                    eprintln!("Invalid phone number '{}': {}", phone_str, e);
                    return Ok(());
                }
            }
        }

        // Parse phone numbers to remove
        let mut remove_phone_numbers = Vec::new();
        for phone_str in remove_phone_strings {
            match PhoneNumber::try_from(phone_str.as_str()) {
                Ok(phone) => remove_phone_numbers.push(phone),
                Err(e) => {
                    eprintln!("Invalid phone number '{}': {}", phone_str, e);
                    return Ok(());
                }
            }
        }

        // Parse emails to add
        let mut add_emails = Vec::new();
        for email_str in add_email_strings {
            match Email::try_from(email_str.as_str()) {
                Ok(email) => add_emails.push(email),
                Err(e) => {
                    eprintln!("Invalid email '{}': {}", email_str, e);
                    return Ok(());
                }
            }
        }

        // Parse emails to remove
        let mut remove_emails = Vec::new();
        for email_str in remove_email_strings {
            match Email::try_from(email_str.as_str()) {
                Ok(email) => remove_emails.push(email),
                Err(e) => {
                    eprintln!("Invalid email '{}': {}", email_str, e);
                    return Ok(());
                }
            }
        }

        let request = UpdateContactRequest {
            contact_id: id,
            first_name,
            last_name,
            notes: None,
            add_phone_numbers,
            remove_phone_numbers,
            add_emails,
            remove_emails,
            add_tags: Vec::new(),
            remove_tags: Vec::new(),
        };

        match self.contact_service.update_contact(request) {
            Ok(response) => {
                println!("✓ {}", response.message);
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        Ok(())
    }

    fn handle_delete(&self, id_str: String, skip_confirmation: bool) -> Result<(), Box<dyn std::error::Error>> {
        let id = match Uuid::parse_str(&id_str) {
            Ok(uuid) => ContactId::from_uuid(uuid),
            Err(_) => {
                eprintln!("Invalid contact ID format");
                return Ok(());
            }
        };

        // Show contact details and ask for confirmation
        if !skip_confirmation {
            let find_request = FindContactRequest { contact_id: id.clone() };
            match self.contact_service.find_contact(find_request) {
                Ok(response) => {
                    if !response.found {
                        eprintln!("Contact not found");
                        return Ok(());
                    }
                    
                    if let Some(contact) = response.contact {
                        println!("Contact to delete:");
                        println!("{}", ContactFormatter::format_contact(&contact));
                        
                        print!("Are you sure you want to delete this contact? (y/N): ");
                        io::stdout().flush()?;
                        
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        
                        if !input.trim().to_lowercase().starts_with('y') {
                            println!("Deletion cancelled");
                            return Ok(());
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error finding contact: {}", e);
                    return Ok(());
                }
            }
        }

        let request = DeleteContactRequest { contact_id: id };

        match self.contact_service.delete_contact(request) {
            Ok(response) => {
                println!("✓ {}", response.message);
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        Ok(())
    }

    fn handle_stats(&self) -> Result<(), Box<dyn std::error::Error>> {
        let request = ListContactsRequest::default();

        match self.contact_service.list_contacts(request) {
            Ok(response) => {
                println!("{}", ContactFormatter::format_stats(response.total_count));
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        Ok(())
    }
}
