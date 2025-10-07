# AI Agent Instructions for module-dependency-graph-creator

This Rust project analyzes Python module dependencies and generates PlantUML diagrams visualizing the dependency graph.

## Project Overview

### Architecture
- The project follows a modular architecture with clear separation of concerns:
  - `python_to_graph.rs`: Core logic for analyzing Python imports and building the dependency graph
  - `graph.rs`: Generic graph data structure implementation
  - `graph_to_uml.rs`: Converts graph data into PlantUML diagram format
  - `module.rs`: Python module representation with package info
  - `python_utils.rs`: Python-specific parsing utilities
  - `file_utils.rs`: File system traversal functions

### Key Workflows

1. **Building the Project**
   ```bash
   cargo build
   ```

2. **Running Tests**
   ```bash
   cargo test
   ```

3. **Usage Example**
   ```bash
   cargo run
   ```
   The program takes a Python project directory as input and outputs PlantUML diagram content to stdout.

### Key Patterns

1. **Graph Construction**
   - Nodes represent Python modules
   - Edges represent import relationships
   - See `build_dependency_graph()` in `python_to_graph.rs` for the main algorithm

2. **Python Import Analysis**
   - Uses `ruff_python_parser` for AST parsing
   - Extracts both relative and absolute imports
   - Filters to only show internal project dependencies

3. **PlantUML Output Format**
   - Generates layered package diagrams
   - Modules are grouped by their package structure
   - See `sample.puml` for example output format

### Development Guidelines

1. **Adding New Features**
   - Graph-related changes go in `graph.rs`
   - Python parsing updates belong in `python_utils.rs`
   - PlantUML generation changes go in `graph_to_uml.rs`

2. **Error Handling**
   - Use `Result` and `Option` for error handling
   - File operations should handle IO errors appropriately

3. **Dependencies**
   - Key external dependencies:
     - `ruff_python_parser`: Python AST parsing
     - `petgraph`: Graph data structures
     - `walkdir`: File system traversal
     - `regex`: Import pattern matching