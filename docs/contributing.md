# Contributing to VectraEdge

Thank you for your interest in contributing to VectraEdge! This guide will help you get started with contributing to the project.

## 🤝 How to Contribute

### Types of Contributions

We welcome various types of contributions:

- **🐛 Bug Reports**: Report bugs and issues
- **💡 Feature Requests**: Suggest new features
- **📝 Documentation**: Improve docs and examples
- **🔧 Code Changes**: Fix bugs, add features, improve performance
- **🧪 Testing**: Write tests, report test results
- **📊 Benchmarks**: Performance testing and optimization
- **🌐 Localization**: Translate documentation
- **💬 Community**: Help other users, answer questions

### Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork locally
3. **Create** a feature branch
4. **Make** your changes
5. **Test** your changes
6. **Commit** with a clear message
7. **Push** to your fork
8. **Open** a Pull Request

## 🛠️ Development Setup

### Prerequisites

- **Rust 1.75+** - For core development
- **Python 3.8+** - For Python bindings
- **Node.js 18+** - For web playground
- **Docker & Docker Compose** - For testing services
- **Git** - For version control

### Local Development Environment

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/vectraedge.git
cd vectraedge

# Add upstream remote
git remote add upstream https://github.com/makalin/vectraedge.git

# Install development dependencies
make setup

# Start development services
make dev
```

### Project Structure

```
vectraedge/
├── src/                    # Rust source code
│   ├── main.rs            # Application entry point
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
├── docs/                   # Documentation
├── tests/                  # Test suites
├── scripts/                # Build and utility scripts
└── examples/               # Code examples
```

## 🔧 Development Workflow

### Making Changes

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/amazing-feature
   ```

2. **Make your changes** following the coding standards

3. **Test your changes**:
   ```bash
   # Run Rust tests
   make test
   
   # Run Python tests
   cd python && pytest
   
   # Run integration tests
   make integration-test
   ```

4. **Format and lint**:
   ```bash
   # Format Rust code
   make format
   
   # Lint Rust code
   make lint
   
   # Format Python code
   cd python && black . && isort .
   ```

5. **Commit your changes**:
   ```bash
   git add .
   git commit -m "feat: add amazing feature"
   ```

### Commit Message Format

We use [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```bash
git commit -m "feat: add HNSW vector index support"
git commit -m "fix: resolve memory leak in streaming operations"
git commit -m "docs: update API reference with new endpoints"
git commit -m "test: add integration tests for vector search"
```

### Pull Request Process

1. **Update your branch** with upstream changes:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Push your changes**:
   ```bash
   git push origin feature/amazing-feature
   ```

3. **Create a Pull Request** on GitHub with:
   - Clear title describing the change
   - Detailed description of what was changed and why
   - Link to related issues
   - Screenshots for UI changes
   - Test results and performance impact

4. **Address review feedback** and make requested changes

5. **Squash commits** if requested during review

## 📝 Coding Standards

### Rust Code

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write comprehensive tests
- Add documentation comments for public APIs

**Example:**
```rust
/// Performs vector similarity search using HNSW indexing.
///
/// # Arguments
///
/// * `query` - The query vector to search for
/// * `limit` - Maximum number of results to return
/// * `distance_threshold` - Maximum distance threshold for results
///
/// # Returns
///
/// A vector of search results with similarity scores.
///
/// # Examples
///
/// ```
/// use vectra::vector_search;
///
/// let results = vector_search(&query_vector, 10, Some(0.5))?;
/// ```
pub fn vector_search(
    query: &[f32],
    limit: usize,
    distance_threshold: Option<f32>,
) -> Result<Vec<SearchResult>, VectraError> {
    // Implementation...
}
```

### Python Code

- Follow [PEP 8](https://pep8.org/) style guide
- Use type hints for all functions
- Write docstrings for all public functions
- Use `black` for formatting and `isort` for imports

**Example:**
```python
from typing import List, Optional, Dict, Any
from vectra.types import SearchResult, VectraError

def vector_search(
    query: str,
    table: str,
    limit: int = 10,
    distance_threshold: Optional[float] = None,
    filters: Optional[Dict[str, Any]] = None,
) -> List[SearchResult]:
    """
    Perform vector similarity search.
    
    Args:
        query: Search query text
        table: Table to search in
        limit: Maximum number of results
        distance_threshold: Maximum distance threshold
        filters: Additional filters to apply
        
    Returns:
        List of search results with similarity scores
        
    Raises:
        VectraError: If search operation fails
        
    Example:
        >>> results = vector_search("machine learning", "documents", limit=5)
        >>> for result in results:
        ...     print(f"Score: {result.score}, Title: {result.data['title']}")
    """
    # Implementation...
```

### Documentation

- Write clear, concise documentation
- Include code examples
- Keep documentation up-to-date with code changes
- Use proper markdown formatting

## 🧪 Testing

### Writing Tests

- Write tests for all new functionality
- Include both unit tests and integration tests
- Test edge cases and error conditions
- Aim for high test coverage

**Rust Test Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_search_basic() {
        let query = vec![0.1, 0.2, 0.3];
        let results = vector_search(&query, 5, None).unwrap();
        assert_eq!(results.len(), 5);
    }

    #[test]
    fn test_vector_search_with_threshold() {
        let query = vec![0.1, 0.2, 0.3];
        let results = vector_search(&query, 10, Some(0.5)).unwrap();
        assert!(results.iter().all(|r| r.score <= 0.5));
    }

    #[test]
    fn test_vector_search_empty_result() {
        let query = vec![0.1, 0.2, 0.3];
        let results = vector_search(&query, 5, Some(0.01)).unwrap();
        assert_eq!(results.len(), 0);
    }
}
```

**Python Test Example:**
```python
import pytest
from vectra import VectraClient
from vectra.exceptions import VectraError

class TestVectorSearch:
    def test_basic_search(self, client: VectraClient):
        """Test basic vector search functionality."""
        results = client.vector_search("test query", "documents", limit=5)
        assert isinstance(results, list)
        assert len(results) <= 5
        
    def test_search_with_filters(self, client: VectraClient):
        """Test vector search with additional filters."""
        filters = {"category": "tutorial"}
        results = client.vector_search("AI", "documents", filters=filters)
        assert all(r.data.get("category") == "tutorial" for r in results)
        
    def test_invalid_table(self, client: VectraClient):
        """Test error handling for invalid table."""
        with pytest.raises(VectraError, match="Table not found"):
            client.vector_search("query", "nonexistent_table")
```

### Running Tests

```bash
# Run all tests
make test

# Run specific test suite
cargo test vector
cargo test --test integration_tests

# Run with coverage
cargo tarpaulin --out Html

# Run Python tests
cd python && pytest --cov=vectra

# Run integration tests
make integration-test
```

## 📊 Performance & Benchmarks

### Writing Benchmarks

- Use `criterion` for Rust benchmarks
- Benchmark critical paths and hot code
- Include memory usage measurements
- Compare against baseline implementations

**Benchmark Example:**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vectra::vector_search;

fn benchmark_vector_search(c: &mut Criterion) {
    let query = vec![0.1; 384];
    let mut group = c.benchmark_group("vector_search");
    
    group.bench_function("hnsw_search", |b| {
        b.iter(|| vector_search(black_box(&query), 10, None))
    });
    
    group.bench_function("hnsw_search_with_threshold", |b| {
        b.iter(|| vector_search(black_box(&query), 10, Some(0.5)))
    });
}

criterion_group!(benches, benchmark_vector_search);
criterion_main!(benches);
```

### Running Benchmarks

```bash
# Run Rust benchmarks
cargo bench

# Run specific benchmarks
cargo bench --bench vector_benchmarks

# Run performance tests
make benchmark
```

## 🔍 Code Review Process

### Review Checklist

Before submitting a PR, ensure:

- [ ] Code follows project style guidelines
- [ ] All tests pass
- [ ] Documentation is updated
- [ ] No performance regressions
- [ ] Error handling is appropriate
- [ ] Security considerations addressed
- [ ] Backward compatibility maintained

### Review Guidelines

- Be constructive and respectful
- Focus on code quality and correctness
- Suggest improvements when possible
- Ask questions for clarification
- Consider security implications
- Check for performance issues

## 🚀 Release Process

### Versioning

We use [Semantic Versioning](https://semver.org/):

- **MAJOR**: Incompatible API changes
- **MINOR**: New functionality in backward-compatible manner
- **PATCH**: Backward-compatible bug fixes

### Release Checklist

- [ ] All tests passing
- [ ] Documentation updated
- [ ] Changelog updated
- [ ] Version bumped
- [ ] Release notes written
- [ ] GitHub release created
- [ ] Docker images tagged
- [ ] PyPI package published

## 🆘 Getting Help

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Questions and community discussions
- **Email**: makalin@gmail.com for direct support
- **Documentation**: Check the [docs](/) first

### Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Python Documentation](https://docs.python.org/)
- [DataFusion Documentation](https://docs.rs/datafusion/)
- [Apache Arrow Documentation](https://arrow.apache.org/docs/)

## 🙏 Recognition

Contributors will be recognized in:

- GitHub contributors list
- Project README
- Release notes
- Project documentation

## 📄 License

By contributing to VectraEdge, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache 2.0).

---

**Thank you for contributing to VectraEdge!** 🚀

Your contributions help make VectraEdge better for everyone in the community.
