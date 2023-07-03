# Clipstash

Final project for the course

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Running

### Web Server

To run the application, run the following command:

```bash
cargo run -q --bin httpd
```

### CLI Client

To run the CLI client and make a request to an endpoint, run the following command:

```bash
cargo run --bin client -- --api-key {generated api key} {subcommand and arguments}
```

### Database

The database is managed by SQLx, to install the CLI run:

```bash
cargo install sqlx-cli
```

To add a new migration run:

```bash
sqlx migrate add <name>
```

To apply the migrations run:

```bash
sqlx migrate run
```

## Main Features

- User authentication
- API Key authentication
- Clip management
- Clip hit tracking via multithreaded background tasks

## Architecture

Clipstash is a fullstack app composed of Web, Service, Data and Domain layers

- The terminology for the models is as follows:
  1. Domain - Business logic
  2. Model - Data layer
  3. Ask - Service layer

### Web Layer

- Exposes the application to the outside world
- Responsible for:
  - Reporting errors
  - Rendering pages
  - Providing the API
- Will manage application state

  - Establish database connections
  - Port bindings
  - Spawning background tasks

#### Templates

- The web layer utilizes templates to render pages
  - Templates are HTML with additional syntax to allow the server to inject data into the page
  - Handlebars is used as the templating engine
  - The server provides data for template rendering which is specified via `contexts`

#### Contexts

- Each web page has a dedicated context, which is variable data that is injected into the template
- Contexts are similar to `HashMaps`
- Rust data structures can be converted to contexts via the `serde` crate by implementing the `Debug` and `Serialize` trait

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

## Rocket

- Rocket is a web framework for Rust, it's an asynchronous web server that supports cookies, forms and encryption
- It handles state management, routing, templating, and has built-in testing mechanism
