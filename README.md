# VectraEdge 🚀
*An in-process, AI-native, real-time OLAP engine with vector search & streaming CDC.*

[![CI](https://github.com/makalin/vectraedge/workflows/CI/badge.svg)](https://github.com/makalin/vectraedge/actions)
[![Crates.io](https://img.shields.io/crates/v/vectra?label=crates)](https://crates.io/crates/vectra)
[![PyPI](https://img.shields.io/crates/v/vectra?label=pypi)](https://pypi.org/project/vectra/)
[![Docker](https://img.shields.io/docker/v/vectraedge/vectra?color=blue&label=docker)](https://hub.docker.com/r/vectraedge/vectra)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache%202.0-blue.svg)](LICENSE)

---

## 🎯 What is VectraEdge?

VectraEdge is a **DuckDB-class OLAP engine** that adds **vector search, real-time CDC streaming, and AI-driven autotuning**—all in a single binary you can embed, run serverless, or cluster.

### ✨ Key Features

- **🚀 DuckDB-class Performance**: Built on DataFusion + Arrow for lightning-fast analytics
- **🧠 AI-Native**: Built-in embeddings, vector search, and model inference
- **🔄 Real-time Streaming**: CDC with Redpanda for live data pipelines
- **🔍 Vector Search**: HNSW indexing with pgvector compatibility
- **💾 Multi-Storage**: RocksDB + Sled for edge-optimized storage
- **🌐 Multi-Language**: Rust core with Python bindings
- **🎮 Interactive Playground**: Next.js web interface for exploration

---

## 🚀 Quick Start

### Docker (Fastest)
```bash
# Clone and start everything
git clone https://github.com/makalin/vectraedge.git
cd vectraedge
make start

# Access services
# - Backend API: http://localhost:8080
# - Playground:  http://localhost:3000
# - Redpanda:    http://localhost:8081
# - Ollama:      http://localhost:11434
```

### Rust (From Source)
```bash
git clone https://github.com/makalin/vectraedge.git
cd vectraedge

# Build and run
make build
make run

# Or development mode
make dev
```

### Python
```bash
# Install Python bindings
pip install vectra

# Quick usage
from vectra import connect, quick_query, quick_search

# Connect to VectraEdge
client = connect(host="localhost", port=8080)

# Execute SQL
result = client.execute_query("SELECT * FROM docs LIMIT 5")

# Vector search
results = client.vector_search("machine learning", limit=10)
```

---

## 🏗️ Architecture

### Core Components

| Layer | Technology | Purpose |
|-------|------------|---------|
| **Query Engine** | DataFusion + Arrow | SQL execution & vectorized processing |
| **Vector Search** | HNSW + pgvector | Similarity search & embeddings |
| **Streaming** | Redpanda + Debezium | Real-time CDC & event streaming |
| **AI Runtime** | Ollama + ONNX | Local LLM inference & embeddings |
| **Storage** | RocksDB + Sled | LSM trees for edge optimization |
| **API** | Axum + Tokio | High-performance HTTP server |
| **Playground** | Next.js + Tailwind | Interactive web interface |

### Data Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Client    │───▶│  VectraEdge │───▶│   Storage   │
│  (SQL/API)  │    │   Engine    │    │ (RocksDB)   │
└─────────────┘    └─────────────┘    └─────────────┘
                           │
                    ┌──────┴──────┐
                    │             │
              ┌─────────┐  ┌─────────┐
              │ Vector  │  │Streaming│
              │ Search  │  │ (CDC)   │
              └─────────┘  └─────────┘
```

---

## 📚 Usage Examples

### SQL with Vector Search
```sql
-- Create table with vector column
CREATE TABLE docs (
  id SERIAL PRIMARY KEY,
  content TEXT,
  embedding VECTOR(384)
);

-- Insert with AI-generated embeddings
INSERT INTO docs(content, embedding) 
VALUES ('machine learning guide', ai_embedding('machine learning guide'));

-- Create HNSW index
CREATE INDEX ON docs USING hnsw (embedding);

-- Vector similarity search
SELECT content, embedding <-> ai_embedding('AI tutorial') as distance
FROM docs 
ORDER BY embedding <-> ai_embedding('AI tutorial')
LIMIT 5;
```

### Python Client
```python
from vectra import VectraClient

# Connect to VectraEdge
client = VectraClient(host="localhost", port=8080)

# Create table
client.create_table("users", "id INT, name TEXT, profile VECTOR(384)")

# Insert data
client.insert_data("users", {
    "id": 1,
    "name": "Alice",
    "profile": [0.1, 0.2, 0.3, ...]  # 384-dim vector
})

# Vector search
results = client.vector_search("software engineer", limit=10)

# Stream subscription
subscription = client.subscribe_stream("user_events")
```

### Real-time Streaming
```bash
# Terminal 1: Produce events
redpanda topic produce vectra_events

# Terminal 2: Subscribe from SQL
SELECT * FROM stream('vectra_events') WHERE op = 'insert';
```

---

## 🛠️ Development

### Prerequisites
- Rust 1.75+
- Python 3.8+
- Node.js 18+
- Docker & Docker Compose

### Setup Development Environment
```bash
git clone https://github.com/makalin/vectraedge.git
cd vectraedge

# Install dependencies and setup
make setup

# Start development
make dev              # Rust backend
make playground-dev   # Next.js frontend
```

### Project Structure
```
vectraedge/
├── src/                    # Rust source code
│   ├── main.rs            # Main application entry
│   ├── engine.rs          # Core OLAP engine
│   ├── vector.rs          # Vector search implementation
│   ├── streaming.rs       # CDC & streaming
│   ├── ai.rs              # AI runtime & models
│   ├── storage.rs         # Storage backends
│   ├── config.rs          # Configuration management
│   └── cli.rs             # Command-line interface
├── python/                 # Python bindings
│   ├── src/lib.rs         # PyO3 bindings
│   └── vectra/            # Python package
├── playground/             # Next.js web interface
├── Dockerfile              # Container configuration
├── docker-compose.yml      # Development services
├── Makefile                # Development commands
└── README.md               # This file
```

### Common Commands
```bash
# Build & test
make build                 # Build Rust application
make test                  # Run tests
make format                # Format code
make lint                  # Lint code

# Docker operations
make docker-run            # Start all services
make docker-stop           # Stop services
make docker-clean          # Clean up

# Development
make dev                   # Run Rust backend
make playground-dev        # Run web interface
make start                 # Quick start with Docker
```

---

## 🔧 Configuration

### Environment Variables
```bash
# Server
VECTRA_HOST=0.0.0.0
VECTRA_PORT=8080
VECTRA_WORKERS=4

# Storage
VECTRA_DATA_DIR=./data
VECTRA_ROCKSDB_PATH=./data/rocksdb
VECTRA_SLED_PATH=./data/sled

# Vector Search
VECTRA_VECTOR_DIMENSION=384
VECTRA_HNSW_M=16
VECTRA_HNSW_EF_CONSTRUCTION=200

# AI Models
VECTRA_OLLAMA_URL=http://localhost:11434
VECTRA_EMBEDDING_MODEL=text-embedding-ada-002

# Streaming
VECTRA_REDPANDA_BROKERS=localhost:9092
VECTRA_KAFKA_COMPATIBILITY=true

# Logging
VECTRA_LOG_LEVEL=info
```

### Configuration Files
```toml
# config/vectra.toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[storage]
data_dir = "./data"
max_memory_mb = 1024
compression = true

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

---

## 🧪 Testing

### Rust Tests
```bash
# Run all tests
cargo test

# Run specific module
cargo test vector

# Run with output
cargo test -- --nocapture
```

### Python Tests
```bash
cd python
pip install -e ".[dev]"
pytest
```

### Integration Tests
```bash
# Start services
make docker-run

# Run tests against running services
cargo test --features integration
```

---

## 🚀 Deployment

### Production Docker
```bash
# Build production image
make prod

# Run with custom config
docker run -d \
  --name vectra \
  -p 8080:8080 \
  -p 6432:6432 \
  -v /data:/app/data \
  -e VECTRA_LOG_LEVEL=warn \
  vectraedge/vectra:latest
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: vectra-edge
spec:
  replicas: 3
  selector:
    matchLabels:
      app: vectra-edge
  template:
    metadata:
      labels:
        app: vectra-edge
    spec:
      containers:
      - name: vectra
        image: vectraedge/vectra:latest
        ports:
        - containerPort: 8080
        - containerPort: 6432
        env:
        - name: VECTRA_HOST
          value: "0.0.0.0"
        - name: VECTRA_PORT
          value: "8080"
        volumeMounts:
        - name: data
          mountPath: /app/data
      volumes:
      - name: data
        persistentVolumeClaim:
          claimName: vectra-data
```

---

## 🤝 Contributing

We ❤️ contributions! Here's how to get started:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Guidelines
- Follow Rust coding standards
- Add tests for new functionality
- Update documentation
- Use conventional commit messages

### Good First Issues
- [ ] Add more vector distance metrics
- [ ] Implement additional storage backends
- [ ] Add more AI model integrations
- [ ] Improve playground UI components
- [ ] Add performance benchmarks

---

## 📄 License

This project is licensed under both MIT and Apache 2.0 licenses:

- **MIT License** - For SDKs and client libraries
- **Apache 2.0** - For the core engine

See [LICENSE](LICENSE) for details.

---

## 🙏 Acknowledgments

- **DataFusion** - Apache Arrow-based query engine
- **Redpanda** - Kafka-compatible streaming platform
- **Ollama** - Local LLM inference
- **pgvector** - PostgreSQL vector extension
- **RocksDB** - High-performance storage engine

---

## 📚 Documentation

- **[Getting Started](docs/getting-started.md)** - Quick setup and first steps
- **[User Guide](docs/user-guide.md)** - Core features and usage examples
- **[API Reference](docs/api-reference.md)** - Complete API documentation
- **[Deployment Guide](docs/deployment.md)** - Production deployment guides
- **[Contributing](docs/contributing.md)** - How to contribute to VectraEdge

## 📞 Support

- **Documentation**: [Project Docs](docs/) - Comprehensive guides and references
- **Issues**: [GitHub Issues](https://github.com/makalin/vectraedge/issues)
- **Discussions**: [GitHub Discussions](https://github.com/makalin/vectraedge/discussions)
- **Email**: makalin@gmail.com

---

**Ready to build the future of AI-native analytics?** 🚀

Clone, build, and you have a **DuckDB++ with built-in AI & streaming** ready for 2025.
