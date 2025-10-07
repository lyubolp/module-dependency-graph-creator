# Module Dependency Graph Creator

This tool generates a dependency graph for Python modules in a project. It analyzes the import statements in each module and creates a visual representation of the dependencies in PlantUML format.

## Usage

```bash
module-dependency-graph-creator <input_path> <output_path>
```

### Arguments

- `input_path`: Path to the root directory of your Python project
- `output_path`: Path where the PlantUML diagram will be saved (must end with .puml)

### Example

```bash
module-dependency-graph-creator ./my_python_project ./output.puml
```

This will analyze all Python files in `my_python_project` and create a dependency diagram in `output.puml`.
