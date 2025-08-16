# VectraEdge User Guide

This guide covers the core features and capabilities of VectraEdge, helping you build AI-native analytics applications.

## 🗄️ Core Concepts

### OLAP Engine
VectraEdge is built on DataFusion and Apache Arrow, providing:
- **SQL Support**: Standard SQL with extensions for vector operations
- **Vectorized Processing**: High-performance columnar data processing
- **Memory Management**: Efficient memory usage with configurable limits
- **Query Optimization**: Automatic query planning and optimization

### Vector Search
- **HNSW Indexing**: Hierarchical Navigable Small World graphs for fast similarity search
- **pgvector Compatibility**: Familiar vector operations and syntax
- **Multiple Distance Metrics**: Cosine, Euclidean, and Manhattan distances
- **AI-Generated Embeddings**: Built-in embedding generation using local models

### Real-time Streaming
- **Change Data Capture (CDC)**: Track data changes in real-time
- **Redpanda Integration**: Kafka-compatible streaming platform
- **Event Processing**: SQL-based stream processing
- **Backpressure Handling**: Automatic flow control for high-throughput scenarios

## 📊 Data Management

### Creating Tables

#### Basic Tables
```sql
-- Standard SQL table creation
CREATE TABLE users (
  id INTEGER PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  email TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Table with vector columns
CREATE TABLE documents (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  content TEXT,
  embedding VECTOR(384),
  metadata JSONB
);
```

#### Vector Tables
```sql
-- Specialized vector table
CREATE TABLE embeddings (
  id SERIAL PRIMARY KEY,
  text TEXT NOT NULL,
  vector VECTOR(1536),
  model TEXT DEFAULT 'text-embedding-ada-002',
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create HNSW index for fast similarity search
CREATE INDEX ON embeddings USING hnsw (vector);
```

### Inserting Data

#### Standard Inserts
```sql
-- Single row insert
INSERT INTO users (username, email) VALUES ('alice', 'alice@example.com');

-- Multiple rows
INSERT INTO users (username, email) VALUES 
  ('bob', 'bob@example.com'),
  ('charlie', 'charlie@example.com');

-- Insert with AI-generated embedding
INSERT INTO documents (title, content, embedding) VALUES 
  ('AI Guide', 'Introduction to artificial intelligence', 
   ai_embedding('Introduction to artificial intelligence'));
```

#### Batch Operations
```python
from vectra import VectraClient

client = VectraClient(host="localhost", port=8080)

# Batch insert
documents = [
    {"title": "ML Basics", "content": "Machine learning fundamentals"},
    {"title": "Deep Learning", "content": "Neural networks and deep learning"},
    {"title": "NLP", "content": "Natural language processing"}
]

for doc in documents:
    doc['embedding'] = client.generate_embedding(doc['content'])
    client.insert_data("documents", doc)
```

## 🔍 Querying Data

### Standard SQL Queries
```sql
-- Basic SELECT
SELECT username, email FROM users WHERE created_at > '2024-01-01';

-- Aggregations
SELECT 
  COUNT(*) as total_users,
  AVG(EXTRACT(YEAR FROM created_at)) as avg_year
FROM users;

-- JOINs
SELECT 
  u.username,
  d.title,
  d.content
FROM users u
JOIN documents d ON u.id = d.user_id
WHERE u.username = 'alice';
```

### Vector Search Queries

#### Similarity Search
```sql
-- Find similar documents
SELECT 
  title,
  content,
  vector <-> ai_embedding('machine learning') as distance
FROM documents 
ORDER BY distance
LIMIT 10;

-- Search within distance threshold
SELECT title, content
FROM documents 
WHERE vector <-> ai_embedding('artificial intelligence') < 0.3;
```

#### Hybrid Search
```sql
-- Combine vector similarity with traditional filters
SELECT 
  title,
  content,
  vector <-> ai_embedding('AI tutorial') as similarity_score
FROM documents 
WHERE 
  created_at > '2024-01-01'
  AND vector <-> ai_embedding('AI tutorial') < 0.5
ORDER BY similarity_score;
```

### Python Client Queries
```python
from vectra import VectraClient

client = VectraClient(host="localhost", port=8080)

# Execute SQL
result = client.execute_query("""
  SELECT title, content 
  FROM documents 
  WHERE vector <-> %s < 0.3
  ORDER BY vector <-> %s
  LIMIT 5
""", [query_embedding, query_embedding])

# Vector search
results = client.vector_search(
    query="machine learning algorithms",
    table="documents",
    limit=10,
    distance_threshold=0.5
)

# Hybrid search
results = client.hybrid_search(
    query="AI tutorial",
    filters={"created_at": "> 2024-01-01"},
    limit=20
)
```

## 🔄 Streaming & CDC

### Setting Up Streams
```sql
-- Create stream from table
CREATE STREAM user_changes FROM users;

-- Create stream with specific columns
CREATE STREAM user_activity FROM users 
  SELECT id, username, created_at;

-- Create filtered stream
CREATE STREAM active_users FROM users 
  WHERE last_login > CURRENT_TIMESTAMP - INTERVAL '7 days';
```

### Consuming Streams
```sql
-- Subscribe to stream
SELECT * FROM stream('user_changes') WHERE op = 'insert';

-- Process stream with windowing
SELECT 
  username,
  COUNT(*) as changes,
  window_start,
  window_end
FROM stream('user_changes')
WINDOW TUMBLING (SIZE 1 HOUR)
GROUP BY username, window_start, window_end;

-- Join stream with table
SELECT 
  s.username,
  s.op,
  u.email,
  s.timestamp
FROM stream('user_changes') s
JOIN users u ON s.id = u.id;
```

### Python Streaming
```python
# Subscribe to stream
subscription = client.subscribe_stream("user_changes")

for event in subscription:
    print(f"Operation: {event.operation}")
    print(f"Data: {event.data}")
    print(f"Timestamp: {event.timestamp}")
    
    # Process event
    if event.operation == "insert":
        process_new_user(event.data)
    elif event.operation == "update":
        process_user_update(event.data)
```

## 🤖 AI Integration

### Embedding Generation
```sql
-- Generate embeddings for text
SELECT ai_embedding('Hello, world!') as embedding;

-- Generate embeddings for table data
SELECT 
  title,
  ai_embedding(title || ' ' || content) as embedding
FROM documents;

-- Update existing embeddings
UPDATE documents 
SET embedding = ai_embedding(content)
WHERE embedding IS NULL;
```

### Model Inference
```sql
-- Text generation
SELECT ai_generate_text('Explain machine learning', 
                       model='llama2', 
                       max_tokens=100) as explanation;

-- Classification
SELECT 
  content,
  ai_classify(content, 
              categories=['tech', 'science', 'business']) as category
FROM documents;
```

### Python AI Operations
```python
# Generate embeddings
embedding = client.generate_embedding("machine learning tutorial")

# Text generation
response = client.generate_text(
    prompt="Explain vector databases",
    model="llama2",
    max_tokens=200
)

# Classification
category = client.classify_text(
    text="This is about artificial intelligence",
    categories=["AI", "ML", "Data Science"]
)
```

## ⚙️ Configuration

### Server Configuration
```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4
max_connections = 1000

[storage]
data_dir = "./data"
max_memory_mb = 2048
compression = true
backup_enabled = true

[vector_search]
dimension = 384
m = 16
ef_construction = 200
ef = 50
distance_metric = "cosine"

[ai]
ollama_url = "http://localhost:11434"
embedding_model = "text-embedding-ada-002"
text_model = "llama2"
max_tokens = 2048
temperature = 0.7
```

### Environment Variables
```bash
# Server
export VECTRA_HOST=0.0.0.0
export VECTRA_PORT=8080
export VECTRA_WORKERS=4

# Storage
export VECTRA_DATA_DIR=/data/vectra
export VECTRA_MAX_MEMORY_MB=2048

# Vector Search
export VECTRA_VECTOR_DIMENSION=384
export VECTRA_HNSW_M=16

# AI
export VECTRA_OLLAMA_URL=http://localhost:11434
export VECTRA_EMBEDDING_MODEL=text-embedding-ada-002
```

## 📈 Performance & Monitoring

### Query Optimization
```sql
-- Analyze query plan
EXPLAIN SELECT * FROM documents WHERE vector <-> %s < 0.3;

-- Check index usage
SELECT * FROM pg_stat_user_indexes WHERE indexrelname LIKE '%hnsw%';

-- Monitor query performance
SELECT 
  query,
  mean_time,
  calls,
  total_time
FROM pg_stat_statements 
ORDER BY mean_time DESC;
```

### System Metrics
```python
# Get system status
status = client.get_status()
print(f"Memory usage: {status.memory_usage_mb}MB")
print(f"Active connections: {status.active_connections}")
print(f"Query count: {status.total_queries}")

# Get performance metrics
metrics = client.get_metrics()
print(f"Average query time: {metrics.avg_query_time_ms}ms")
print(f"Vector search latency: {metrics.vector_search_latency_ms}ms")
```

## 🔒 Security

### Authentication
```python
# Connect with credentials
client = VectraClient(
    host="localhost",
    port=8080,
    username="admin",
    password="secure_password"
)

# API key authentication
client = VectraClient(
    host="localhost",
    port=8080,
    api_key="your_api_key_here"
)
```

### Row-Level Security
```sql
-- Enable RLS
ALTER TABLE documents ENABLE ROW LEVEL SECURITY;

-- Create policy
CREATE POLICY user_documents ON documents
  FOR ALL TO authenticated_users
  USING (user_id = current_user_id());

-- Check current user
SELECT current_user_id(), current_user_role();
```

---

*Ready to explore advanced features? Check out the [API Reference](api-reference.md) for complete documentation!*
