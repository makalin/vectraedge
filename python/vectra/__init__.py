"""
VectraEdge Python Package

AI-Native OLAP Engine with vector search and streaming capabilities.
"""

from .client import (
    VectraClient,
    connect,
    quick_query,
    quick_search,
    MockStreamSubscription,
    MockVectorIndex,
)

# Try to import Rust bindings if available
try:
    from . import (
        VectraClient as RustVectraClient,
        VectorIndex,
        StreamSubscription,
        health_check,
        version,
    )
    __all__ = [
        "VectraClient",
        "RustVectraClient", 
        "VectorIndex",
        "StreamSubscription",
        "connect",
        "quick_query",
        "quick_search",
        "health_check",
        "version",
        "MockStreamSubscription",
        "MockVectorIndex",
    ]
except ImportError:
    # Rust bindings not available, use HTTP-only mode
    __all__ = [
        "VectraClient",
        "connect",
        "quick_query", 
        "quick_search",
        "MockStreamSubscription",
        "MockVectorIndex",
    ]

__version__ = "0.1.0"
__author__ = "VectraEdge Team"
__email__ = "team@vectraedge.com"
__description__ = "AI-Native OLAP Engine with vector search and streaming capabilities"
__url__ = "https://github.com/vectraedge/vectra"
__license__ = "MIT OR Apache-2.0"
