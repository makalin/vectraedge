# VectraEdge API Reference

Complete API documentation for VectraEdge, including SQL functions, Python client methods, and HTTP endpoints.

## 🔌 SQL Functions

### Vector Operations

#### `ai_embedding(text)`
Generate embeddings for text using the configured AI model.

**Parameters:**
- `text` (TEXT): Input text to embed

**Returns:** `VECTOR(dimension)`

**Example:**
```sql
SELECT ai_embedding('machine learning tutorial') as embedding;
```

#### `ai_generate_text(prompt, model, max_tokens, temperature)`
Generate text using the specified AI model.

**Parameters:**
- `prompt` (TEXT): Input prompt
- `model` (TEXT, optional): Model name (default: configured default)
- `max_tokens` (INTEGER, optional): Maximum tokens to generate (default: 2048)
- `temperature` (FLOAT, optional): Sampling temperature (default: 0.7)

**Returns:** `TEXT`

**Example:**
```sql
SELECT ai_generate_text('Explain AI', 'llama2', 100, 0.5) as response;
```

#### `ai_classify(text, categories)`
Classify text into predefined categories.

**Parameters:**
- `text` (TEXT): Text to classify
- `categories` (ARRAY): Array of category names

**Returns:** `TEXT`

**Example:**
```sql
SELECT ai_classify('machine learning', ['tech', 'science', 'business']) as category;
```

### Vector Search Functions

#### `vector_distance(vector1, vector2, metric)`
Calculate distance between two vectors.

**Parameters:**
- `vector1` (VECTOR): First vector
- `vector2` (VECTOR): Second vector
- `metric` (TEXT, optional): Distance metric ('cosine', 'euclidean', 'manhattan')

**Returns:** `FLOAT`

**Example:**
```sql
SELECT vector_distance(embedding1, embedding2, 'cosine') as similarity;
```

#### `vector_norm(vector)`
Calculate the L2 norm (magnitude) of a vector.

**Parameters:**
- `vector` (VECTOR): Input vector

**Returns:** `FLOAT`

**Example:**
```sql
SELECT vector_norm(embedding) as magnitude FROM documents;
```

### Streaming Functions

#### `stream(stream_name)`
Access a named stream for real-time data processing.

**Parameters:**
- `stream_name` (TEXT): Name of the stream

**Returns:** Stream of events

**Example:**
```sql
SELECT * FROM stream('user_changes') WHERE op = 'insert';
```

#### `stream_window(stream_name, size, slide)`
Apply windowing to a stream.

**Parameters:**
- `stream_name` (TEXT): Name of the stream
- `size` (INTERVAL): Window size
- `slide` (INTERVAL, optional): Slide interval (default: same as size)

**Returns:** Windowed stream

**Example:**
```sql
SELECT 
  username,
  COUNT(*) as events
FROM stream_window('user_activity', INTERVAL '1 hour')
GROUP BY username, window_start, window_end;
```

## 🐍 Python Client API

### Connection

#### `VectraClient(host, port, username, password, api_key, **kwargs)`
Create a connection to VectraEdge.

**Parameters:**
- `host` (str): Server hostname (default: "localhost")
- `port` (int): Server port (default: 8080)
- `username` (str, optional): Username for authentication
- `password` (str, optional): Password for authentication
- `api_key` (str, optional): API key for authentication
- `timeout` (int, optional): Connection timeout in seconds (default: 30)
- `ssl` (bool, optional): Use SSL/TLS (default: False)

**Returns:** `VectraClient` instance

**Example:**
```python
from vectra import VectraClient

client = VectraClient(
    host="localhost",
    port=8080,
    username="admin",
    password="password"
)
```

### Core Operations

#### `execute_query(sql, params=None)`
Execute a SQL query.

**Parameters:**
- `sql` (str): SQL query string
- `params` (list, optional): Query parameters

**Returns:** `QueryResult`

**Example:**
```python
result = client.execute_query(
    "SELECT * FROM users WHERE age > %s",
    [25]
)

for row in result:
    print(f"User: {row['username']}, Age: {row['age']}")
```

#### `execute_batch(queries)`
Execute multiple SQL queries in a batch.

**Parameters:**
- `queries` (list): List of SQL query strings

**Returns:** `list[QueryResult]`

**Example:**
```python
queries = [
    "CREATE TABLE users (id INT, name TEXT)",
    "INSERT INTO users VALUES (1, 'Alice')",
    "SELECT * FROM users"
]

results = client.execute_batch(queries)
```

### Table Management

#### `create_table(name, schema)`
Create a new table.

**Parameters:**
- `name` (str): Table name
- `schema` (str): Table schema definition

**Returns:** `bool`

**Example:**
```python
success = client.create_table(
    "documents",
    "id SERIAL PRIMARY KEY, title TEXT, content TEXT, embedding VECTOR(384)"
)
```

#### `drop_table(name, if_exists=False)`
Drop a table.

**Parameters:**
- `name` (str): Table name
- `if_exists` (bool): Don't error if table doesn't exist

**Returns:** `bool`

**Example:**
```python
client.drop_table("old_table", if_exists=True)
```

#### `table_exists(name)`
Check if a table exists.

**Parameters:**
- `name` (str): Table name

**Returns:** `bool`

**Example:**
```python
if client.table_exists("users"):
    print("Users table exists")
```

### Data Operations

#### `insert_data(table, data)`
Insert data into a table.

**Parameters:**
- `table` (str): Table name
- `data` (dict): Data to insert

**Returns:** `int` (number of rows inserted)

**Example:**
```python
user_data = {
    "username": "alice",
    "email": "alice@example.com",
    "age": 30
}

rows_inserted = client.insert_data("users", user_data)
```

#### `insert_batch(table, data_list)`
Insert multiple rows in a batch.

**Parameters:**
- `table` (str): Table name
- `data_list` (list): List of data dictionaries

**Returns:** `int` (number of rows inserted)

**Example:**
```python
users = [
    {"username": "alice", "email": "alice@example.com"},
    {"username": "bob", "email": "bob@example.com"}
]

rows_inserted = client.insert_batch("users", users)
```

#### `update_data(table, data, where)`
Update data in a table.

**Parameters:**
- `table` (str): Table name
- `data` (dict): Data to update
- `where` (dict): WHERE conditions

**Returns:** `int` (number of rows updated)

**Example:**
```python
update_data = {"age": 31}
where_conditions = {"username": "alice"}

rows_updated = client.update_data("users", update_data, where_conditions)
```

#### `delete_data(table, where)`
Delete data from a table.

**Parameters:**
- `table` (str): Table name
- `where` (dict): WHERE conditions

**Returns:** `int` (number of rows deleted)

**Example:**
```python
where_conditions = {"username": "alice"}
rows_deleted = client.delete_data("users", where_conditions)
```

### Vector Search

#### `vector_search(query, table, limit=10, distance_threshold=None, filters=None)`
Perform vector similarity search.

**Parameters:**
- `query` (str): Search query text
- `table` (str): Table to search in
- `limit` (int, optional): Maximum results (default: 10)
- `distance_threshold` (float, optional): Maximum distance threshold
- `filters` (dict, optional): Additional filters

**Returns:** `list[SearchResult]`

**Example:**
```python
results = client.vector_search(
    query="machine learning",
    table="documents",
    limit=20,
    distance_threshold=0.5,
    filters={"category": "tutorial"}
)

for result in results:
    print(f"Score: {result.score}, Title: {result.data['title']}")
```

#### `hybrid_search(query, filters=None, limit=10, distance_threshold=None)`
Perform hybrid search combining vector similarity with traditional filters.

**Parameters:**
- `query` (str): Search query text
- `filters` (dict, optional): Traditional filters
- `limit` (int, optional): Maximum results (default: 10)
- `distance_threshold` (float, optional): Maximum distance threshold

**Returns:** `list[SearchResult]`

**Example:**
```python
results = client.hybrid_search(
    query="AI tutorial",
    filters={
        "created_at": "> 2024-01-01",
        "category": "tutorial"
    },
    limit=15
)
```

### AI Operations

#### `generate_embedding(text, model=None)`
Generate embeddings for text.

**Parameters:**
- `text` (str): Input text
- `model` (str, optional): Model name (default: configured default)

**Returns:** `list[float]`

**Example:**
```python
embedding = client.generate_embedding("machine learning tutorial")
print(f"Embedding dimension: {len(embedding)}")
```

#### `generate_text(prompt, model=None, max_tokens=None, temperature=None)`
Generate text using AI models.

**Parameters:**
- `prompt` (str): Input prompt
- `model` (str, optional): Model name
- `max_tokens` (int, optional): Maximum tokens to generate
- `temperature` (float, optional): Sampling temperature

**Returns:** `str`

**Example:**
```python
response = client.generate_text(
    prompt="Explain vector databases",
    model="llama2",
    max_tokens=200,
    temperature=0.7
)
```

#### `classify_text(text, categories, model=None)`
Classify text into categories.

**Parameters:**
- `text` (str): Text to classify
- `categories` (list): List of category names
- `model` (str, optional): Model name

**Returns:** `str`

**Example:**
```python
category = client.classify_text(
    text="This is about artificial intelligence",
    categories=["AI", "ML", "Data Science"]
)
```

### Streaming

#### `subscribe_stream(stream_name, filters=None)`
Subscribe to a data stream.

**Parameters:**
- `stream_name` (str): Name of the stream
- `filters` (dict, optional): Stream filters

**Returns:** `StreamSubscription`

**Example:**
```python
subscription = client.subscribe_stream("user_changes")

for event in subscription:
    print(f"Operation: {event.operation}")
    print(f"Data: {event.data}")
    print(f"Timestamp: {event.timestamp}")
```

#### `create_stream(stream_name, table, columns=None, filters=None)`
Create a new stream from a table.

**Parameters:**
- `stream_name` (str): Name of the stream
- `table` (str): Source table name
- `columns` (list, optional): Columns to include
- `filters` (dict, optional): Row filters

**Returns:** `bool`

**Example:**
```python
success = client.create_stream(
    "user_activity",
    "users",
    columns=["id", "username", "last_login"],
    filters={"active": True}
)
```

### System Operations

#### `get_status()`
Get system status information.

**Returns:** `SystemStatus`

**Example:**
```python
status = client.get_status()
print(f"Memory: {status.memory_usage_mb}MB")
print(f"Connections: {status.active_connections}")
print(f"Queries: {status.total_queries}")
```

#### `get_metrics()`
Get performance metrics.

**Returns:** `PerformanceMetrics`

**Example:**
```python
metrics = client.get_metrics()
print(f"Avg query time: {metrics.avg_query_time_ms}ms")
print(f"Vector search latency: {metrics.vector_search_latency_ms}ms")
```

#### `health_check()`
Check if the server is healthy.

**Returns:** `bool`

**Example:**
```python
if client.health_check():
    print("Server is healthy")
else:
    print("Server is not responding")
```

## 🌐 HTTP API Endpoints

### Authentication

#### `POST /auth/login`
Authenticate with username and password.

**Request Body:**
```json
{
  "username": "admin",
  "password": "password"
}
```

**Response:**
```json
{
  "token": "jwt_token_here",
  "expires_at": "2024-12-31T23:59:59Z"
}
```

#### `POST /auth/refresh`
Refresh authentication token.

**Headers:**
```
Authorization: Bearer <token>
```

**Response:**
```json
{
  "token": "new_jwt_token_here",
  "expires_at": "2024-12-31T23:59:59Z"
}
```

### SQL Execution

#### `POST /sql/execute`
Execute a SQL query.

**Headers:**
```
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "query": "SELECT * FROM users WHERE age > ?",
  "params": [25]
}
```

**Response:**
```json
{
  "columns": ["id", "username", "age"],
  "rows": [
    [1, "alice", 30],
    [2, "bob", 28]
  ],
  "row_count": 2
}
```

#### `POST /sql/batch`
Execute multiple SQL queries.

**Request Body:**
```json
{
  "queries": [
    "CREATE TABLE users (id INT, name TEXT)",
    "INSERT INTO users VALUES (1, 'Alice')",
    "SELECT * FROM users"
  ]
}
```

### Vector Search

#### `POST /search/vector`
Perform vector similarity search.

**Request Body:**
```json
{
  "query": "machine learning",
  "table": "documents",
  "limit": 10,
  "distance_threshold": 0.5,
  "filters": {
    "category": "tutorial"
  }
}
```

**Response:**
```json
{
  "results": [
    {
      "score": 0.123,
      "data": {
        "id": 1,
        "title": "ML Basics",
        "content": "Machine learning fundamentals"
      }
    }
  ],
  "total": 1
}
```

### AI Operations

#### `POST /ai/embed`
Generate embeddings.

**Request Body:**
```json
{
  "text": "machine learning tutorial",
  "model": "text-embedding-ada-002"
}
```

**Response:**
```json
{
  "embedding": [0.1, 0.2, 0.3, ...],
  "dimension": 384,
  "model": "text-embedding-ada-002"
}
```

#### `POST /ai/generate`
Generate text.

**Request Body:**
```json
{
  "prompt": "Explain AI",
  "model": "llama2",
  "max_tokens": 100,
  "temperature": 0.7
}
```

**Response:**
```json
{
  "text": "Artificial Intelligence (AI) is a branch of computer science...",
  "tokens_used": 45
}
```

### Streaming

#### `GET /streams/{stream_name}/subscribe`
Subscribe to a stream via Server-Sent Events.

**Headers:**
```
Authorization: Bearer <token>
```

**Response:**
```
data: {"operation": "insert", "data": {"id": 1, "name": "Alice"}}

data: {"operation": "update", "data": {"id": 1, "name": "Alice Smith"}}
```

### System

#### `GET /health`
Health check endpoint.

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T12:00:00Z",
  "version": "1.0.0"
}
```

#### `GET /status`
Get system status.

**Response:**
```json
{
  "memory_usage_mb": 512,
  "active_connections": 5,
  "total_queries": 1234,
  "uptime_seconds": 3600
}
```

#### `GET /metrics`
Get performance metrics.

**Response:**
```json
{
  "avg_query_time_ms": 45.2,
  "vector_search_latency_ms": 12.8,
  "queries_per_second": 22.1,
  "memory_efficiency": 0.85
}
```

## 📊 Data Types

### Vector Type
- **Syntax**: `VECTOR(dimension)`
- **Example**: `VECTOR(384)`
- **Operations**: `<->`, `<#>`, `<=>`
- **Functions**: `vector_distance()`, `vector_norm()`

### JSONB Type
- **Syntax**: `JSONB`
- **Example**: `{"key": "value", "array": [1, 2, 3]}`
- **Operations**: `->`, `->>`, `@>`, `?`
- **Functions**: `jsonb_extract_path()`, `jsonb_array_elements()`

### Stream Type
- **Syntax**: `STREAM`
- **Usage**: `SELECT * FROM stream('name')`
- **Operations**: Window functions, aggregations
- **Functions**: `stream_window()`, `stream_filter()`

## 🔧 Configuration

### Server Configuration
```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4
max_connections = 1000
request_timeout = 30
max_request_size = "10MB"

[security]
jwt_secret = "your-secret-key"
jwt_expiry = "24h"
cors_origins = ["*"]
rate_limit = 1000

[logging]
level = "info"
format = "json"
output = "stdout"
```

### Storage Configuration
```toml
[storage]
data_dir = "./data"
max_memory_mb = 2048
compression = true
backup_enabled = true
backup_interval = "1h"

[rocksdb]
max_open_files = 1000
block_cache_size_mb = 512
write_buffer_size_mb = 64
max_write_buffer_number = 4

[sled]
cache_capacity = 1000000
compression = true
```

### Vector Search Configuration
```toml
[vector_search]
dimension = 384
m = 16
ef_construction = 200
ef = 50
distance_metric = "cosine"
index_type = "hnsw"

[hnsw]
max_elements = 1000000
ef_construction = 200
ef = 50
num_threads = 4
```

---

*This API reference covers the core functionality. For advanced usage and examples, check the [User Guide](user-guide.md) and [Examples](examples/) directory.*
