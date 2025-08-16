use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    pub data: T,
    pub created_at: Instant,
    pub accessed_at: Instant,
    pub access_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub max_entries: usize,
    pub ttl_seconds: u64,
    pub max_memory_mb: usize,
    pub eviction_policy: EvictionPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvictionPolicy {
    LRU,      // Least Recently Used
    LFU,      // Least Frequently Used
    TTL,      // Time To Live
    Random,   // Random eviction
}

pub struct Cache<T> {
    config: CacheConfig,
    entries: Arc<RwLock<HashMap<String, CacheEntry<T>>>>,
    memory_usage: Arc<RwLock<usize>>,
}

impl<T> Cache<T> {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            config,
            entries: Arc::new(RwLock::new(HashMap::new())),
            memory_usage: Arc::new(RwLock::new(0)),
        }
    }
    
    pub async fn get(&self, key: &str) -> Option<T> 
    where T: Clone
    {
        let mut entries = self.entries.write().await;
        
        if let Some(entry) = entries.get_mut(key) {
            // Check if entry has expired
            if self.is_expired(entry) {
                entries.remove(key);
                return None;
            }
            
            // Update access statistics
            entry.accessed_at = Instant::now();
            entry.access_count += 1;
            
            Some(entry.data.clone())
        } else {
            None
        }
    }
    
    pub async fn set(&self, key: String, value: T) -> Result<()> 
    where T: Clone
    {
        let mut entries = self.entries.write().await;
        
        // Check if we need to evict entries
        if entries.len() >= self.config.max_entries {
            self.evict_entries(&mut entries).await?;
        }
        
        // Check memory usage
        let estimated_size = std::mem::size_of::<T>();
        let mut memory_usage = self.memory_usage.write().await;
        
        if *memory_usage + estimated_size > self.config.max_memory_mb * 1024 * 1024 {
            self.evict_entries(&mut entries).await?;
            *memory_usage = 0; // Reset after eviction
        }
        
        let entry = CacheEntry {
            data: value.clone(),
            created_at: Instant::now(),
            accessed_at: Instant::now(),
            access_count: 1,
        };
        
        entries.insert(key, entry);
        *memory_usage += estimated_size;
        
        Ok(())
    }
    
    pub async fn remove(&self, key: &str) -> Option<T> {
        let mut entries = self.entries.write().await;
        let mut memory_usage = self.memory_usage.write().await;
        
        if let Some(entry) = entries.remove(key) {
            let estimated_size = std::mem::size_of::<T>();
            *memory_usage = memory_usage.saturating_sub(estimated_size);
            Some(entry.data)
        } else {
            None
        }
    }
    
    pub async fn clear(&self) {
        let mut entries = self.entries.write().await;
        let mut memory_usage = self.memory_usage.write().await;
        
        entries.clear();
        *memory_usage = 0;
    }
    
    pub async fn size(&self) -> usize {
        let entries = self.entries.read().await;
        entries.len()
    }
    
    pub async fn memory_usage(&self) -> usize {
        let memory_usage = self.memory_usage.read().await;
        *memory_usage
    }
    
    pub async fn keys(&self) -> Vec<String> {
        let entries = self.entries.read().await;
        entries.keys().cloned().collect()
    }
    
    pub async fn contains_key(&self, key: &str) -> bool {
        let entries = self.entries.read().await;
        entries.contains_key(key)
    }
    
    pub async fn get_stats(&self) -> CacheStats {
        let entries = self.entries.read().await;
        let memory_usage = self.memory_usage.read().await;
        
        let mut total_access_count = 0;
        let mut oldest_entry = Instant::now();
        let mut newest_entry = Instant::now();
        
        for entry in entries.values() {
            total_access_count += entry.access_count;
            if entry.created_at < oldest_entry {
                oldest_entry = entry.created_at;
            }
            if entry.created_at > newest_entry {
                newest_entry = entry.created_at;
            }
        }
        
        CacheStats {
            total_entries: entries.len(),
            memory_usage_bytes: *memory_usage,
            total_access_count,
            oldest_entry_age: oldest_entry.elapsed().as_secs(),
            newest_entry_age: newest_entry.elapsed().as_secs(),
            hit_rate: if total_access_count > 0 {
                (entries.len() as f64 / total_access_count as f64) * 100.0
            } else {
                0.0
            },
        }
    }
    
    fn is_expired(&self, entry: &CacheEntry<T>) -> bool {
        let age = entry.created_at.elapsed();
        age.as_secs() > self.config.ttl_seconds
    }
    
    async fn evict_entries(&self, entries: &mut HashMap<String, CacheEntry<T>>) -> Result<()> {
        let entries_to_remove = match self.config.eviction_policy {
            EvictionPolicy::LRU => self.get_lru_entries(entries),
            EvictionPolicy::LFU => self.get_lfu_entries(entries),
            EvictionPolicy::TTL => self.get_expired_entries(entries),
            EvictionPolicy::Random => self.get_random_entries(entries),
        };
        
        for key in entries_to_remove {
            entries.remove(&key);
        }
        
        Ok(())
    }
    
    fn get_lru_entries(&self, entries: &HashMap<String, CacheEntry<T>>) -> Vec<String> {
        let mut entries_vec: Vec<_> = entries.iter().collect();
        entries_vec.sort_by_key(|(_, entry)| entry.accessed_at);
        
        let evict_count = entries.len() / 4; // Evict 25% of entries
        entries_vec.into_iter()
            .take(evict_count)
            .map(|(key, _)| key.clone())
            .collect()
    }
    
    fn get_lfu_entries(&self, entries: &HashMap<String, CacheEntry<T>>) -> Vec<String> {
        let mut entries_vec: Vec<_> = entries.iter().collect();
        entries_vec.sort_by_key(|(_, entry)| entry.access_count);
        
        let evict_count = entries.len() / 4; // Evict 25% of entries
        entries_vec.into_iter()
            .take(evict_count)
            .map(|(key, _)| key.clone())
            .collect()
    }
    
    fn get_expired_entries(&self, entries: &HashMap<String, CacheEntry<T>>) -> Vec<String> {
        entries.iter()
            .filter(|(_, entry)| self.is_expired(entry))
            .map(|(key, _)| key.clone())
            .collect()
    }
    
    fn get_random_entries(&self, entries: &HashMap<String, CacheEntry<T>>) -> Vec<String> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        
        let mut keys: Vec<String> = entries.keys().cloned().collect();
        let mut rng = thread_rng();
        keys.shuffle(&mut rng);
        
        let evict_count = entries.len() / 4; // Evict 25% of entries
        keys.into_iter().take(evict_count).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_entries: usize,
    pub memory_usage_bytes: usize,
    pub total_access_count: u64,
    pub oldest_entry_age: u64,
    pub newest_entry_age: u64,
    pub hit_rate: f64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 1000,
            ttl_seconds: 3600, // 1 hour
            max_memory_mb: 100, // 100 MB
            eviction_policy: EvictionPolicy::LRU,
        }
    }
}

// Specialized caches for common use cases
pub struct QueryCache {
    cache: Cache<String>,
}

impl QueryCache {
    pub fn new() -> Self {
        let config = CacheConfig {
            max_entries: 500,
            ttl_seconds: 1800, // 30 minutes
            max_memory_mb: 50,
            eviction_policy: EvictionPolicy::LRU,
        };
        
        Self {
            cache: Cache::new(config),
        }
    }
    
    pub async fn get_query_result(&self, sql: &str) -> Option<String> {
        self.cache.get(sql).await
    }
    
    pub async fn cache_query_result(&self, sql: String, result: String) -> Result<()> {
        self.cache.set(sql, result).await
    }
    
    pub async fn invalidate_query(&self, sql: &str) -> Option<String> {
        self.cache.remove(sql).await
    }
}

pub struct VectorCache {
    cache: Cache<Vec<f32>>,
}

impl VectorCache {
    pub fn new() -> Self {
        let config = CacheConfig {
            max_entries: 2000,
            ttl_seconds: 7200, // 2 hours
            max_memory_mb: 200, // 200 MB for vectors
            eviction_policy: EvictionPolicy::LFU,
        };
        
        Self {
            cache: Cache::new(config),
        }
    }
    
    pub async fn get_embedding(&self, text: &str) -> Option<Vec<f32>> {
        self.cache.get(text).await
    }
    
    pub async fn cache_embedding(&self, text: String, embedding: Vec<f32>) -> Result<()> {
        self.cache.set(text, embedding).await
    }
    
    pub async fn get_stats(&self) -> CacheStats {
        self.cache.get_stats().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[tokio::test]
    async fn test_cache_basic_operations() {
        let cache = Cache::<String>::new(CacheConfig::default());
        
        // Test set and get
        cache.set("key1".to_string(), "value1".to_string()).await.unwrap();
        assert_eq!(cache.get("key1").await, Some("value1".to_string()));
        
        // Test contains_key
        assert!(cache.contains_key("key1").await);
        assert!(!cache.contains_key("key2").await);
        
        // Test size
        assert_eq!(cache.size().await, 1);
        
        // Test remove
        let removed = cache.remove("key1").await;
        assert_eq!(removed, Some("value1".to_string()));
        assert_eq!(cache.size().await, 0);
    }
    
    #[tokio::test]
    async fn test_cache_eviction() {
        let config = CacheConfig {
            max_entries: 2,
            ttl_seconds: 3600,
            max_memory_mb: 1,
            eviction_policy: EvictionPolicy::LRU,
        };
        
        let cache = Cache::<String>::new(config);
        
        // Fill cache to capacity
        cache.set("key1".to_string(), "value1".to_string()).await.unwrap();
        cache.set("key2".to_string(), "value2".to_string()).await.unwrap();
        assert_eq!(cache.size().await, 2);
        
        // Add one more to trigger eviction
        cache.set("key3".to_string(), "value3".to_string()).await.unwrap();
        
        // Should have evicted some entries
        assert!(cache.size().await <= 2);
    }
    
    #[tokio::test]
    async fn test_cache_ttl() {
        let config = CacheConfig {
            max_entries: 1000,
            ttl_seconds: 1, // 1 second TTL
            max_memory_mb: 100,
            eviction_policy: EvictionPolicy::TTL,
        };
        
        let cache = Cache::<String>::new(config);
        
        cache.set("key1".to_string(), "value1".to_string()).await.unwrap();
        assert_eq!(cache.get("key1").await, Some("value1".to_string()));
        
        // Wait for TTL to expire
        thread::sleep(Duration::from_secs(2));
        
        // Should be expired now
        assert_eq!(cache.get("key1").await, None);
    }
    
    #[tokio::test]
    async fn test_query_cache() {
        let query_cache = QueryCache::new();
        
        let sql = "SELECT * FROM users WHERE age > 18";
        let result = r#"{"rows": 5, "data": [{"id": 1, "name": "Alice"}]}"#;
        
        query_cache.cache_query_result(sql.to_string(), result.to_string()).await.unwrap();
        
        let cached_result = query_cache.get_query_result(sql).await;
        assert_eq!(cached_result, Some(result.to_string()));
    }
    
    #[tokio::test]
    async fn test_vector_cache() {
        let vector_cache = VectorCache::new();
        
        let text = "machine learning";
        let embedding = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        
        vector_cache.cache_embedding(text.to_string(), embedding.clone()).await.unwrap();
        
        let cached_embedding = vector_cache.get_embedding(text).await;
        assert_eq!(cached_embedding, Some(embedding));
    }
}
