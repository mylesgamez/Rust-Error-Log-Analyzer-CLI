# Rust Log Analyzer CLI

This repository contains a command-line interface (CLI) tool written in Rust that can parse and analyze error log files, providing basic analytics and statistics. The tool can parse logs with messages in the format `[ERROR] ErrorType: ErrorMessage`, and it provides counts for each type of error and the five most frequently occurring error messages.

## Features

- Parses error logs with messages in the format `[ERROR] ErrorType: ErrorMessage`
- Provides a count of each type of error in the log
- Provides the five most common error messages in the log

## Usage

To use the Rust Log Analyzer CLI, follow these steps:

1. Clone this repository
2. Compile the Rust program with `cargo build`
3. Run the tool with `cargo run <path-to-log-file>`, replacing `<path-to-log-file>` with the path to the log file you want to analyze

For example:

```
git clone https://github.com/<your-github-username>/rust-log-analyzer-cli.git
cd rust-log-analyzer-cli
cargo build
cargo run logs/errors.log
```

## License
MIT
