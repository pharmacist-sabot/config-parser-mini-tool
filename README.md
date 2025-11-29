# Config Parser Mini Tool

![Rust](https://img.shields.io/badge/rust-edition%202024-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

A lightweight, educational Rust application designed to demonstrate core language concepts through a practical configuration parsing scenario. This tool simulates reading and processing a configuration file, showcasing variable management, type safety, and string manipulation in Rust.

## 🚀 Features

This project serves as a practical example of the following Rust concepts:

- **Constants**: Usage of `const` for system-wide immutable values (e.g., default ports, timeouts).
- **Immutability by Default**: Demonstrating how Rust handles variables that don't need to change.
- **Mutable Variables**: Using `mut` for counters and state that evolves during execution.
- **Variable Shadowing**: A powerful Rust feature allowing variables to be redeclared in the same scope, useful for type transformations (e.g., parsing a string port to an integer).
- **String Parsing**: extracting key-value pairs from raw text data.
- **Error Handling**: Basic error management when parsing data types.

## 🛠️ Prerequisites

Before you begin, ensure you have the following installed:

- **Rust**: You can install Rust using `rustup`.
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

## 📦 Installation

1.  **Clone the repository**
    ```bash
    git clone https://github.com/pharmacist-sabot/config-parser-mini-tool.git
    cd config-parser-mini-tool
    ```

2.  **Build the project**
    ```bash
    cargo build
    ```

## 🏃 Usage

Run the application using `cargo`:

```bash
cargo run
```

### Expected Output

You should see output demonstrating the parsing process:

```text
--- Parsing Configuration ---
Input:
host=127.0.0.1
port=3000
max_connections=100

Initial host: localhost, port 8080
Found host: 127.0.0.1
Found port: 3000

---Final Configuration---
```

## 📂 Project Structure

The core logic resides in `src/main.rs`:

```rust
fn main() {
    // Constants for default values
    const DEFAULT_PORT: u16 = 8080;
    
    // Simulating config input
    let config_input = "host=127.0.0.1\nport=3000...";

    // Parsing logic loop
    for line in config_input.lines() {
        // ...
    }
}
```

## 🤝 Contributing

Contributions are welcome! If you'd like to improve this educational tool or add more complex parsing features (like TOML or JSON support), feel free to fork the repository and submit a Pull Request.

## 📄 License

This project is open source and available under the [MIT License](LICENSE).
