# SKYNET-RUST 🤖

> Ultra-lightweight AI agent framework in Rust

SKYNET-RUST is a high-performance, modular AI agent framework built in Rust. Designed for speed, safety, and scalability, it provides a robust foundation for building autonomous AI agents with minimal overhead.

## ✨ Features

- **🚀 Ultra-fast**: Built in Rust for maximum performance
- **🔧 Modular**: Pluggable LLM providers, tools, and memory systems
- **💾 Persistent**: SQLite-based memory with search capabilities
- **🫀 Monitored**: Built-in heartbeat and health monitoring
- **🌐 Async**: Full async/await support with Tokio
- **🔒 Safe**: Memory-safe with Rust's ownership system
- **📦 Lightweight**: Minimal dependencies and resource usage

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   SkynetAgent   │────│   LLMProvider    │────│   Anthropic     │
│                 │    │   (trait)        │    │   OpenAI        │
│  - Message Loop │    │                  │    │   Local LLMs    │
│  - Tool Exec    │    │  - generate()    │    │   ...           │
│  - Memory Mgmt  │    │  - health_check()│    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                       │
         │              ┌─────────────────┐    ┌─────────────────┐
         │──────────────│     Tools       │────│   File System   │
         │              │   (trait)       │    │   Web Requests  │
         │              │                 │    │   System Calls  │
         │              │  - execute()    │    │   ...           │
         │              │  - name()       │    │                 │
         │              │  - description()│    │                 │
         │              └─────────────────┘    └─────────────────┘
         │
         │              ┌─────────────────┐    ┌─────────────────┐
         └──────────────│     Memory      │────│   SQLite DB     │
                        │   (trait)       │    │   Search Index  │
                        │                 │    │   Context Mgmt  │
                        │  - store()      │    │                 │
                        │  - retrieve()   │    │                 │
                        │  - search()     │    │                 │
                        └─────────────────┘    └─────────────────┘
```

### Core Components

- **SkynetAgent**: Main agent orchestrator that runs the message processing loop
- **LLMProvider**: Abstraction for different AI providers (Claude, GPT, local models)
- **Tools**: Executable functions that the agent can call
- **Memory**: Persistent storage and retrieval system
- **Pulse**: Heartbeat monitoring for health checks

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Git

### Installation

```bash
# Clone the repository
git clone https://github.com/dlhiwig/skynet-rust.git
cd skynet-rust

# Build the project
cargo build --release

# Run tests
cargo test

# Set up environment variables
export ANTHROPIC_API_KEY="your-anthropic-api-key"

# Run SKYNET
cargo run
```

### Configuration

SKYNET can be configured via:

1. **Environment variables** (recommended for API keys)
2. **Configuration file** (`skynet.toml`)

#### Environment Variables

```bash
# Required
export ANTHROPIC_API_KEY="sk-ant-api03-..."

# Optional
export SKYNET_MODEL="claude-3-sonnet-20240229"
export SKYNET_DB_PATH="./skynet.db"
export SKYNET_LOG_LEVEL="info"
export SKYNET_MAX_TOKENS="1000"
```

#### Configuration File

Generate a default config:

```bash
cargo run -- --create-config skynet.toml
```

Example `skynet.toml`:

```toml
anthropic_api_key = "sk-ant-api03-..."
default_model = "claude-3-sonnet-20240229"

[database]
path = "./skynet.db"
max_connections = 10

[agent]
max_context_messages = 50
heartbeat_interval_secs = 30
max_tokens = 1000
temperature = 0.7

[logging]
level = "info"
file_logging = false
log_file = "skynet.log"
```

## 🛠️ Usage as Library

Add to your `Cargo.toml`:

```toml
[dependencies]
skynet-rust = { git = "https://github.com/dlhiwig/skynet-rust" }
tokio = { version = "1", features = ["full"] }
```

Basic usage:

```rust
use skynet_rust::{Config, SkynetAgent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::load().await?;
    
    // Create and start agent
    let mut agent = SkynetAgent::new(config).await?;
    agent.run().await?;
    
    Ok(())
}
```

## 🔧 Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run with logs
RUST_LOG=debug cargo run
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for security issues
cargo audit
```

## 🤝 Contributing

We welcome contributions! Here's how to get started:

### Development Setup

1. **Fork the repository**
2. **Clone your fork:**
   ```bash
   git clone https://github.com/yourusername/skynet-rust.git
   cd skynet-rust
   ```

3. **Set up development environment:**
   ```bash
   # Install Rust if needed
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install development tools
   cargo install cargo-edit cargo-audit
   
   # Run tests to ensure everything works
   cargo test
   ```

### Making Changes

1. **Create a feature branch:**
   ```bash
   git checkout -b feature/amazing-feature
   ```

2. **Make your changes**
3. **Test thoroughly:**
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

4. **Commit with conventional commits:**
   ```bash
   git commit -m "feat: add amazing feature"
   ```

5. **Push and create a Pull Request**

### Contribution Guidelines

- **Code Style**: Follow `rustfmt` defaults
- **Testing**: Add tests for new features
- **Documentation**: Update docs for API changes
- **Commits**: Use [Conventional Commits](https://conventionalcommits.org/)
- **Performance**: Profile performance-critical changes

### Areas for Contribution

- 🧠 **New LLM Providers**: OpenAI, local models (Ollama), etc.
- 🛠️ **Tools**: File system, web scraping, API integrations
- 💾 **Memory Systems**: Vector databases, cloud storage
- 🔍 **Monitoring**: Metrics, logging, observability
- 📚 **Documentation**: Examples, tutorials, guides
- ⚡ **Performance**: Optimizations, benchmarks

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- **Repository**: https://github.com/dlhiwig/skynet-rust
- **Issues**: https://github.com/dlhiwig/skynet-rust/issues
- **Discussions**: https://github.com/dlhiwig/skynet-rust/discussions

## 📞 Support

- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/dlhiwig/skynet-rust/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/dlhiwig/skynet-rust/discussions)
- 📧 **Email**: daniel.heiwig@outlook.com

---

Built with ❤️ by [Black Eagle Project](https://blackeagleproject.org)