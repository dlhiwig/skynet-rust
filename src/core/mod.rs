//! Core SKYNET components

pub mod agent;
pub mod message;

pub use agent::SkynetAgent;
pub use message::{Message, MessageType, Role};