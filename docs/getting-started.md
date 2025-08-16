# Getting Started with VectraEdge

This guide will walk you through setting up and running VectraEdge for the first time.

## 🚀 Quick Start (5 minutes)

### Prerequisites

- **Docker & Docker Compose** - For the fastest setup
- **Rust 1.75+** - If building from source
- **Python 3.8+** - For Python bindings
- **Node.js 18+** - For the web playground

### Option 1: Docker (Recommended)

```bash
# Clone the repository
git clone https://github.com/makalin/vectraedge.git
cd vectraedge

# Start all services
make start

# Access services
# - Backend API: http://localhost:8080
# - Playground:  http://localhost:3000
# - Redpanda:    http://localhost:8081
# - Ollama:      http://localhost:11434
```

### Option 2: From Source

```bash
# Clone and build
git clone https://github.com/makalin/vectraedge.git
cd vectraedge

# Build Rust application
make build

# Run the server
make run
```

### Option 3: Python Package

```bash
# Install Python bindings
pip install vectra

# Quick test
python -c "
from vectra import connect
client = connect(host='localhost', port=8080)
print('Connected to VectraEdge!')
"
```

## 🧪 Your First Query

Once VectraEdge is running, try these examples:

### SQL Query
```sql
-- Create a simple table
CREATE TABLE users (
  id INTEGER PRIMARY KEY,
  name TEXT,
  age INTEGER
);

-- Insert data
INSERT INTO users VALUES (1, 'Alice', 30), (2, 'Bob', 25);

-- Query data
SELECT * FROM users WHERE age > 25;
```

### Vector Search
```sql
-- Create table with vector column
CREATE TABLE documents (
  id SERIAL PRIMARY KEY,
  content TEXT,
  embedding VECTOR(384)
);

-- Insert with AI-generated embedding
INSERT INTO documents(content, embedding) 
VALUES ('machine learning tutorial', ai_embedding('machine learning tutorial'));

-- Vector similarity search
SELECT content, embedding <-> ai_embedding('AI guide') as distance
FROM documents 
ORDER BY distance
LIMIT 5;
```

### Python Client
```python
from vectra import VectraClient

# Connect
client = VectraClient(host="localhost", port=8080)

# Execute SQL
result = client.execute_query("SELECT version()")
print(result)

# Vector search
results = client.vector_search("artificial intelligence", limit=10)
for doc in results:
    print(f"Score: {doc.score}, Content: {doc.content}")
```

## 🔧 Configuration

### Environment Variables
```bash
# Server settings
export VECTRA_HOST=0.0.0.0
export VECTRA_PORT=8080
export VECTRA_LOG_LEVEL=info

# Storage
export VECTRA_DATA_DIR=./data

# AI Models
export VECTRA_OLLAMA_URL=http://localhost:11434
```

### Configuration File
Create `config/vectra.toml`:
```toml
[server]
host = "0.0.0.0"
port = 8080

[storage]
data_dir = "./data"
max_memory_mb = 1024

[ai]
ollama_url = "http://localhost:11434"
embedding_model = "text-embedding-ada-002"
```

## 🎮 Web Playground

Access the interactive web interface at `http://localhost:3000` to:

- Execute SQL queries
- Explore vector search
- Monitor system metrics
- Test streaming features

## 📊 Next Steps

- **Read the [User Guide](user-guide.md)** for detailed feature explanations
- **Check [API Reference](api-reference.md)** for complete API documentation
- **Explore [Examples](examples/)** for more use cases
- **Join the community** on [GitHub Discussions](https://github.com/makalin/vectraedge/discussions)

## 🆘 Need Help?

- **Documentation**: Browse this site for guides
- **Issues**: Report bugs on [GitHub](https://github.com/makalin/vectraedge/issues)
- **Support**: Email makalin@gmail.com
- **Examples**: Check the [examples](examples/) directory

---

*Ready to explore more? Check out the [User Guide](user-guide.md)!*
