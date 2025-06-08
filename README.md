# Quotes Client

A simple web client that consumes a REST API to display quotes using Rust, Leptos framework, and WebAssembly. Features a responsive interface with client-side rendering for browsing and managing quotes.

## Author

Chia-Wei Hsu (chiawei@pdx.edu)

## Features

- Browse all quotes from the database
- View random quotes
- Create new quotes (with authentication)
- Update existing quotes (with authentication)
- Delete quotes (with authentication)
- Responsive design with Tailwind CSS
- Client-side routing with Leptos Router

## Technology Stack

- **Rust** - Programming language
- **Leptos** - Web framework for WebAssembly
- **WebAssembly** - Runtime for web applications
- **Trunk** - Build tool for Rust WebAssembly applications
- **Tailwind CSS** - Utility-first CSS framework
- **reqwasm** - HTTP client for WebAssembly

## Dependencies

- leptos
- leptos_router
- serde
- serde_json
- reqwasm
- web-sys
- See `Cargo.toml` for a complete list

## Setup

1. Make sure you have Rust installed
2. Install Trunk build tool:
   ```bash
   cargo install trunk
   ```
3. Clone this repository
4. Make sure the [quote server](https://github.com/planetaska/quote-server) is running at `http://localhost:3000`

## Running the Application

Start the development server with:

```bash
trunk serve
```

The client will be available at: `http://localhost:8080`
Note: the quote server will only accept `localhost` - not `127.0.0.1` for its default CORS config.

For production build:

```bash
trunk build --release
```

## Available Pages

- `/` - Home page with navigation
- `/quotes` - Browse all quotes
- `/quotes/random` - View a random quote
- `/quotes/{id}` - View a specific quote
- `/quotes/create` - Create a new quote (requires authentication)
- `/about` - About page

## API Integration

This client consumes the following API endpoints from the quote server:

- `GET /api/v1/quotes` - Get all quotes as JSON
- `GET /api/v1/quotes/{id}` - Get a specific quote by ID as JSON
- `GET /api/v1/quotes/random` - Get a random quote as JSON
- `POST /api/v1/quotes` - Create a new quote (requires JWT authentication)
- `PUT /api/v1/quotes/{id}` - Update an existing quote (requires JWT authentication)
- `DELETE /api/v1/quotes/{id}` - Delete a quote by ID (requires JWT authentication)

### Authentication

Protected operations require JWT authentication. The client handles authentication by registering with the server using the provided credentials.

## Project Structure

```
.
├── public/
│   ├── openapi.json            # OpenAPI specification
│   ├── favicon.ico             # Favicon and app icons
│   └── ...
├── src/
│   ├── components/
│   │   ├── app.rs              # Main application component
│   │   ├── home.rs             # Home page component
│   │   ├── quotes_page.rs      # Quotes listing page
│   │   ├── quote_display.rs    # Individual quote display
│   │   ├── create_quote_form.rs # Quote creation form
│   │   ├── navigation.rs       # Navigation component
│   │   └── ...
│   ├── api.rs                  # API client functions
│   ├── types.rs                # Type definitions
│   └── main.rs                 # Application entry point
├── index.html                  # HTML template
├── styles.css                  # Global styles
├── tailwind.config.js          # Tailwind CSS configuration
├── Trunk.toml                  # Trunk build configuration
├── Cargo.toml                  # Cargo package configuration
└── README.md                   # This file
```

## Development

To run in development mode with hot reloading:

```bash
trunk serve
```

To build for production:

```bash
trunk build --release
```

The built files will be in the `dist/` directory.

## Prerequisites

- The quote server must be running at `http://localhost:3000`