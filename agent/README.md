# Logrero Agent

## Table of Contents

1. [Features](#features)
2. [Getting Started](#getting-started)
3. [Installation](#installation)
4. [Usage](#usage)
5. [Configuration](#configuration)
6. [License](#license)

## Features

- **Dynamic Configuration**: The server controls the agent's log levels,
    enabling dynamic adjustments without restarting the agent.
- **Log Filtering**: Only logs matching the configured log level are collected
    and sent to reduce bandwidth and storage use.

## Getting Started

These instructions will help you set up and run the **Logrero** agent on your
system.

### Prerequisites

Before setting up the project, ensure you have the following tools installed:

- **Rust** (Latest stable version): Install from
    [Rust](https://www.rust-lang.org/tools/install)
- **Systemd**: Log source on Linux systems.

## Installation

### Step 1: Clone the Repository

```bash
git clone https://github.com/logrero/agent.git
cd agent 
```

### Step 2: Build the Project

1. Install Rust if not already installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Build the project:

```bash
cargo build --release
```

### Step 3: Run the Logrero Agent

```bash
./target/release/logrero
```

The agent will automatically begin collecting logs from **systemd** and sending
them to the server, based on the configuration it receives from the
configuration file.

## Usage

### Running the Agent

After installation, you can start the **Logrero agent**:

```bash
./target/release/logrero
```

The agent will run in the background, collecting logs from **systemd** and
transmitting them to the central server for monitoring and storage.

## Configuration

The **Logrero agent** uses a **TOML** configuration file to define server
settings and logging behavior. You can specify the configuration file in
`config.toml`.

### Example `config.toml`

```toml
[server]
address = "logrero-server.example.com"
port = 8080
token = "your-auth-token"
```

## License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE)
file for more details.
