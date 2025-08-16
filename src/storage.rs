use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

use crate::config::Config;

pub struct StorageManager {
    config: Config,
    rocksdb: Option<Arc<rocksdb::DB>>,
    sled_db: Option<Arc<sled::Db>>,
    tables: Arc<RwLock<HashMap<String, TableMetadata>>>,
}

pub struct TableMetadata {
    pub name: String,
    pub schema: String,
    pub row_count: u64,
    pub size_bytes: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_modified: chrono::DateTime<chrono::Utc>,
}

impl StorageManager {
    pub async fn new(config: &Config) -> Result<Self> {
        let mut manager = Self {
            config: config.clone(),
            rocksdb: None,
            sled_db: None,
            tables: Arc::new(RwLock::new(HashMap::new())),
        };
        
        // Initialize storage backends
        manager.initialize_storage().await?;
        
        Ok(manager)
    }
    
    async fn initialize_storage(&mut self) -> Result<()> {
        // Initialize RocksDB if configured
        if let Some(rocksdb_path) = &self.config.storage.rocksdb_path {
            let db = rocksdb::DB::open_default(rocksdb_path)?;
            self.rocksdb = Some(Arc::new(db));
            tracing::info!("RocksDB initialized at {}", rocksdb_path);
        }
        
        // Initialize Sled if configured
        if let Some(sled_path) = &self.config.storage.sled_path {
            let db = sled::open(sled_path)?;
            self.sled_db = Some(Arc::new(db));
            tracing::info!("Sled initialized at {}", sled_path);
        }
        
        Ok(())
    }
    
    pub async fn create_table(&self, name: &str, schema: &str) -> Result<()> {
        let mut tables = self.tables.write().await;
        
        let metadata = TableMetadata {
            name: name.to_string(),
            schema: schema.to_string(),
            row_count: 0,
            size_bytes: 0,
            created_at: chrono::Utc::now(),
            last_modified: chrono::Utc::now(),
        };
        
        tables.insert(name.to_string(), metadata);
        
        // Store table metadata in persistent storage
        if let Some(rocksdb) = &self.rocksdb {
            let key = format!("table:{}", name);
            let value = serde_json::to_string(&metadata)?;
            rocksdb.put(key.as_bytes(), value.as_bytes())?;
        }
        
        tracing::info!("Created table: {}", name);
        Ok(())
    }
    
    pub async fn drop_table(&self, name: &str) -> Result<()> {
        let mut tables = self.tables.write().await;
        tables.remove(name);
        
        // Remove from persistent storage
        if let Some(rocksdb) = &self.rocksdb {
            let key = format!("table:{}", name);
            rocksdb.delete(key.as_bytes())?;
        }
        
        tracing::info!("Dropped table: {}", name);
        Ok(())
    }
    
    pub async fn insert_data(&self, table_name: &str, key: &str, value: &Value) -> Result<()> {
        // Store data in RocksDB
        if let Some(rocksdb) = &self.rocksdb {
            let full_key = format!("{}:{}", table_name, key);
            let value_bytes = serde_json::to_string(value)?.into_bytes();
            rocksdb.put(full_key.as_bytes(), &value_bytes)?;
        }
        
        // Store data in Sled for fast access
        if let Some(sled_db) = &self.sled_db {
            let tree = sled_db.open_tree(table_name)?;
            let key_bytes = key.as_bytes();
            let value_bytes = serde_json::to_string(value)?.into_bytes();
            tree.insert(key_bytes, value_bytes)?;
        }
        
        // Update table metadata
        self.update_table_stats(table_name, 1, value.to_string().len() as u64).await?;
        
        Ok(())
    }
    
    pub async fn get_data(&self, table_name: &str, key: &str) -> Result<Option<Value>> {
        // Try Sled first (faster)
        if let Some(sled_db) = &self.sled_db {
            if let Ok(tree) = sled_db.open_tree(table_name) {
                if let Ok(Some(value_bytes)) = tree.get(key.as_bytes()) {
                    if let Ok(value) = serde_json::from_slice::<Value>(&value_bytes) {
                        return Ok(Some(value));
                    }
                }
            }
        }
        
        // Fallback to RocksDB
        if let Some(rocksdb) = &self.rocksdb {
            let full_key = format!("{}:{}", table_name, key);
            if let Ok(Some(value_bytes)) = rocksdb.get(full_key.as_bytes()) {
                if let Ok(value) = serde_json::from_slice::<Value>(&value_bytes) {
                    return Ok(Some(value));
                }
            }
        }
        
        Ok(None)
    }
    
    pub async fn delete_data(&self, table_name: &str, key: &str) -> Result<()> {
        // Remove from RocksDB
        if let Some(rocksdb) = &self.rocksdb {
            let full_key = format!("{}:{}", table_name, key);
            rocksdb.delete(full_key.as_bytes())?;
        }
        
        // Remove from Sled
        if let Some(sled_db) = &self.sled_db {
            if let Ok(tree) = sled_db.open_tree(table_name) {
                tree.remove(key.as_bytes())?;
            }
        }
        
        // Update table metadata
        self.update_table_stats(table_name, -1, 0).await?;
        
        Ok(())
    }
    
    pub async fn list_tables(&self) -> Result<Vec<TableMetadata>> {
        let tables = self.tables.read().await;
        Ok(tables.values().cloned().collect())
    }
    
    pub async fn get_table_info(&self, name: &str) -> Result<Option<TableMetadata>> {
        let tables = self.tables.read().await;
        Ok(tables.get(name).cloned())
    }
    
    async fn update_table_stats(&self, table_name: &str, row_delta: i64, size_delta: u64) -> Result<()> {
        let mut tables = self.tables.write().await;
        
        if let Some(table) = tables.get_mut(table_name) {
            if row_delta > 0 {
                table.row_count += row_delta as u64;
            } else {
                table.row_count = table.row_count.saturating_sub((-row_delta) as u64);
            }
            
            table.size_bytes = table.size_bytes.saturating_add(size_delta);
            table.last_modified = chrono::Utc::now();
        }
        
        Ok(())
    }
    
    pub async fn get_storage_stats(&self) -> Result<Value> {
        let tables = self.tables.read().await;
        let total_tables = tables.len();
        let total_rows: u64 = tables.values().map(|t| t.row_count).sum();
        let total_size: u64 = tables.values().map(|t| t.size_bytes).sum();
        
        Ok(serde_json::json!({
            "total_tables": total_tables,
            "total_rows": total_rows,
            "total_size_bytes": total_size,
            "rocksdb_available": self.rocksdb.is_some(),
            "sled_available": self.sled_db.is_some()
        }))
    }
    
    pub async fn compact_storage(&self) -> Result<()> {
        // Compact RocksDB
        if let Some(rocksdb) = &self.rocksdb {
            rocksdb.compact_range(None::<&[u8]>, None::<&[u8]>)?;
            tracing::info!("RocksDB compaction completed");
        }
        
        // Compact Sled
        if let Some(sled_db) = &self.sled_db {
            sled_db.flush()?;
            tracing::info!("Sled flush completed");
        }
        
        Ok(())
    }
}

impl Clone for TableMetadata {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            schema: self.schema.clone(),
            row_count: self.row_count,
            size_bytes: self.size_bytes,
            created_at: self.created_at,
            last_modified: self.last_modified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_create_table() {
        let mut config = Config::default();
        let temp_dir = tempdir().unwrap();
        config.storage.rocksdb_path = Some(temp_dir.path().join("rocksdb").to_string_lossy().to_string());
        
        let manager = StorageManager::new(&config).await.unwrap();
        
        manager.create_table("test_table", "id INT, name TEXT").await.unwrap();
        
        let tables = manager.list_tables().await.unwrap();
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].name, "test_table");
    }
}
