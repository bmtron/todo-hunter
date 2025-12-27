# TodoHunter

A fast, simple CLI tool to scan your Git repository for TODO comments and display them in an organized format.

## Features

- Scans all Git-tracked files for TODO items
- Supports both single-line and multi-line comments
- Custom tags for organizing TODOs by team member or category
- Fast and lightweight

## Installation

### From Source
```bash
# Clone the repository
git clone https://github.com/yourusername/todohunter.git
cd todohunter

# Build the release binary
cargo build --release

# The binary will be located at target/release/tdhunt
# Optionally, install it to your cargo bin directory
cargo install --path .
```

### Prerequisites

- Rust 1.70 or higher ([install Rust](https://rustup.rs/))
- Git

## Usage

### Basic Usage

Tag your comments with `TODO` and end them with `/td`:
```rust
// TODO: Refactor this function for better performance /td

/* TODO: This is a multi-line todo
   that spans several lines
   and needs attention /td */
```

Then run from your project directory:
```bash
tdhunt
```

### Command-Line Arguments

| Argument | Short | Description | Default |
|----------|-------|-------------|---------|
| `--path` | `-p` | Path to the Git repository | Current directory |
| `--tag` | `-t` | Custom tag suffix (e.g., `-t BM` searches for `TODO_BM`) | None |

### Examples
```bash
# Scan current directory
tdhunt

# Scan specific repository
tdhunt -p /path/to/repo

# Find TODOs with custom tag
tdhunt -t BM
# This will search for: TODO_BM: message /td
```

## Output Format

TodoHunter displays results in a clean, parseable format:
```
src/main.rs:42:Fix performance bottleneck in parser
src/utils.rs:15:Add error handling for edge cases
        This needs to handle null inputs
        and validate the range
```

## How It Works

TodoHunter scans only Git-tracked files, looking for comments that:
1. Contain the `TODO` keyword (or `TODO_<TAG>` if using custom tags)
2. End with the `/td` marker

This ensures you get relevant results without scanning build artifacts, dependencies, or untracked files.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Built with Rust** 🦀

