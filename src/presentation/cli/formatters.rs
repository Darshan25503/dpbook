use crate::domain::entities::Contact;

/// Formatter for displaying contacts in various formats
pub struct ContactFormatter;

impl ContactFormatter {
    /// Format a single contact for display
    pub fn format_contact(contact: &Contact) -> String {
        let mut output = String::new();
        
        output.push_str(&format!("ID: {}\n", contact.id()));
        output.push_str(&format!("Name: {}\n", contact.full_name()));
        
        if !contact.phone_numbers().is_empty() {
            output.push_str("Phone Numbers:\n");
            for phone in contact.phone_numbers() {
                output.push_str(&format!("  - {}\n", phone));
            }
        }
        
        if !contact.emails().is_empty() {
            output.push_str("Emails:\n");
            for email in contact.emails() {
                output.push_str(&format!("  - {}\n", email));
            }
        }
        
        if let Some(notes) = contact.notes() {
            output.push_str(&format!("Notes: {}\n", notes));
        }
        
        if !contact.tags().is_empty() {
            output.push_str(&format!("Tags: {}\n", contact.tags().join(", ")));
        }
        
        output
    }

    /// Format a contact for list display (compact format)
    pub fn format_contact_compact(contact: &Contact) -> String {
        let phone = contact.phone_numbers()
            .first()
            .map(|p| p.to_string())
            .unwrap_or_else(|| "No phone".to_string());
        
        let email = contact.emails()
            .first()
            .map(|e| e.to_string())
            .unwrap_or_else(|| "No email".to_string());

        format!(
            "{:<8} {:<25} {:<15} {}",
            &contact.id().to_string()[..8],
            contact.full_name(),
            phone,
            email
        )
    }

    /// Format the header for contact list
    pub fn format_list_header() -> String {
        format!(
            "{:<8} {:<25} {:<15} {}",
            "ID", "Name", "Phone", "Email"
        )
    }

    /// Format a separator line
    pub fn format_separator() -> String {
        "-".repeat(80)
    }

    /// Format search results summary
    pub fn format_search_summary(query: &str, count: usize) -> String {
        format!("Found {} contact(s) matching '{}'\n", count, query)
    }

    /// Format list pagination info
    pub fn format_pagination_info(page: usize, page_size: usize, total: usize, has_more: bool) -> String {
        let start = page * page_size + 1;
        let end = std::cmp::min((page + 1) * page_size, total);
        
        let mut info = format!("Showing {} - {} of {} contacts", start, end, total);
        
        if has_more {
            info.push_str(&format!(" (Page {})", page + 1));
        }
        
        info
    }

    /// Format statistics
    pub fn format_stats(total_contacts: usize) -> String {
        format!("Total contacts: {}", total_contacts)
    }
}
