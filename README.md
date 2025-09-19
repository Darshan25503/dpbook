# dpbook - CLI Phonebook Application

[![CI/CD Pipeline](https://github.com/Darshan25503/dpbook/actions/workflows/ci.yml/badge.svg)](https://github.com/Darshan25503/dpbook/actions/workflows/ci.yml)
[![Release](https://github.com/Darshan25503/dpbook/actions/workflows/release.yml/badge.svg)](https://github.com/Darshan25503/dpbook/actions/workflows/release.yml)
[![Crates.io](https://img.shields.io/crates/v/dpbook.svg)](https://crates.io/crates/dpbook)
[![Docker](https://img.shields.io/docker/v/darshan25503/dpbook?label=docker)](https://hub.docker.com/r/darshan25503/dpbook)

A Rust-based command-line phonebook application built with clean architecture principles and SOLID design patterns.

## Features

- **Add contacts** with names, phone numbers, and email addresses
- **Search contacts** by name, phone, or email
- **List contacts** with pagination and sorting options
- **Update contact** information
- **Delete contacts** with confirmation
- **Persistent storage** using JSON files
- **Input validation** and comprehensive error handling

## Architecture

This application follows **Clean Architecture** principles with clear separation of concerns:

```
src/
├── domain/           # Business logic and entities
│   ├── entities/     # Core business entities (Contact)
│   ├── value_objects/# Value objects (PhoneNumber, Email, ContactId)
│   ├── repositories/ # Repository interfaces (abstractions)
│   └── errors.rs     # Domain-specific errors
├── application/      # Use cases and business rules
│   ├── use_cases/    # Individual use cases (Add, Find, Update, etc.)
│   ├── services/     # Application services
│   └── validation.rs # Input validation utilities
├── infrastructure/   # External concerns (file storage, etc.)
│   ├── repositories/ # Repository implementations
│   └── persistence/  # File storage implementation
└── presentation/     # User interface (CLI)
    └── cli/          # Command-line interface
```

## SOLID Principles Applied

### Single Responsibility Principle (SRP)
- Each use case handles only one business operation
- Value objects have single, well-defined purposes
- Separate modules for different concerns (validation, formatting, etc.)

### Open/Closed Principle (OCP)
- Repository trait allows for different storage implementations
- Use cases are closed for modification but open for extension
- New contact fields can be added without changing existing code

### Liskov Substitution Principle (LSP)
- Repository implementations can be substituted without breaking functionality
- Value objects maintain their contracts across implementations

### Interface Segregation Principle (ISP)
- Repository interfaces are focused and specific
- CLI commands are separated into distinct operations

### Dependency Inversion Principle (DIP)
- Use cases depend on repository abstractions, not concrete implementations
- High-level modules don't depend on low-level modules

## Installation

### Option 1: Download Pre-built Binaries

Download the latest release for your platform from the [releases page](https://github.com/Darshan25503/dpbook/releases).

### Option 2: Install from crates.io

```bash
cargo install dpbook
```

### Option 3: Docker

```bash
# Pull and run the Docker image
docker pull darshan25503/dpbook:latest

# Run with persistent data
docker run -v $(pwd)/data:/app/data darshan25503/dpbook:latest list
```

### Option 4: Building from Source

#### Prerequisites
- Rust 1.70 or later
- Cargo (comes with Rust)

```bash
# Clone the repository
git clone https://github.com/Darshan25503/dpbook.git
cd dpbook

# Build the application
cargo build --release

# The binary will be available at target/release/dpbook
```

## Usage

### Basic Commands

```bash
# Add a new contact
dpbook add --first-name "John" --last-name "Doe" --phone "555-123-4567" --email "john@example.com"

# List all contacts
dpbook list

# Search for contacts
dpbook search "john"

# Find a specific contact by ID
dpbook find <contact-id>

# Update a contact
dpbook update <contact-id> --first-name "Jane" --add-phone "555-987-6543"

# Delete a contact
dpbook delete <contact-id>

# Show statistics
dpbook stats
```

### Advanced Usage

```bash
# List with pagination and sorting
dpbook list --page 0 --page-size 5 --sort-by last-name --reverse

# Add contact with multiple phone numbers and emails
dpbook add \
  --first-name "Alice" \
  --last-name "Smith" \
  --phone "555-111-2222" \
  --phone "555-333-4444" \
  --email "alice@work.com" \
  --email "alice@personal.com"

# Update contact by adding and removing information
dpbook update <contact-id> \
  --add-phone "555-999-8888" \
  --remove-email "old@email.com" \
  --notes "Updated contact information"

# Use custom data file
dpbook --file /path/to/contacts.json list
```

## Data Storage

Contacts are stored in a JSON file (default: `contacts.json` in the current directory). The file is created automatically when you add your first contact.

### Data Format

```json
{
  "contacts": {
    "contact-uuid": {
      "id": "contact-uuid",
      "first_name": "John",
      "last_name": "Doe",
      "phone_numbers": [
        {
          "value": "5551234567"
        }
      ],
      "emails": [
        {
          "value": "john@example.com"
        }
      ],
      "notes": null,
      "tags": [],
      "metadata": {}
    }
  }
}
```

## Development



### Code Quality

```bash
# Check code formatting
cargo fmt --check

# Run clippy for linting
cargo clippy

# Check for security vulnerabilities
cargo audit
```

### Project Structure

The codebase follows clean architecture with these layers:

1. **Domain Layer**: Core business logic, entities, and value objects
2. **Application Layer**: Use cases and application services
3. **Infrastructure Layer**: External dependencies and implementations
4. **Presentation Layer**: User interface (CLI)

## Error Handling

The application uses comprehensive error handling with custom error types for each layer:

- **Domain Errors**: Business rule violations, validation errors
- **Application Errors**: Use case specific errors
- **Infrastructure Errors**: File system, network, database errors
- **Presentation Errors**: CLI input/output errors

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes following the existing architecture
4. Run formatting and linting (`cargo fmt && cargo clippy`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- CLI parsing with [clap](https://clap.rs/)
- Serialization with [serde](https://serde.rs/)
- Error handling with [thiserror](https://github.com/dtolnay/thiserror)
