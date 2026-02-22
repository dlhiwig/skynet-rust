use crate::{Config, Result};
use crate::core::message::Message;
use crate::providers::LLMProvider;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error};

/// Tool execution trait
#[async_trait]
pub trait Tool: Send + Sync {
    async fn execute(&self, args: serde_json::Value) -> Result<String>;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

/// Memory storage trait
#[async_trait]
pub trait Memory: Send + Sync {
    async fn store(&self, message: &Message) -> Result<()>;
    async fn retrieve(&self, limit: usize) -> Result<Vec<Message>>;
    async fn search(&self, query: &str) -> Result<Vec<Message>>;
}

/// Main SKYNET agent implementation
pub struct SkynetAgent {
    config: Config,
    provider: Box<dyn LLMProvider>,
    tools: Vec<Arc<dyn Tool>>,
    memory: Arc<dyn Memory>,
    conversation: Arc<RwLock<Vec<Message>>>,
    running: Arc<RwLock<bool>>,
}

impl SkynetAgent {
    /// Create a new SKYNET agent
    pub async fn new(config: Config) -> Result<Self> {
        info!("Initializing SKYNET agent...");

        // Initialize provider (TODO: make configurable)
        let provider = crate::providers::anthropic::AnthropicProvider::new(
            config.anthropic_api_key.clone()
        )?;

        // Initialize memory (TODO: implement proper memory storage)
        let memory = Arc::new(InMemoryStorage::new());

        // Initialize tools (empty for now)
        let tools = Vec::new();

        Ok(Self {
            config,
            provider: Box::new(provider),
            tools,
            memory,
            conversation: Arc::new(RwLock::new(Vec::new())),
            running: Arc::new(RwLock::new(false)),
        })
    }

    /// Start the main agent loop
    pub async fn run(&mut self) -> Result<()> {
        info!("🚀 Starting SKYNET agent loop");
        
        // Set running state
        {
            let mut running = self.running.write().await;
            *running = true;
        }

        // Main agent loop
        loop {
            // Check if we should continue running
            {
                let running = self.running.read().await;
                if !*running {
                    break;
                }
            }

            // Agent loop steps:
            match self.process_cycle().await {
                Ok(_) => debug!("Agent cycle completed successfully"),
                Err(e) => {
                    error!("Agent cycle error: {}", e);
                    // Continue running unless it's a fatal error
                }
            }

            // Sleep between cycles to prevent busy waiting
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        info!("SKYNET agent loop terminated");
        Ok(())
    }

    /// Process a single agent cycle
    async fn process_cycle(&self) -> Result<()> {
        // 1. Receive message (placeholder - would come from queue/channel)
        // For now, we'll just process a test message
        let input_message = Message::user("Hello, SKYNET!".to_string());

        // 2. Load context from memory
        let context = self.memory.retrieve(10).await?;
        debug!("Loaded {} messages from context", context.len());

        // 3. Build conversation for LLM
        let mut conversation = context;
        conversation.push(input_message.clone());

        // Store input in memory
        self.memory.store(&input_message).await?;

        // 4. Call LLM provider
        let response = self.provider.generate(&conversation).await?;
        debug!("LLM response: {}", response);

        // 5. Execute any tools (placeholder)
        // TODO: Parse tool calls from LLM response and execute them

        // 6. Store response in memory
        let response_message = Message::assistant(response);
        self.memory.store(&response_message).await?;

        // 7. Send response (placeholder - would send to output channel)
        info!("Agent response: {}", response_message.content);

        Ok(())
    }

    /// Gracefully stop the agent
    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
        info!("SKYNET agent stop requested");
    }
}

/// Simple in-memory storage implementation for testing
struct InMemoryStorage {
    messages: Arc<RwLock<Vec<Message>>>,
}

impl InMemoryStorage {
    fn new() -> Self {
        Self {
            messages: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[async_trait]
impl Memory for InMemoryStorage {
    async fn store(&self, message: &Message) -> Result<()> {
        let mut messages = self.messages.write().await;
        messages.push(message.clone());
        debug!("Stored message: {}", message.id);
        Ok(())
    }

    async fn retrieve(&self, limit: usize) -> Result<Vec<Message>> {
        let messages = self.messages.read().await;
        let start = if messages.len() > limit {
            messages.len() - limit
        } else {
            0
        };
        Ok(messages[start..].to_vec())
    }

    async fn search(&self, query: &str) -> Result<Vec<Message>> {
        let messages = self.messages.read().await;
        let results: Vec<Message> = messages
            .iter()
            .filter(|msg| msg.content.to_lowercase().contains(&query.to_lowercase()))
            .cloned()
            .collect();
        Ok(results)
    }
}