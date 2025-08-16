use vectra::{
    engine::VectraEngine,
    vector::VectorIndex,
    streaming::StreamManager,
    ai::AIRuntime,
    storage::StorageManager,
    config::Config,
    metrics::MetricsCollector,
    cache::{QueryCache, VectorCache},
    sql_parser::SQLParser,
};

use tempfile::tempdir;
use tokio::test;

#[test]
async fn test_full_engine_workflow() {
    // Setup
    let config = Config::default();
    let engine = VectraEngine::new(&config).await.unwrap();
    
    // Test table creation
    engine.create_table("test_docs", "id INT, content TEXT, embedding VECTOR(384)").await.unwrap();
    
    // Test data insertion
    let test_data = serde_json::json!({
        "id": 1,
        "content": "machine learning introduction",
        "embedding": [0.1, 0.2, 0.3]
    });
    engine.insert_data("test_docs", &test_data).await.unwrap();
    
    // Test vector index creation
    engine.create_vector_index("test_docs", "embedding").await.unwrap();
    
    // Test vector search
    let results = engine.vector_search("machine learning", 5).await.unwrap();
    assert!(!results.is_empty());
    
    // Test SQL execution
    let sql_result = engine.execute_query("SELECT COUNT(*) FROM test_docs").await.unwrap();
    assert!(sql_result.get("rows").is_some());
}

#[test]
async fn test_vector_search_integration() {
    let config = Config::default();
    let vector_index = VectorIndex::new(&config).await.unwrap();
    
    // Create index
    vector_index.create_index("test_table", "embedding").await.unwrap();
    
    // Insert test vectors
    for i in 0..100 {
        let vector: Vec<f32> = (0..384).map(|j| (i + j) as f32 / 1000.0).collect();
        vector_index.insert_vector("test_table", "embedding", i as u32, &vector).await.unwrap();
    }
    
    // Test search
    let query_vector: Vec<f32> = (0..384).map(|i| i as f32 / 1000.0).collect();
    let results = vector_index.search(&query_vector, 10).await.unwrap();
    
    assert_eq!(results.len(), 10);
    
    // Verify results are ordered by similarity
    if results.len() > 1 {
        let first_score = results[0]["score"].as_f64().unwrap();
        let second_score = results[1]["score"].as_f64().unwrap();
        assert!(first_score >= second_score); // Higher score = more similar
    }
}

#[test]
async fn test_streaming_integration() {
    let config = Config::default();
    let stream_manager = StreamManager::new(&config).await.unwrap();
    
    // Create topic
    stream_manager.create_topic("test_events", 1, 1).await.unwrap();
    
    // Subscribe to stream
    let subscription = stream_manager.subscribe("test_events").await.unwrap();
    assert_eq!(subscription.topic, "test_events");
    assert_eq!(subscription.status, "active");
    
    // Publish message
    let message = serde_json::json!({
        "event": "user_login",
        "user_id": 123,
        "timestamp": "2024-01-01T00:00:00Z"
    });
    stream_manager.publish("test_events", message).await.unwrap();
    
    // Get topic stats
    let stats = stream_manager.get_topic_stats("test_events").await.unwrap();
    assert_eq!(stats["status"], "active");
    assert!(stats["subscriptions"].as_u64().unwrap() > 0);
    
    // Unsubscribe
    stream_manager.unsubscribe(&subscription.id).await.unwrap();
    
    // Clean up
    stream_manager.delete_topic("test_events").await.unwrap();
}

#[test]
async fn test_ai_runtime_integration() {
    let config = Config::default();
    let ai_runtime = AIRuntime::new(&config).await.unwrap();
    
    // Test embedding generation
    let text = "artificial intelligence and machine learning";
    let embedding = ai_runtime.generate_embedding(text).await.unwrap();
    assert_eq!(embedding.len(), 384);
    
    // Test text generation
    let generated_text = ai_runtime.generate_text("Explain AI", 50).await.unwrap();
    assert!(!generated_text.is_empty());
    
    // Test text classification
    let categories = vec!["technology".to_string(), "science".to_string(), "other".to_string()];
    let classification = ai_runtime.classify_text("machine learning algorithms", &categories).await.unwrap();
    
    assert_eq!(classification.len(), 3);
    
    // Verify probabilities sum to 1.0
    let total_prob: f32 = classification.values().sum();
    assert!((total_prob - 1.0).abs() < 0.001);
    
    // Test model management
    ai_runtime.add_model(
        "custom_model",
        vectra::ai::ModelType::Custom,
        std::collections::HashMap::new()
    ).await.unwrap();
    
    let models = ai_runtime.list_models().await.unwrap();
    assert!(models.iter().any(|m| m.name == "custom_model"));
}

#[test]
async fn test_storage_integration() {
    let temp_dir = tempdir().unwrap();
    let mut config = Config::default();
    config.storage.rocksdb_path = Some(temp_dir.path().join("rocksdb").to_string_lossy().to_string());
    config.storage.sled_path = Some(temp_dir.path().join("sled").to_string_lossy().to_string());
    
    let storage = StorageManager::new(&config).await.unwrap();
    
    // Test table operations
    storage.create_table("test_users", "id INT, name TEXT, age INT").await.unwrap();
    
    let tables = storage.list_tables().await.unwrap();
    assert!(tables.iter().any(|t| t.name == "test_users"));
    
    // Test data operations
    let user_data = serde_json::json!({
        "id": 1,
        "name": "Alice",
        "age": 30
    });
    
    storage.insert_data("test_users", "user_1", &user_data).await.unwrap();
    
    let retrieved_data = storage.get_data("test_users", "user_1").await.unwrap();
    assert!(retrieved_data.is_some());
    assert_eq!(retrieved_data.unwrap()["name"], "Alice");
    
    // Test table info
    let table_info = storage.get_table_info("test_users").await.unwrap();
    assert!(table_info.is_some());
    let info = table_info.unwrap();
    assert_eq!(info.name, "test_users");
    assert_eq!(info.row_count, 1);
    
    // Test storage stats
    let stats = storage.get_storage_stats().await.unwrap();
    assert_eq!(stats["total_tables"], 1);
    assert_eq!(stats["total_rows"], 1);
    
    // Clean up
    storage.drop_table("test_users").await.unwrap();
}

#[test]
async fn test_metrics_integration() {
    let metrics = MetricsCollector::new();
    
    // Test counter operations
    metrics.increment_counter("test_counter", None).await;
    metrics.increment_counter("test_counter", None).await;
    
    let all_metrics = metrics.get_metrics().await;
    let counter = all_metrics.iter().find(|m| m.name == "test_counter").unwrap();
    assert_eq!(counter.value, 2.0);
    
    // Test gauge operations
    metrics.set_gauge("test_gauge", 42.5, None).await;
    
    let updated_metrics = metrics.get_metrics().await;
    let gauge = updated_metrics.iter().find(|m| m.name == "test_gauge").unwrap();
    assert_eq!(gauge.value, 42.5);
    
    // Test histogram operations
    metrics.record_query_duration("select", std::time::Duration::from_millis(150)).await;
    metrics.record_query_duration("select", std::time::Duration::from_millis(250)).await;
    
    let histograms = metrics.get_histograms().await;
    let query_histogram = histograms.iter().find(|h| h.name == "query_duration_seconds").unwrap();
    assert_eq!(query_histogram.count, 2);
    assert_eq!(query_histogram.sum, 0.4); // 0.15 + 0.25
    
    // Test Prometheus export
    let prometheus_export = metrics.export_prometheus().await;
    assert!(prometheus_export.contains("test_counter 2"));
    assert!(prometheus_export.contains("test_gauge 42.5"));
    assert!(prometheus_export.contains("query_duration_seconds_count 2"));
}

#[test]
async fn test_cache_integration() {
    let query_cache = QueryCache::new();
    let vector_cache = VectorCache::new();
    
    // Test query cache
    let sql = "SELECT * FROM users WHERE age > 18";
    let result = r#"{"rows": 5, "data": [{"id": 1, "name": "Alice"}]}"#;
    
    query_cache.cache_query_result(sql.to_string(), result.to_string()).await.unwrap();
    
    let cached_result = query_cache.get_query_result(sql).await;
    assert_eq!(cached_result, Some(result.to_string()));
    
    // Test vector cache
    let text = "machine learning algorithms";
    let embedding = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    
    vector_cache.cache_embedding(text.to_string(), embedding.clone()).await.unwrap();
    
    let cached_embedding = vector_cache.get_embedding(text).await;
    assert_eq!(cached_embedding, Some(embedding));
    
    // Test cache statistics
    let query_stats = query_cache.cache.get_stats().await;
    assert_eq!(query_stats.total_entries, 1);
    
    let vector_stats = vector_cache.get_stats().await;
    assert_eq!(vector_stats.total_entries, 1);
}

#[test]
async fn test_sql_parser_integration() {
    let parser = SQLParser::new();
    
    // Test SELECT parsing
    let select_sql = "SELECT id, name FROM users WHERE age > 18 LIMIT 10";
    let parsed_select = parser.parse(select_sql).await.unwrap();
    
    assert_eq!(parsed_select.statement_type, vectra::sql_parser::StatementType::Select);
    assert_eq!(parsed_select.table_name, Some("users".to_string()));
    assert_eq!(parsed_select.limit, Some(10));
    
    // Test CREATE parsing
    let create_sql = "CREATE TABLE docs (id INT PRIMARY KEY, content TEXT, embedding VECTOR(384))";
    let parsed_create = parser.parse(create_sql).await.unwrap();
    
    assert_eq!(parsed_create.statement_type, vectra::sql_parser::StatementType::Create);
    assert_eq!(parsed_create.table_name, Some("docs".to_string()));
    assert_eq!(parsed_create.columns.len(), 3);
    
    // Test vector operations extraction
    let vector_sql = "SELECT * FROM docs ORDER BY embedding <-> ai_embedding('query') LIMIT 5";
    let vector_ops = parser.extract_vector_operations(vector_sql);
    
    assert!(!vector_ops.is_empty());
    assert_eq!(vector_ops[0].operation_type, vectra::sql_parser::VectorOperationType::Search);
    
    // Test validation
    let mut invalid_parsed = parsed_create.clone();
    invalid_parsed.table_name = None;
    
    assert!(parser.validate_sql(&invalid_parsed).is_err());
    
    let valid_parsed = parsed_create.clone();
    assert!(parser.validate_sql(&valid_parsed).is_ok());
}

#[test]
async fn test_end_to_end_workflow() {
    let config = Config::default();
    let engine = VectraEngine::new(&config).await.unwrap();
    
    // 1. Create a document table with vector support
    engine.create_table(
        "documents",
        "id INT PRIMARY KEY, title TEXT, content TEXT, embedding VECTOR(384), created_at TIMESTAMP"
    ).await.unwrap();
    
    // 2. Insert documents with embeddings
    let documents = vec![
        ("Introduction to AI", "Artificial intelligence is a field of computer science..."),
        ("Machine Learning Basics", "Machine learning is a subset of AI..."),
        ("Deep Learning", "Deep learning uses neural networks..."),
        ("Natural Language Processing", "NLP deals with human language..."),
    ];
    
    for (i, (title, content)) in documents.iter().enumerate() {
        let doc_data = serde_json::json!({
            "id": i + 1,
            "title": title,
            "content": content,
            "created_at": "2024-01-01T00:00:00Z"
        });
        
        engine.insert_data("documents", &format!("doc_{}", i + 1), &doc_data).await.unwrap();
    }
    
    // 3. Create vector index
    engine.create_vector_index("documents", "embedding").await.unwrap();
    
    // 4. Perform vector search
    let search_results = engine.vector_search("artificial intelligence", 3).await.unwrap();
    assert!(!search_results.is_empty());
    
    // 5. Execute complex SQL query
    let sql = "SELECT title, content FROM documents WHERE title LIKE '%AI%' OR title LIKE '%Machine%' LIMIT 5";
    let sql_results = engine.execute_query(sql).await.unwrap();
    assert!(sql_results.get("rows").is_some());
    
    // 6. Subscribe to document changes
    let subscription = engine.subscribe_stream("document_changes").await.unwrap();
    assert_eq!(subscription.topic, "document_changes");
    
    // 7. Get system statistics
    let tables = engine.storage.list_tables().await.unwrap();
    assert!(tables.iter().any(|t| t.name == "documents"));
    
    let stats = engine.storage.get_storage_stats().await.unwrap();
    assert!(stats["total_tables"].as_u64().unwrap() > 0);
    assert!(stats["total_rows"].as_u64().unwrap() > 0);
}

#[test]
async fn test_error_handling() {
    let config = Config::default();
    let engine = VectraEngine::new(&config).await.unwrap();
    
    // Test invalid SQL
    let invalid_sql = "INVALID SQL STATEMENT";
    let result = engine.execute_query(invalid_sql).await;
    assert!(result.is_err());
    
    // Test non-existent table
    let result = engine.execute_query("SELECT * FROM non_existent_table").await;
    assert!(result.is_err());
    
    // Test invalid vector search
    let result = engine.vector_search("", 0).await;
    assert!(result.is_err());
}

#[test]
async fn test_concurrent_operations() {
    let config = Config::default();
    let engine = Arc::new(VectraEngine::new(&config).await.unwrap());
    
    let mut handles = vec![];
    
    // Spawn multiple concurrent operations
    for i in 0..10 {
        let engine_clone = Arc::clone(&engine);
        let handle = tokio::spawn(async move {
            // Create table
            engine_clone.create_table(
                &format!("concurrent_table_{}", i),
                "id INT, data TEXT"
            ).await.unwrap();
            
            // Insert data
            let data = serde_json::json!({
                "id": i,
                "data": format!("concurrent data {}", i)
            });
            engine_clone.insert_data(&format!("concurrent_table_{}", i), &format!("key_{}", i), &data).await.unwrap();
            
            // Query data
            let result = engine_clone.execute_query(&format!("SELECT * FROM concurrent_table_{}", i)).await.unwrap();
            assert!(result.get("rows").is_some());
        });
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }
    
    // Verify all tables were created
    let tables = engine.storage.list_tables().await.unwrap();
    assert!(tables.len() >= 10);
}
