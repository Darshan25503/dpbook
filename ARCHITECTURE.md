# Architecture Documentation

## Overview

This phonebook application is built using **Clean Architecture** principles, ensuring separation of concerns, testability, and maintainability. The architecture follows the **SOLID principles** and implements several design patterns.

## Architecture Layers

### 1. Domain Layer (`src/domain/`)

The innermost layer containing business logic and rules.

#### Entities (`src/domain/entities/`)
- **Contact**: Core business entity representing a person in the phonebook
  - Contains business logic for contact management
  - Implements search functionality
  - Maintains data integrity

#### Value Objects (`src/domain/value_objects/`)
- **ContactId**: Unique identifier for contacts using UUID
- **PhoneNumber**: Validates and formats phone numbers
- **Email**: Validates and normalizes email addresses

#### Repository Interfaces (`src/domain/repositories/`)
- **ContactRepository**: Async repository interface
- **ContactRepositorySync**: Synchronous repository interface
- **RepositoryError**: Domain-specific repository errors

#### Domain Errors (`src/domain/errors.rs`)
- Comprehensive error hierarchy for all layers
- Type-safe error handling with `thiserror`

### 2. Application Layer (`src/application/`)

Contains use cases and application-specific business rules.

#### Use Cases (`src/application/use_cases/`)
Each use case follows the **Single Responsibility Principle**:

- **AddContactUseCase**: Handles contact creation
- **FindContactUseCase**: Retrieves contacts by ID
- **UpdateContactUseCase**: Modifies existing contacts
- **DeleteContactUseCase**: Removes contacts
- **ListContactsUseCase**: Lists contacts with pagination/sorting
- **SearchContactsUseCase**: Searches contacts by query

#### Services (`src/application/services/`)
- **ContactService**: Facade pattern implementation
- Orchestrates use cases
- Provides simplified interface for presentation layer

#### Validation (`src/application/validation.rs`)
- **Validator**: Centralized validation utilities
- Input sanitization and business rule validation
- Reusable validation functions

### 3. Infrastructure Layer (`src/infrastructure/`)

Handles external concerns and implementations.

#### Repository Implementations (`src/infrastructure/repositories/`)
- **FileContactRepository**: JSON file-based storage
- Implements `ContactRepositorySync` trait
- In-memory caching with file persistence

#### Persistence (`src/infrastructure/persistence/`)
- **FileStorage**: Low-level file operations
- JSON serialization/deserialization
- Error handling for I/O operations

### 4. Presentation Layer (`src/presentation/`)

User interface and external API.

#### CLI (`src/presentation/cli/`)
- **Commands**: Clap-based command definitions
- **App**: Main application orchestrator
- **Formatters**: Output formatting utilities

## SOLID Principles Implementation

### Single Responsibility Principle (SRP)
- Each use case has one reason to change
- Value objects handle single concerns
- Separate modules for validation, formatting, persistence

### Open/Closed Principle (OCP)
- Repository trait allows new storage implementations
- Use cases are extensible without modification
- Value objects can be extended with new validation rules

### Liskov Substitution Principle (LSP)
- Any `ContactRepositorySync` implementation is substitutable
- Value objects maintain contracts across implementations

### Interface Segregation Principle (ISP)
- Repository interfaces are focused and minimal
- CLI commands are separated by functionality
- No client depends on methods it doesn't use

### Dependency Inversion Principle (DIP)
- Use cases depend on repository abstractions
- High-level modules don't depend on low-level details
- Dependency injection through constructor parameters

## Design Patterns Used

### Repository Pattern
- Abstracts data access logic
- Enables testing with mock implementations
- Separates business logic from data persistence

### Use Case Pattern
- Encapsulates business operations
- Single entry point for each business function
- Clear input/output contracts

### Value Object Pattern
- Immutable objects representing domain concepts
- Built-in validation and formatting
- Type safety for domain primitives

### Facade Pattern
- `ContactService` provides simplified interface
- Hides complexity of multiple use cases
- Single point of entry for application operations

### Builder Pattern (Implicit)
- Contact creation with fluent interface
- Request/Response DTOs for use cases

## Error Handling Strategy

### Hierarchical Error Types
```rust
PhonebookError
├── PresentationError
├── ApplicationError
│   ├── DomainError
│   └── RepositoryError
├── InfrastructureError
└── DomainError
```

### Error Propagation
- Errors bubble up through layers
- Each layer can add context
- Type-safe error handling with `Result<T, E>`

### Validation Strategy
- Input validation at presentation layer
- Business rule validation in domain layer
- Data integrity validation in value objects

## Testing Strategy

### Unit Tests
- Value object validation logic
- Use case business logic
- Repository implementations

### Integration Tests
- End-to-end CLI functionality
- File persistence operations
- Error handling scenarios

### Test Organization
```
tests/
├── unit/
│   ├── domain/
│   ├── application/
│   └── infrastructure/
└── integration/
    └── cli/
```

## Data Flow

### Adding a Contact
1. CLI parses command arguments
2. Presentation layer validates input format
3. Use case validates business rules
4. Domain entities are created with value objects
5. Repository persists to storage
6. Success/error response flows back

### Searching Contacts
1. CLI receives search query
2. Use case validates query format
3. Repository searches stored contacts
4. Domain entities filter by search criteria
5. Results formatted for display
6. Output presented to user

## Extension Points

### Adding New Storage Backends
1. Implement `ContactRepositorySync` trait
2. Add new module in `infrastructure/repositories/`
3. Update dependency injection in main

### Adding New Contact Fields
1. Update `Contact` entity
2. Modify serialization format
3. Update CLI commands and validation
4. Add migration logic if needed

### Adding New Output Formats
1. Create new formatter in `presentation/cli/formatters/`
2. Add command-line options
3. Update output logic in CLI app

## Performance Considerations

### Caching Strategy
- In-memory cache for frequently accessed data
- Lazy loading of contact data
- Write-through cache for consistency

### File I/O Optimization
- Batch operations where possible
- Atomic file writes for data integrity
- Minimal file reads with caching

### Memory Management
- Efficient data structures (HashMap for lookups)
- Minimal cloning with references where possible
- Streaming for large datasets (future enhancement)

## Security Considerations

### Input Validation
- Comprehensive validation at all entry points
- Sanitization of user input
- Protection against injection attacks

### File System Security
- Safe file operations with proper error handling
- Atomic writes to prevent corruption
- Configurable file permissions

### Data Privacy
- No sensitive data logging
- Secure handling of contact information
- Optional encryption for storage (future enhancement)
