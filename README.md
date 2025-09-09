# MongoDB Connection Test

A simple Rust application that tests MongoDB connections using the MongoDB Rust driver.

## Features

- Tests MongoDB connections using a provided URI
- Optional username and password authentication
- Clear success/failure reporting
- Command-line interface with helpful error messages

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Building

```bash
cargo build --release
```

## Usage

### Basic usage with MongoDB URI:
```bash
cargo run -- "mongodb://localhost:27017"
```

### With username and password:
```bash
cargo run -- "mongodb://localhost:27017" --username myuser --password mypass
```

### Using the compiled binary:
```bash
./target/release/mongodb-connection-test "mongodb://localhost:27017"
```

### Help:
```bash
cargo run -- --help
```

## Command Line Arguments

- `<URI>` (required): MongoDB connection URI
- `-u, --username <USERNAME>`: Username for authentication (optional)
- `-p, --password <PASSWORD>`: Password for authentication (optional)
- `-h, --help`: Show help information

## Examples

### Test local MongoDB:
```bash
cargo run -- "mongodb://localhost:27017"
```

### Test MongoDB Atlas:
```bash
cargo run -- "mongodb+srv://cluster0.example.mongodb.net/mydb" --username myuser --password mypass
```

### Test with authentication in URI:
```bash
cargo run -- "mongodb://myuser:mypass@localhost:27017/mydb"
```

## Output

The application will output:
- ✅ Connection successful! - if the connection works
- ❌ Connection failed! - if the connection fails, along with the error details

## Dependencies

- `mongodb` - MongoDB Rust driver
- `tokio` - Async runtime
- `clap` - Command line argument parsing
