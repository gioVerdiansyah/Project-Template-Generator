# Rust Template Copier

A flexible Rust tool for creating templated project structures with customizable placeholders.

## Overview

This tool allows you to copy a template directory structure and replace placeholders in the content of files. It's particularly useful for:

- Creating new projects with predefined structures
- Generating boilerplate code with custom values
- Maintaining consistent project organization

## How It Works

1. The system reads an example template from `src/templates/example`
2. It generates code to recreate the template structure
3. Users can specify placeholder replacements via command-line arguments
4. The tool rebuilds the template with customized content

## Project Structure

```
.
├── src/
│   ├── main.rs                 # Main entry point
│   ├── lib.rs                  # Library exports
│   ├── templates/              # Generated template code
│   │   ├── example/            # YOUR TEMPLATE HERE
│   └── utils/                  # Utility functions
│       ├── mod.rs
│       ├── create_file.rs      # File creation helper
│       ├── create_folder.rs    # Folder creation helper
│       ├── global_args.rs      # Global argument storage
│       └── parse_args.rs       # Argument parsing logic
└── bin/
    └── copier_template.rs      # Setup script
    └── reset.rs                # Reset files and folders
```

## Getting Started

### Prerequisites

- Rust and Cargo installed
- Make and place your template files in `src/templates/example/`

### Installation

1. Clone this repository
   ```
   git clone https://github.com/gioVerdiansyah/Project-Template-Generator.git
   ```
2. Run:
   ```
   cargo build
   ```

## Usage
### Setup

First, set up the template copier:

```
cargo run --bin copier_template
```

This will process your template files in `src/templates/example` and generate the necessary code to recreate them.

### Build
After copied template to rust file, now you can build to get executable files.
```
cargo build --release
```

### Optional
1. #### Development/Testing Only
   ```
   cargo run --bin generate_template -- --pattern='{"<the_pattern>": "<replace_pattern>"}' [optional]
   ```
   For Windows CMD:<br>
   ```
   cargo run -- --pattern="{\"<package_name>\": \"test_app\"}" [optional]
   ```
   Alternative simple format:<br>
   ```
   cargo run -- --pattern="{<package_name>: test_app}" [optional]
   ```
   That commands will be generate file and folder di root project!<br>
   If still failures, try run on powershell.
2. #### Reset and remove files and folders
   ```
   cargo run --bin reset --development [optional] --with_example [optional]
   ```

## Placeholder Pattern

- Create files in the `src/templates/example/` directory
- Use placeholders like `<package_name>` in your template files
- When running the application, specify replacement values for these placeholders


## How to Create Templates

1. Create a directory structure in `src/templates/example/`
2. Add files with placeholders like:
   `{"val_to_replace": "replace_val"}`
   Based on JSON.
4. Run `cargo run --bin generate_template` to process the template
5. Use the main application with your desired placeholder values

## Features

- Recursive directory processing
- File content templating
- Flexible placeholder formats
- Support for dotfiles (automatically handles leading dots)
- Robust error handling

## Development

To add new utility functions or modify the template processing logic:

1. Add new modules to `src/utils/`
2. Update `src/utils/mod.rs` to export them
3. Run `cargo run --bin copier_template` to regenerate the template code

## License

This project is open source and available under the [MIT License](https://github.com/gioVerdiansyah/Project-Template-Generator/blob/main/LICENSE).
