use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub labels: HashMap<String, String>,
    pub metric_type: MetricType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramBucket {
    pub le: f64,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramMetric {
    pub name: String,
    pub buckets: Vec<HistogramBucket>,
    pub sum: f64,
    pub count: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct MetricsCollector {
    metrics: Arc<RwLock<HashMap<String, Metric>>>,
    histograms: Arc<RwLock<HashMap<String, HistogramMetric>>>,
    start_time: Instant,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            histograms: Arc::new(RwLock::new(HashMap::new())),
            start_time: Instant::now(),
        }
    }
    
    pub async fn increment_counter(&self, name: &str, labels: Option<HashMap<String, String>>) {
        let key = self.metric_key(name, labels.as_ref());
        let mut metrics = self.metrics.write().await;
        
        if let Some(metric) = metrics.get_mut(&key) {
            metric.value += 1.0;
            metric.timestamp = chrono::Utc::now();
        } else {
            let metric = Metric {
                name: name.to_string(),
                value: 1.0,
                timestamp: chrono::Utc::now(),
                labels: labels.unwrap_or_default(),
                metric_type: MetricType::Counter,
            };
            metrics.insert(key, metric);
        }
    }
    
    pub async fn set_gauge(&self, name: &str, value: f64, labels: Option<HashMap<String, String>>) {
        let key = self.metric_key(name, labels.as_ref());
        let mut metrics = self.metrics.write().await;
        
        let metric = Metric {
            name: name.to_string(),
            value,
            timestamp: chrono::Utc::now(),
            labels: labels.unwrap_or_default(),
            metric_type: MetricType::Gauge,
        };
        metrics.insert(key, metric);
    }
    
    pub async fn observe_histogram(&self, name: &str, value: f64, labels: Option<HashMap<String, String>>) {
        let key = self.metric_key(name, labels.as_ref());
        let mut histograms = self.histograms.write().await;
        
        if let Some(histogram) = histograms.get_mut(&key) {
            histogram.sum += value;
            histogram.count += 1;
            histogram.timestamp = chrono::Utc::now();
            
            // Update buckets
            for bucket in &mut histogram.buckets {
                if value <= bucket.le {
                    bucket.count += 1;
                }
            }
        } else {
            // Create new histogram with default buckets
            let buckets = vec![
                HistogramBucket { le: 0.1, count: 0 },
                HistogramBucket { le: 0.5, count: 0 },
                HistogramBucket { le: 1.0, count: 0 },
                HistogramBucket { le: 2.5, count: 0 },
                HistogramBucket { le: 5.0, count: 0 },
                HistogramBucket { le: 10.0, count: 0 },
                HistogramBucket { le: f64::INFINITY, count: 0 },
            ];
            
            let mut histogram = HistogramMetric {
                name: name.to_string(),
                buckets,
                sum: value,
                count: 1,
                timestamp: chrono::Utc::now(),
            };
            
            // Update buckets for initial value
            for bucket in &mut histogram.buckets {
                if value <= bucket.le {
                    bucket.count = 1;
                }
            }
            
            histograms.insert(key, histogram);
        }
    }
    
    pub async fn record_query_duration(&self, query_type: &str, duration: Duration) {
        let labels = {
            let mut map = HashMap::new();
            map.insert("query_type".to_string(), query_type.to_string());
            map
        };
        
        self.observe_histogram("query_duration_seconds", duration.as_secs_f64(), Some(labels)).await;
    }
    
    pub async fn record_vector_search_duration(&self, dimension: usize, duration: Duration) {
        let labels = {
            let mut map = HashMap::new();
            map.insert("dimension".to_string(), dimension.to_string());
            map
        };
        
        self.observe_histogram("vector_search_duration_seconds", duration.as_secs_f64(), Some(labels)).await;
    }
    
    pub async fn record_storage_operation(&self, operation: &str, table: &str, duration: Duration) {
        let labels = {
            let mut map = HashMap::new();
            map.insert("operation".to_string(), operation.to_string());
            map.insert("table".to_string(), table.to_string());
            map
        };
        
        self.observe_histogram("storage_operation_duration_seconds", duration.as_secs_f64(), Some(labels)).await;
    }
    
    pub async fn record_ai_operation(&self, operation: &str, model: &str, duration: Duration) {
        let labels = {
            let mut map = HashMap::new();
            map.insert("operation".to_string(), operation.to_string());
            map.insert("model".to_string(), model.to_string());
            map
        };
        
        self.observe_histogram("ai_operation_duration_seconds", duration.as_secs_f64(), Some(labels)).await;
    }
    
    pub async fn get_metrics(&self) -> Vec<Metric> {
        let metrics = self.metrics.read().await;
        metrics.values().cloned().collect()
    }
    
    pub async fn get_histograms(&self) -> Vec<HistogramMetric> {
        let histograms = self.histograms.read().await;
        histograms.values().cloned().collect()
    }
    
    pub async fn get_uptime_seconds(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }
    
    pub async fn export_prometheus(&self) -> String {
        let mut output = String::new();
        
        // Add uptime
        let uptime = self.get_uptime_seconds().await;
        output.push_str(&format!("# HELP vectra_uptime_seconds Total uptime in seconds\n"));
        output.push_str(&format!("# TYPE vectra_uptime_seconds gauge\n"));
        output.push_str(&format!("vectra_uptime_seconds {}\n", uptime));
        
        // Export metrics
        let metrics = self.get_metrics().await;
        for metric in metrics {
            let labels_str = if metric.labels.is_empty() {
                String::new()
            } else {
                let label_pairs: Vec<String> = metric.labels
                    .iter()
                    .map(|(k, v)| format!("{}=\"{}\"", k, v))
                    .collect();
                format!("{{{}}}", label_pairs.join(","))
            };
            
            output.push_str(&format!("{}{} {}\n", metric.name, labels_str, metric.value));
        }
        
        // Export histograms
        let histograms = self.get_histograms().await;
        for histogram in histograms {
            let labels_str = if histogram.name.contains("query_type") {
                let query_type = histogram.name.split('_').last().unwrap_or("unknown");
                format!("{{query_type=\"{}\"}}", query_type)
            } else {
                String::new()
            };
            
            output.push_str(&format!("# HELP {}_sum Total sum of observed values\n", histogram.name));
            output.push_str(&format!("# TYPE {}_sum counter\n", histogram.name));
            output.push_str(&format!("{}_sum{} {}\n", histogram.name, labels_str, histogram.sum));
            
            output.push_str(&format!("# HELP {}_count Total count of observed values\n", histogram.name));
            output.push_str(&format!("# TYPE {}_count counter\n", histogram.name));
            output.push_str(&format!("{}_count{} {}\n", histogram.name, labels_str, histogram.count));
            
            for bucket in &histogram.buckets {
                let bucket_labels = if bucket.le == f64::INFINITY {
                    format!("{}le=\"+Inf\"", labels_str)
                } else {
                    format!("{}le=\"{}\"", labels_str, bucket.le)
                };
                output.push_str(&format!("{}_bucket{} {}\n", histogram.name, bucket_labels, bucket.count));
            }
        }
        
        output
    }
    
    fn metric_key(&self, name: &str, labels: Option<&HashMap<String, String>>) -> String {
        if let Some(labels) = labels {
            let mut sorted_labels: Vec<_> = labels.iter().collect();
            sorted_labels.sort_by_key(|(k, _)| *k);
            
            let labels_str = sorted_labels
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join(",");
            
            if labels_str.is_empty() {
                name.to_string()
            } else {
                format!("{}_{}", name, labels_str)
            }
        } else {
            name.to_string()
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[tokio::test]
    async fn test_counter_increment() {
        let collector = MetricsCollector::new();
        
        collector.increment_counter("test_counter", None).await;
        collector.increment_counter("test_counter", None).await;
        
        let metrics = collector.get_metrics().await;
        let counter = metrics.iter().find(|m| m.name == "test_counter").unwrap();
        
        assert_eq!(counter.value, 2.0);
        assert_eq!(counter.metric_type, MetricType::Counter);
    }
    
    #[tokio::test]
    async fn test_gauge_setting() {
        let collector = MetricsCollector::new();
        
        collector.set_gauge("test_gauge", 42.5, None).await;
        
        let metrics = collector.get_metrics().await;
        let gauge = metrics.iter().find(|m| m.name == "test_gauge").unwrap();
        
        assert_eq!(gauge.value, 42.5);
        assert_eq!(gauge.metric_type, MetricType::Gauge);
    }
    
    #[tokio::test]
    async fn test_histogram_observation() {
        let collector = MetricsCollector::new();
        
        collector.observe_histogram("test_histogram", 1.5, None).await;
        collector.observe_histogram("test_histogram", 2.5, None).await;
        
        let histograms = collector.get_histograms().await;
        let histogram = histograms.iter().find(|h| h.name == "test_histogram").unwrap();
        
        assert_eq!(histogram.sum, 4.0);
        assert_eq!(histogram.count, 2);
    }
}
