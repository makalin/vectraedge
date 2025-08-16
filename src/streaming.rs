use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

use crate::config::Config;

pub struct StreamManager {
    config: Config,
    subscriptions: Arc<RwLock<HashMap<String, StreamSubscription>>>,
    producers: Arc<RwLock<HashMap<String, StreamProducer>>>,
    consumers: Arc<RwLock<HashMap<String, StreamConsumer>>>,
}

pub struct StreamSubscription {
    pub id: String,
    pub topic: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct StreamProducer {
    pub topic: String,
    pub status: String,
}

pub struct StreamConsumer {
    pub topic: String,
    pub subscription_id: String,
    pub status: String,
}

impl StreamManager {
    pub async fn new(config: &Config) -> Result<Self> {
        Ok(Self {
            config,
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            producers: Arc::new(RwLock::new(HashMap::new())),
            consumers: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    pub async fn subscribe(&self, topic: &str) -> Result<StreamSubscription> {
        let subscription_id = Uuid::new_v4().to_string();
        
        let subscription = StreamSubscription {
            id: subscription_id.clone(),
            topic: topic.to_string(),
            status: "active".to_string(),
            created_at: chrono::Utc::now(),
        };
        
        // Store subscription
        let mut subscriptions = self.subscriptions.write().await;
        subscriptions.insert(subscription_id.clone(), subscription.clone());
        
        // Create consumer for the topic
        let consumer = StreamConsumer {
            topic: topic.to_string(),
            subscription_id: subscription_id.clone(),
            status: "active".to_string(),
        };
        
        let mut consumers = self.consumers.write().await;
        consumers.insert(subscription_id, consumer);
        
        Ok(subscription)
    }
    
    pub async fn unsubscribe(&self, subscription_id: &str) -> Result<()> {
        let mut subscriptions = self.subscriptions.write().await;
        subscriptions.remove(subscription_id);
        
        let mut consumers = self.consumers.write().await;
        consumers.remove(subscription_id);
        
        Ok(())
    }
    
    pub async fn publish(&self, topic: &str, message: Value) -> Result<()> {
        // In a real implementation, this would publish to Redpanda
        // For now, we'll just log the message
        
        tracing::info!("Publishing to topic {}: {:?}", topic, message);
        
        // Store producer if it doesn't exist
        let mut producers = self.producers.write().await;
        if !producers.contains_key(topic) {
            producers.insert(topic.to_string(), StreamProducer {
                topic: topic.to_string(),
                status: "active".to_string(),
            });
        }
        
        Ok(())
    }
    
    pub async fn get_subscription(&self, subscription_id: &str) -> Result<Option<StreamSubscription>> {
        let subscriptions = self.subscriptions.read().await;
        Ok(subscriptions.get(subscription_id).cloned())
    }
    
    pub async fn list_subscriptions(&self) -> Result<Vec<StreamSubscription>> {
        let subscriptions = self.subscriptions.read().await;
        Ok(subscriptions.values().cloned().collect())
    }
    
    pub async fn get_topic_stats(&self, topic: &str) -> Result<Value> {
        let subscriptions = self.subscriptions.read().await;
        let consumers = self.consumers.read().await;
        let producers = self.producers.read().await;
        
        let topic_subscriptions: Vec<_> = subscriptions
            .values()
            .filter(|s| s.topic == topic)
            .collect();
        
        let topic_consumers: Vec<_> = consumers
            .values()
            .filter(|c| c.topic == topic)
            .collect();
        
        let has_producer = producers.contains_key(topic);
        
        Ok(serde_json::json!({
            "topic": topic,
            "subscriptions": topic_subscriptions.len(),
            "consumers": topic_consumers.len(),
            "has_producer": has_producer,
            "status": if has_producer && !topic_subscriptions.is_empty() { "active" } else { "inactive" }
        }))
    }
    
    pub async fn create_topic(&self, topic: &str, partitions: u32, replication_factor: u32) -> Result<()> {
        // In a real implementation, this would create a topic in Redpanda
        tracing::info!("Creating topic {} with {} partitions and replication factor {}", 
                      topic, partitions, replication_factor);
        
        Ok(())
    }
    
    pub async fn delete_topic(&self, topic: &str) -> Result<()> {
        // In a real implementation, this would delete a topic from Redpanda
        
        // Remove all subscriptions for this topic
        let mut subscriptions = self.subscriptions.write().await;
        subscriptions.retain(|_, s| s.topic != topic);
        
        // Remove all consumers for this topic
        let mut consumers = self.consumers.write().await;
        consumers.retain(|_, c| c.topic != topic);
        
        // Remove producer for this topic
        let mut producers = self.producers.write().await;
        producers.remove(topic);
        
        tracing::info!("Deleted topic {}", topic);
        
        Ok(())
    }
}

impl Clone for StreamSubscription {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            topic: self.topic.clone(),
            status: self.status.clone(),
            created_at: self.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_subscribe() {
        let config = Config::default();
        let manager = StreamManager::new(&config).await.unwrap();
        
        let subscription = manager.subscribe("test_topic").await.unwrap();
        assert_eq!(subscription.topic, "test_topic");
        assert_eq!(subscription.status, "active");
        
        let retrieved = manager.get_subscription(&subscription.id).await.unwrap();
        assert!(retrieved.is_some());
    }
}
