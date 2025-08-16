use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::Arc;
use tokio::runtime::Runtime;

use crate::{
    engine::VectraEngine,
    vector::VectorIndex,
    storage::StorageManager,
    ai::AIRuntime,
    config::Config,
};

pub fn vector_search_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    rt.block_on(async {
        let config = Config::default();
        let vector_index = VectorIndex::new(&config).await.unwrap();
        
        // Create test index
        vector_index.create_index("benchmark_table", "embedding").await.unwrap();
        
        // Insert test vectors
        for i in 0..1000 {
            let vector: Vec<f32> = (0..384).map(|j| (i + j) as f32 / 1000.0).collect();
            vector_index.insert_vector("benchmark_table", "embedding", i as u32, &vector).await.unwrap();
        }
        
        // Benchmark search
        c.bench_function("vector_search_1000_vectors", |b| {
            b.iter(|| {
                let query_vector: Vec<f32> = (0..384).map(|i| i as f32 / 1000.0).collect();
                rt.block_on(async {
                    vector_index.search(&query_vector, 10).await.unwrap()
                });
            });
        });
    });
}

pub fn storage_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    rt.block_on(async {
        let config = Config::default();
        let storage = StorageManager::new(&config).await.unwrap();
        
        // Benchmark table creation
        c.bench_function("create_table", |b| {
            b.iter(|| {
                rt.block_on(async {
                    storage.create_table("benchmark_table", "id INT, data TEXT").await.unwrap();
                });
            });
        });
        
        // Benchmark data insertion
        let test_data = serde_json::json!({
            "id": 1,
            "data": "benchmark test data"
        });
        
        c.bench_function("insert_data", |b| {
            b.iter(|| {
                rt.block_on(async {
                    storage.insert_data("benchmark_table", "key1", &test_data).await.unwrap();
                });
            });
        });
        
        // Benchmark data retrieval
        c.bench_function("get_data", |b| {
            b.iter(|| {
                rt.block_on(async {
                    storage.get_data("benchmark_table", "key1").await.unwrap();
                });
            });
        });
    });
}

pub fn ai_runtime_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    rt.block_on(async {
        let config = Config::default();
        let ai_runtime = AIRuntime::new(&config).await.unwrap();
        
        // Benchmark embedding generation
        c.bench_function("generate_embedding", |b| {
            b.iter(|| {
                rt.block_on(async {
                    ai_runtime.generate_embedding("benchmark test text").await.unwrap();
                });
            });
        });
        
        // Benchmark text generation
        c.bench_function("generate_text", |b| {
            b.iter(|| {
                rt.block_on(async {
                    ai_runtime.generate_text("benchmark prompt", 100).await.unwrap();
                });
            });
        });
        
        // Benchmark text classification
        let categories = vec!["positive".to_string(), "negative".to_string()];
        
        c.bench_function("classify_text", |b| {
            b.iter(|| {
                rt.block_on(async {
                    ai_runtime.classify_text("benchmark text to classify", &categories).await.unwrap();
                });
            });
        });
    });
}

pub fn engine_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    rt.block_on(async {
        let config = Config::default();
        let engine = VectraEngine::new(&config).await.unwrap();
        
        // Benchmark SQL execution
        c.bench_function("execute_sql_query", |b| {
            b.iter(|| {
                rt.block_on(async {
                    engine.execute_query("SELECT 1 as test").await.unwrap();
                });
            });
        });
        
        // Benchmark vector search
        c.bench_function("engine_vector_search", |b| {
            b.iter(|| {
                rt.block_on(async {
                    engine.vector_search("benchmark query", 10).await.unwrap();
                });
            });
        });
    });
}

criterion_group!(
    benches,
    vector_search_benchmark,
    storage_benchmark,
    ai_runtime_benchmark,
    engine_benchmark
);
criterion_main!(benches);
