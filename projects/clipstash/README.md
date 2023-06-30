# Clipstash

Final project for the course

## Architecture

Clipstash is a fullstack app composed of Web, Service, Data and Domain layers
// TODO: is it basically Domain-driven design?

### Web Component Layer

- Exposes the application to the outside world
- Responsible for:
  - Reporting errors
  - Rendering pages
  - Providing the API
- Will manage application state
  - Establish database connections
  - Port bindings
  - Spawning background tasks

### Service Layer

- Will serve as an intermediate layer between web requests and the database
- Abstracts user requests and data access
- Application logic will be implemented here so that all clients access the same logic

### Data Layer

- Manages data storage and retrieval
- Will be responsible for:
  - Database migrations
  - Database connections
  - Data access
- No logic outside of queries

### Domain

- Contains data types that are shared between all layers
- Enforces business rules on data, domain objects cannot be created unless all rules pass