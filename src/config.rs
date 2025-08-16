use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub storage: StorageConfig,
    pub vector_search: VectorSearchConfig,
    pub streaming: StreamingConfig,
    pub ai: AIConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub max_connections: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub rocksdb_path: Option<String>,
    pub sled_path: Option<String>,
    pub data_dir: String,
    pub max_memory_mb: usize,
    pub compression: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorSearchConfig {
    pub m: usize,
    pub ef_construction: usize,
    pub ef: usize,
    pub dimension: usize,
    pub distance_metric: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingConfig {
    pub redpanda_brokers: Vec<String>,
    pub kafka_compatibility: bool,
    pub max_message_size: usize,
    pub retention_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub ollama_url: String,
    pub embedding_model: String,
    pub text_model: String,
    pub max_tokens: usize,
    pub temperature: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub output: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            storage: StorageConfig::default(),
            vector_search: VectorSearchConfig::default(),
            streaming: StreamingConfig::default(),
            ai: AIConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            workers: num_cpus::get(),
            max_connections: 1000,
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            rocksdb_path: Some("./data/rocksdb".to_string()),
            sled_path: Some("./data/sled".to_string()),
            data_dir: "./data".to_string(),
            max_memory_mb: 1024,
            compression: true,
        }
    }
}

impl Default for VectorSearchConfig {
    fn default() -> Self {
        Self {
            m: 16,
            ef_construction: 200,
            ef: 50,
            dimension: 384,
            distance_metric: "cosine".to_string(),
        }
    }
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            redpanda_brokers: vec!["localhost:9092".to_string()],
            kafka_compatibility: true,
            max_message_size: 1024 * 1024, // 1MB
            retention_ms: 7 * 24 * 60 * 60 * 1000, // 7 days
        }
    }
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            ollama_url: "http://localhost:11434".to_string(),
            embedding_model: "text-embedding-ada-002".to_string(),
            text_model: "llama2".to_string(),
            max_tokens: 2048,
            temperature: 0.7,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
            output: "stdout".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // Try to load from environment variables first
        let mut config = Self::from_env()?;
        
        // Try to load from config file
        if let Ok(file_config) = Self::from_file() {
            config = config.merge(file_config);
        }
        
        // Validate configuration
        config.validate()?;
        
        Ok(config)
    }
    
    pub fn from_env() -> Result<Self> {
        let mut config = Config::default();
        
        // Server config
        if let Ok(host) = env::var("VECTRA_HOST") {
            config.server.host = host;
        }
        if let Ok(port) = env::var("VECTRA_PORT") {
            config.server.port = port.parse()?;
        }
        if let Ok(workers) = env::var("VECTRA_WORKERS") {
            config.server.workers = workers.parse()?;
        }
        
        // Storage config
        if let Ok(data_dir) = env::var("VECTRA_DATA_DIR") {
            config.storage.data_dir = data_dir;
        }
        if let Ok(rocksdb_path) = env::var("VECTRA_ROCKSDB_PATH") {
            config.storage.rocksdb_path = Some(rocksdb_path);
        }
        if let Ok(sled_path) = env::var("VECTRA_SLED_PATH") {
            config.storage.sled_path = Some(sled_path);
        }
        
        // Vector search config
        if let Ok(dimension) = env::var("VECTRA_VECTOR_DIMENSION") {
            config.vector_search.dimension = dimension.parse()?;
        }
        if let Ok(m) = env::var("VECTRA_HNSW_M") {
            config.vector_search.m = m.parse()?;
        }
        
        // Streaming config
        if let Ok(brokers) = env::var("VECTRA_REDPANDA_BROKERS") {
            config.streaming.redpanda_brokers = brokers.split(',').map(|s| s.trim().to_string()).collect();
        }
        
        // AI config
        if let Ok(ollama_url) = env::var("VECTRA_OLLAMA_URL") {
            config.ai.ollama_url = ollama_url;
        }
        if let Ok(embedding_model) = env::var("VECTRA_EMBEDDING_MODEL") {
            config.ai.embedding_model = embedding_model;
        }
        
        // Logging config
        if let Ok(level) = env::var("VECTRA_LOG_LEVEL") {
            config.logging.level = level;
        }
        
        Ok(config)
    }
    
    pub fn from_file() -> Result<Self> {
        let config_paths = vec![
            "./config/vectra.toml",
            "./config/vectra.yaml",
            "./config/vectra.json",
            "./vectra.toml",
            "./vectra.yaml",
            "./vectra.json",
        ];
        
        for path in config_paths {
            if let Ok(config) = Self::load_from_file(path) {
                return Ok(config);
            }
        }
        
        Err(anyhow::anyhow!("No configuration file found"))
    }
    
    fn load_from_file(path: &str) -> Result<Self> {
        let path_buf = PathBuf::from(path);
        let extension = path_buf.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("toml");
        
        let content = std::fs::read_to_string(path)?;
        
        match extension {
            "toml" => Ok(toml::from_str(&content)?),
            "yaml" | "yml" => Ok(serde_yaml::from_str(&content)?),
            "json" => Ok(serde_json::from_str(&content)?),
            _ => Err(anyhow::anyhow!("Unsupported config file format: {}", extension)),
        }
    }
    
    pub fn merge(self, other: Self) -> Self {
        // This is a simple merge strategy - in production you might want more sophisticated merging
        Self {
            server: other.server,
            storage: other.storage,
            vector_search: other.vector_search,
            streaming: other.streaming,
            ai: other.ai,
            logging: other.logging,
        }
    }
    
    pub fn validate(&self) -> Result<()> {
        // Validate server config
        if self.server.port == 0 {
            return Err(anyhow::anyhow!("Invalid port number"));
        }
        
        // Validate storage config
        if let Some(ref path) = self.storage.rocksdb_path {
            let path_buf = PathBuf::from(path);
            if !path_buf.is_absolute() {
                // Convert to absolute path
                let absolute_path = std::env::current_dir()?.join(path_buf);
                // Ensure parent directory exists
                if let Some(parent) = absolute_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
            }
        }
        
        // Validate vector search config
        if self.vector_search.dimension == 0 {
            return Err(anyhow::anyhow!("Vector dimension must be greater than 0"));
        }
        
        // Validate AI config
        if self.ai.temperature < 0.0 || self.ai.temperature > 2.0 {
            return Err(anyhow::anyhow!("Temperature must be between 0.0 and 2.0"));
        }
        
        Ok(())
    }
    
    pub fn to_toml(&self) -> Result<String> {
        Ok(toml::to_string_pretty(self)?)
    }
    
    pub fn to_yaml(&self) -> Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }
    
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.vector_search.dimension, 384);
        assert_eq!(config.ai.temperature, 0.7);
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        config.server.port = 0;
        
        assert!(config.validate().is_err());
    }
}
