# Simple Web Server in Rust

A lightweight web server implemented in Rust, designed to serve static HTML content and handle basic HTTP requests. This server demonstrates the capabilities of Rust for network programming and is suitable for educational purposes or as a starting point for more complex web applications.

## Features

- Serves HTML files from the local file system.
- Handles basic GET requests, including a sleep route to demonstrate delayed responses.
- Returns a 404 error page for unrecognized routes.

## Prerequisites

- Rust programming language installed on your machine. You can install Rust by following the instructions on [rust-lang.org](https://www.rust-lang.org/tools/install).
- Cargo, the Rust package manager, which comes with the Rust installation.

## Installation

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/h471x/rust_server.git
   cd rust_server
   ```

3. **Build the Project**:
   Run the following command to build the project and download any necessary dependencies:
   ```bash
   cargo build
   ```

## Usage

1. **Run the Server**:
   Execute the following command to start the server:
   ```bash
   cargo run
   ```

2. **Access the Server**:
   Open your web browser and navigate to:
   ```
   http://127.0.0.1:7878
   ```

   You can also access the sleep route by navigating to:
   ```
   http://127.0.0.1:7878/sleep
   ```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests. For major changes, please open an issue first to discuss what you would like to change.

## Acknowledgments

- Inspired by the simplicity and performance of Rust.
- Thanks to the Rust community for their support and resources.