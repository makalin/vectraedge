use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

use crate::config::Config;

pub struct AIRuntime {
    config: Config,
    models: Arc<RwLock<HashMap<String, AIModel>>>,
    embedding_cache: Arc<RwLock<HashMap<String, Vec<f32>>>>,
}

pub struct AIModel {
    pub name: String,
    pub model_type: ModelType,
    pub status: String,
    pub parameters: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum ModelType {
    Embedding,
    TextGeneration,
    Classification,
    Custom,
}

impl AIRuntime {
    pub async fn new(config: &Config) -> Result<Self> {
        let mut runtime = Self {
            config,
            models: Arc::new(RwLock::new(HashMap::new())),
            embedding_cache: Arc::new(RwLock::new(HashMap::new())),
        };
        
        // Initialize default models
        runtime.initialize_default_models().await?;
        
        Ok(runtime)
    }
    
    async fn initialize_default_models(&mut self) -> Result<()> {
        let mut models = self.models.write().await;
        
        // Add default embedding model
        models.insert("text-embedding-ada-002".to_string(), AIModel {
            name: "text-embedding-ada-002".to_string(),
            model_type: ModelType::Embedding,
            status: "active".to_string(),
            parameters: HashMap::new(),
        });
        
        // Add default text generation model
        models.insert("llama2".to_string(), AIModel {
            name: "llama2".to_string(),
            model_type: ModelType::TextGeneration,
            status: "active".to_string(),
            parameters: HashMap::new(),
        });
        
        Ok(())
    }
    
    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        // Check cache first
        let cache_key = format!("embedding:{}", text);
        let cache = self.embedding_cache.read().await;
        if let Some(cached_embedding) = cache.get(&cache_key) {
            return Ok(cached_embedding.clone());
        }
        drop(cache);
        
        // Generate new embedding
        let embedding = self.generate_embedding_internal(text).await?;
        
        // Cache the result
        let mut cache = self.embedding_cache.write().await;
        cache.insert(cache_key, embedding.clone());
        
        Ok(embedding)
    }
    
    async fn generate_embedding_internal(&self, text: &str) -> Result<Vec<f32>> {
        // In a real implementation, this would call Ollama or ONNX Runtime
        // For now, we'll generate a mock embedding
        
        let mut embedding = Vec::with_capacity(384);
        let text_bytes = text.as_bytes();
        
        // Generate deterministic "embedding" based on text content
        for (i, &byte) in text_bytes.iter().enumerate() {
            let value = (byte as f32 + i as f32) / 255.0;
            embedding.push(value);
        }
        
        // Pad to 384 dimensions
        while embedding.len() < 384 {
            embedding.push(0.0);
        }
        
        // Normalize
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for value in &mut embedding {
                *value /= magnitude;
            }
        }
        
        Ok(embedding)
    }
    
    pub async fn generate_text(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        // In a real implementation, this would call Ollama
        // For now, return a mock response
        
        let response = format!("Generated response to: '{}' (max tokens: {})", prompt, max_tokens);
        Ok(response)
    }
    
    pub async fn classify_text(&self, text: &str, categories: &[String]) -> Result<HashMap<String, f32>> {
        // In a real implementation, this would use a classification model
        // For now, return mock probabilities
        
        let mut results = HashMap::new();
        for category in categories {
            // Generate mock probability based on text content
            let hash = text.len() as u64 + category.len() as u64;
            let probability = (hash % 100) as f32 / 100.0;
            results.insert(category.clone(), probability);
        }
        
        // Normalize probabilities
        let total: f32 = results.values().sum();
        if total > 0.0 {
            for value in results.values_mut() {
                *value /= total;
            }
        }
        
        Ok(results)
    }
    
    pub async fn add_model(&self, name: &str, model_type: ModelType, parameters: HashMap<String, Value>) -> Result<()> {
        let mut models = self.models.write().await;
        
        let model = AIModel {
            name: name.to_string(),
            model_type,
            status: "active".to_string(),
            parameters,
        };
        
        models.insert(name.to_string(), model);
        
        Ok(())
    }
    
    pub async fn remove_model(&self, name: &str) -> Result<()> {
        let mut models = self.models.write().await;
        models.remove(name);
        Ok(())
    }
    
    pub async fn list_models(&self) -> Result<Vec<AIModel>> {
        let models = self.models.read().await;
        Ok(models.values().cloned().collect())
    }
    
    pub async fn get_model(&self, name: &str) -> Result<Option<AIModel>> {
        let models = self.models.read().await;
        Ok(models.get(name).cloned())
    }
    
    pub async fn clear_embedding_cache(&self) -> Result<()> {
        let mut cache = self.embedding_cache.write().await;
        cache.clear();
        Ok(())
    }
    
    pub async fn get_cache_stats(&self) -> Result<Value> {
        let cache = self.embedding_cache.read().await;
        Ok(serde_json::json!({
            "cached_embeddings": cache.len(),
            "total_models": {
                "embedding": 1,
                "text_generation": 1,
                "classification": 0
            }
        }))
    }
}

impl Clone for AIModel {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            model_type: self.model_type.clone(),
            status: self.status.clone(),
            parameters: self.parameters.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_generate_embedding() {
        let config = Config::default();
        let runtime = AIRuntime::new(&config).await.unwrap();
        
        let embedding = runtime.generate_embedding("hello world").await.unwrap();
        assert_eq!(embedding.len(), 384);
        
        // Test caching
        let cached_embedding = runtime.generate_embedding("hello world").await.unwrap();
        assert_eq!(embedding, cached_embedding);
    }
    
    #[tokio::test]
    async fn test_classify_text() {
        let config = Config::default();
        let runtime = AIRuntime::new(&config).await.unwrap();
        
        let categories = vec!["positive".to_string(), "negative".to_string()];
        let results = runtime.classify_text("I love this!", &categories).await.unwrap();
        
        assert_eq!(results.len(), 2);
        let total: f32 = results.values().sum();
        assert!((total - 1.0).abs() < 0.001); // Probabilities should sum to 1
    }
}
