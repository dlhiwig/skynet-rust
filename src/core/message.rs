use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Message role in conversation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

/// Message type classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    ToolCall,
    ToolResult,
    Error,
}

/// Core message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub role: Role,
    pub content: String,
    pub message_type: MessageType,
    pub metadata: HashMap<String, serde_json::Value>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Message {
    /// Create a new text message
    pub fn new_text(role: Role, content: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            role,
            content,
            message_type: MessageType::Text,
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create a system message
    pub fn system(content: String) -> Self {
        Self::new_text(Role::System, content)
    }

    /// Create a user message
    pub fn user(content: String) -> Self {
        Self::new_text(Role::User, content)
    }

    /// Create an assistant message
    pub fn assistant(content: String) -> Self {
        Self::new_text(Role::Assistant, content)
    }

    /// Add metadata to the message
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}