# VectraEdge  
*An in-process, AI-native, real-time OLAP engine.*  
DuckDB-class performance with built-in vector search & streaming CDC.

![CI](https://github.com/vectraedge/vectra/workflows/CI/badge.svg)
![Crates.io](https://img.shields.io/crates/v/vectra?label=crates)
![PyPI](https://img.shields.io/pypi/v/vectra?label=pypi)
![Docker](https://img.shields.io/docker/v/vectraedge/vectra?color=blue&label=docker)
![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)

---

## 📦 Install

### Docker (fastest)
```bash
docker run -d --name vectra -p 6432:6432 -p 8080:8080 vectraedge/vectra:latest
```

### Cargo (Rust)
```bash
cargo install vectra
```

### Pip (Python)
```bash
pip install vectra
```

---

## 🚀 Quick Start

1. **Connect**  
   ```bash
   psql postgresql://vectra@localhost:6432/vectra
   ```

2. **Create a table with a vector column**
   ```sql
   CREATE TABLE docs (
     id  SERIAL PRIMARY KEY,
     txt TEXT,
     emb VECTOR(384)
   );
   ```

3. **Insert & index**
   ```sql
   INSERT INTO docs(txt, emb)
   VALUES ('hello world', ai_embedding('hello world'));

   CREATE INDEX ON docs USING hnsw (emb);
   ```

4. **Vector search**
   ```sql
   SELECT id, txt
   FROM docs
   ORDER BY emb <-> ai_embedding('hi')
   LIMIT 3;
   ```

---

## 🔄 Real-time CDC

1. **Produce changes**  
   ```bash
   redpanda topic produce vectra_binlog
   ```

2. **Subscribe from SQL**  
   ```sql
   SELECT * FROM stream('vectra_binlog') WHERE op = 'insert';
   ```

---

## 🧩 Language Bindings

| Language | Package | Example |
|----------|---------|---------|
| Rust     | crates.io/vectra | `examples/rust` |
| Python   | PyPI: vectra      | `examples/python` |
| JS/TS    | npm: vectra-node  | `examples/nodejs` |

---

## 🏗️ Build from Source

```bash
git clone https://github.com/makalin/vectraedge/vectra.git
cd vectra
just build-release   # requires just & cargo
```

---

## 📚 Docs

* [Architecture](docs/architecture.md)
* [SQL Reference](docs/sql.md)
* [Deployment](docs/deploy.md)

---

## 🤝 Contributing

We ❤️ contributions!  
See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines and good first issues.

---

## 📄 License

MIT licensed.
