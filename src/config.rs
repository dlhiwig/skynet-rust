use crate::Result;
use serde::{Deserialize, Serialize};
use std::env;

/// Main configuration for SKYNET
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Anthropic API key for Claude
    pub anthropic_api_key: String,
    
    /// Default model to use
    pub default_model: String,
    
    /// Database configuration
    pub database: DatabaseConfig,
    
    /// Agent configuration
    pub agent: AgentConfig,
    
    /// Logging configuration
    pub logging: LoggingConfig,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// SQLite database path
    pub path: String,
    
    /// Maximum number of connections
    pub max_connections: u32,
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Maximum number of messages to keep in context
    pub max_context_messages: usize,
    
    /// Heartbeat interval in seconds
    pub heartbeat_interval_secs: u64,
    
    /// Maximum tokens per request
    pub max_tokens: u32,
    
    /// Temperature for LLM generation
    pub temperature: f32,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,
    
    /// Whether to log to file
    pub file_logging: bool,
    
    /// Log file path (if file_logging is true)
    pub log_file: Option<String>,
}

impl Config {
    /// Load configuration from environment and defaults
    pub async fn load() -> Result<Self> {
        // Try to load from config file first
        if let Ok(config) = Self::load_from_file("skynet.toml").await {
            return Ok(config);
        }

        // Fall back to environment variables and defaults
        Self::load_from_env()
    }

    /// Load configuration from a TOML file
    pub async fn load_from_file(path: &str) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Load configuration from environment variables
    pub fn load_from_env() -> Result<Self> {
        let anthropic_api_key = env::var("ANTHROPIC_API_KEY")
            .or_else(|_| env::var("CLAUDE_API_KEY"))
            .map_err(|_| "ANTHROPIC_API_KEY environment variable is required")?;

        Ok(Config {
            anthropic_api_key,
            default_model: env::var("SKYNET_MODEL")
                .unwrap_or_else(|_| "claude-3-sonnet-20240229".to_string()),
            database: DatabaseConfig {
                path: env::var("SKYNET_DB_PATH")
                    .unwrap_or_else(|_| "./skynet.db".to_string()),
                max_connections: env::var("SKYNET_DB_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
            },
            agent: AgentConfig {
                max_context_messages: env::var("SKYNET_MAX_CONTEXT")
                    .unwrap_or_else(|_| "50".to_string())
                    .parse()
                    .unwrap_or(50),
                heartbeat_interval_secs: env::var("SKYNET_HEARTBEAT_INTERVAL")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
                max_tokens: env::var("SKYNET_MAX_TOKENS")
                    .unwrap_or_else(|_| "1000".to_string())
                    .parse()
                    .unwrap_or(1000),
                temperature: env::var("SKYNET_TEMPERATURE")
                    .unwrap_or_else(|_| "0.7".to_string())
                    .parse()
                    .unwrap_or(0.7),
            },
            logging: LoggingConfig {
                level: env::var("SKYNET_LOG_LEVEL")
                    .unwrap_or_else(|_| "info".to_string()),
                file_logging: env::var("SKYNET_FILE_LOGGING")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()
                    .unwrap_or(false),
                log_file: env::var("SKYNET_LOG_FILE").ok(),
            },
        })
    }

    /// Save configuration to a TOML file
    pub async fn save_to_file(&self, path: &str) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }

    /// Create a default configuration file
    pub async fn create_default_config(path: &str) -> Result<()> {
        let default_config = Config {
            anthropic_api_key: "your-api-key-here".to_string(),
            default_model: "claude-3-sonnet-20240229".to_string(),
            database: DatabaseConfig {
                path: "./skynet.db".to_string(),
                max_connections: 10,
            },
            agent: AgentConfig {
                max_context_messages: 50,
                heartbeat_interval_secs: 30,
                max_tokens: 1000,
                temperature: 0.7,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file_logging: false,
                log_file: None,
            },
        };

        default_config.save_to_file(path).await?;
        Ok(())
    }
}