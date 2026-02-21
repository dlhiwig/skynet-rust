use crate::core::message::Message;
use crate::Result;
use async_trait::async_trait;

/// LLM Provider trait for different AI services
#[async_trait]
pub trait LLMProvider: Send + Sync {
    /// Generate a response from the conversation history
    async fn generate(&self, messages: &[Message]) -> Result<String>;
    
    /// Get the provider name
    fn name(&self) -> &str;
    
    /// Get the model being used
    fn model(&self) -> &str;
    
    /// Check if the provider is healthy/available
    async fn health_check(&self) -> Result<bool>;
}

/// Configuration for LLM providers
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub api_key: String,
    pub model: String,
    pub base_url: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}