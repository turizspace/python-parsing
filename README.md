# Python Parsing with Tree-Sitter

This Rust-based project uses the `tree-sitter` parser to analyze Python code and generate a graph representation of functions, classes, endpoints, and their relationships.

## Features

- **Parses Python files** to detect:
  - **Functions**
  - **Classes**
  - **Endpoints** (via decorators like `@app.route`)
- **Creates a graph** of nodes (classes, functions, endpoints) and edges (relationships between them).

## Running Tests

To run the tests and see detailed output, use the `-- --nocapture` flag:
```bash
cargo test -- --nocapture
```

This will display logs, & information about:
- Classes and functions found in Python files
- Created endpoints and their relationships
