//! # SKYNET-RUST
//!
//! Ultra-lightweight AI agent framework in Rust
//!
//! ## Features
//! - Modular LLM provider architecture
//! - Built-in memory and persistence
//! - Tool execution system
//! - Heartbeat monitoring
//! - Async-first design

pub mod config;
pub mod core;
pub mod providers;
pub mod skynet;

pub use config::Config;
pub use core::agent::SkynetAgent;
pub use core::message::{Message, MessageType, Role};

/// Result type used throughout the SKYNET framework
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");