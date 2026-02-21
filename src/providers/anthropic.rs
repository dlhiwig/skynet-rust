use crate::core::message::{Message, Role};
use crate::providers::LLMProvider;
use crate::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

/// Anthropic Claude provider
pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl AnthropicProvider {
    /// Create a new Anthropic provider
    pub fn new(api_key: String) -> Result<Self> {
        let client = Client::new();
        let model = "claude-3-sonnet-20240229".to_string();
        let base_url = "https://api.anthropic.com".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
        })
    }

    /// Convert internal messages to Anthropic API format
    fn convert_messages(&self, messages: &[Message]) -> Vec<AnthropicMessage> {
        messages
            .iter()
            .filter(|msg| matches!(msg.role, Role::User | Role::Assistant))
            .map(|msg| AnthropicMessage {
                role: match msg.role {
                    Role::User => "user".to_string(),
                    Role::Assistant => "assistant".to_string(),
                    _ => "user".to_string(), // fallback
                },
                content: msg.content.clone(),
            })
            .collect()
    }

    /// Extract system messages
    fn extract_system_message(&self, messages: &[Message]) -> Option<String> {
        messages
            .iter()
            .find(|msg| matches!(msg.role, Role::System))
            .map(|msg| msg.content.clone())
    }
}

#[async_trait]
impl LLMProvider for AnthropicProvider {
    async fn generate(&self, messages: &[Message]) -> Result<String> {
        debug!("Generating response with Anthropic Claude");

        let api_messages = self.convert_messages(messages);
        let system_message = self.extract_system_message(messages);

        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 1000,
            messages: api_messages,
            system: system_message,
        };

        let response = self
            .client
            .post(&format!("{}/v1/messages", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Anthropic API error: {}", error_text);
            return Err(format!("Anthropic API error: {}", error_text).into());
        }

        let anthropic_response: AnthropicResponse = response.json().await?;
        
        if let Some(content) = anthropic_response.content.first() {
            Ok(content.text.clone())
        } else {
            Err("No content in Anthropic response".into())
        }
    }

    fn name(&self) -> &str {
        "anthropic"
    }

    fn model(&self) -> &str {
        &self.model
    }

    async fn health_check(&self) -> Result<bool> {
        // Simple health check - could be improved
        Ok(true)
    }
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
}

#[derive(Deserialize)]
struct AnthropicContent {
    text: String,
}