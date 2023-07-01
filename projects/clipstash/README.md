# Clipstash

Final project for the course

## Architecture

Clipstash is a fullstack app composed of Web, Service, Data and Domain layers

- The terminology for the models is as follows:
  1. Domain - Business logic
  2. Model - Data layer
  3. Ask - Service layer

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

### Domain Layer

- Contains data types that are shared between all layers
- Enforces business rules on data, domain objects cannot be created unless all rules pass

#### Field Structs

- Field structs are used to validate data before it is stored in the database
- Every field for the `Clip` structure is a module, this guarantees that all data is valid before it is stored in the database, and splits the code into smaller, more manageable chunks, even with extensive validation logic
