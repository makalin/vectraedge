"""
VectraEdge Python Client

Main client interface for the VectraEdge AI-Native OLAP Engine.
"""

import json
import requests
from typing import Optional, Dict, Any, List, Union
from . import VectraClient as RustVectraClient, VectorIndex, StreamSubscription


class VectraClient:
    """
    Python client for VectraEdge.
    
    This client provides a high-level interface to the VectraEdge
    AI-Native OLAP Engine, including SQL execution, vector search,
    and streaming capabilities.
    """
    
    def __init__(self, host: str = "127.0.0.1", port: int = 8080, 
                 use_rust: bool = False):
        """
        Initialize the VectraEdge client.
        
        Args:
            host: VectraEdge server host
            port: VectraEdge server port
            use_rust: Whether to use Rust bindings (default: False)
        """
        self.host = host
        self.port = port
        self.base_url = f"http://{host}:{port}"
        self.use_rust = use_rust
        
        if use_rust:
            self._rust_client = RustVectraClient(host, port)
        else:
            self._rust_client = None
    
    def execute_query(self, sql: str) -> Dict[str, Any]:
        """
        Execute a SQL query.
        
        Args:
            sql: SQL query string
            
        Returns:
            Query results as a dictionary
        """
        if self.use_rust and self._rust_client:
            return self._rust_client.execute_query(sql)
        
        # HTTP implementation
        url = f"{self.base_url}/query"
        payload = {"query": sql}
        
        try:
            response = requests.post(url, json=payload, timeout=30)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            raise RuntimeError(f"Failed to execute query: {e}")
    
    def vector_search(self, query: str, limit: int = 10) -> Dict[str, Any]:
        """
        Perform vector search.
        
        Args:
            query: Query text for vector search
            limit: Maximum number of results
            
        Returns:
            Search results as a dictionary
        """
        if self.use_rust and self._rust_client:
            return self._rust_client.vector_search(query, limit)
        
        # HTTP implementation
        url = f"{self.base_url}/vector/search"
        payload = {"query": query, "limit": limit}
        
        try:
            response = requests.post(url, json=payload, timeout=30)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            raise RuntimeError(f"Failed to perform vector search: {e}")
    
    def subscribe_stream(self, topic: str) -> StreamSubscription:
        """
        Subscribe to a stream topic.
        
        Args:
            topic: Topic name to subscribe to
            
        Returns:
            Stream subscription object
        """
        if self.use_rust and self._rust_client:
            return self._rust_client.subscribe_stream(topic)
        
        # HTTP implementation
        url = f"{self.base_url}/stream/subscribe"
        payload = {"topic": topic}
        
        try:
            response = requests.post(url, json=payload, timeout=30)
            response.raise_for_status()
            data = response.json()
            
            # Create a mock subscription object
            return MockStreamSubscription(
                id=data.get("subscription_id", "unknown"),
                topic=topic,
                status=data.get("status", "unknown")
            )
        except requests.exceptions.RequestException as e:
            raise RuntimeError(f"Failed to subscribe to stream: {e}")
    
    def create_table(self, name: str, schema: str) -> None:
        """
        Create a new table.
        
        Args:
            name: Table name
            schema: Table schema definition
        """
        if self.use_rust and self._rust_client:
            return self._rust_client.create_table(name, schema)
        
        # HTTP implementation would go here
        print(f"Creating table '{name}' with schema: {schema}")
    
    def insert_data(self, table: str, data: Union[Dict[str, Any], List[Dict[str, Any]]]) -> None:
        """
        Insert data into a table.
        
        Args:
            table: Table name
            data: Data to insert (dict or list of dicts)
        """
        if self.use_rust and self._rust_client:
            return self._rust_client.insert_data(table, data)
        
        # HTTP implementation would go here
        print(f"Inserting data into table '{table}': {data}")
    
    def create_vector_index(self, table: str, column: str) -> VectorIndex:
        """
        Create a vector index on a table column.
        
        Args:
            table: Table name
            column: Column name
            
        Returns:
            Vector index object
        """
        if self.use_rust and self._rust_client:
            return self._rust_client.create_vector_index(table, column)
        
        # HTTP implementation would go here
        print(f"Creating vector index on {table}.{column}")
        return MockVectorIndex(table, column)
    
    def list_tables(self) -> List[str]:
        """
        List all tables.
        
        Returns:
            List of table names
        """
        if self.use_rust and self._rust_client:
            return self._rust_client.list_tables()
        
        # HTTP implementation would go here
        return ["docs", "users", "products"]  # Mock data
    
    def get_table_info(self, table: str) -> Dict[str, Any]:
        """
        Get information about a table.
        
        Args:
            table: Table name
            
        Returns:
            Table information as a dictionary
        """
        if self.use_rust and self._rust_client:
            return self._rust_client.get_table_info(table)
        
        # HTTP implementation would go here
        return {
            "name": table,
            "rows": 1000,
            "size_bytes": 1024000,
            "created_at": "2024-01-01T00:00:00Z"
        }
    
    def get_stats(self) -> Dict[str, Any]:
        """
        Get storage statistics.
        
        Returns:
            Storage statistics as a dictionary
        """
        if self.use_rust and self._rust_client:
            return self._rust_client.get_stats()
        
        # HTTP implementation would go here
        return {
            "total_tables": 3,
            "total_rows": 5000,
            "total_size_bytes": 5120000
        }
    
    def health_check(self) -> Dict[str, Any]:
        """
        Perform a health check.
        
        Returns:
            Health status as a dictionary
        """
        url = f"{self.base_url}/health"
        
        try:
            response = requests.get(url, timeout=10)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            raise RuntimeError(f"Health check failed: {e}")


class MockStreamSubscription:
    """Mock stream subscription for HTTP-only mode."""
    
    def __init__(self, id: str, topic: str, status: str):
        self.id = id
        self.topic = topic
        self.status = status
    
    def get_id(self) -> str:
        return self.id
    
    def get_topic(self) -> str:
        return self.topic
    
    def get_status(self) -> str:
        return self.status
    
    def unsubscribe(self) -> None:
        print(f"Unsubscribing from topic: {self.topic}")


class MockVectorIndex:
    """Mock vector index for HTTP-only mode."""
    
    def __init__(self, table_name: str, column_name: str):
        self.table_name = table_name
        self.column_name = column_name
    
    def insert_vector(self, id: int, vector: List[float]) -> None:
        print(f"Inserting vector {id} into index {self.table_name}.{self.column_name}")
    
    def search(self, query_vector: List[float], limit: Optional[int] = None) -> Dict[str, Any]:
        limit = limit or 10
        return {
            "results": [
                {"id": 1, "score": 0.95, "metadata": {"text": "Sample result"}},
                {"id": 2, "score": 0.87, "metadata": {"text": "Another result"}}
            ],
            "limit": limit
        }
    
    def delete_index(self) -> None:
        print(f"Deleting index on {self.table_name}.{self.column_name}")


# Convenience functions
def connect(host: str = "127.0.0.1", port: int = 8080, 
            use_rust: bool = False) -> VectraClient:
    """
    Create a VectraEdge client connection.
    
    Args:
        host: VectraEdge server host
        port: VectraEdge server port
        use_rust: Whether to use Rust bindings
        
    Returns:
        VectraClient instance
    """
    return VectraClient(host=host, port=port, use_rust=use_rust)


def quick_query(sql: str, host: str = "127.0.0.1", port: int = 8080) -> Dict[str, Any]:
    """
    Execute a quick SQL query.
    
    Args:
        sql: SQL query string
        host: VectraEdge server host
        port: VectraEdge server port
        
    Returns:
        Query results
    """
    client = VectraClient(host=host, port=port)
    return client.execute_query(sql)


def quick_search(query: str, limit: int = 10, host: str = "127.0.0.1", 
                 port: int = 8080) -> Dict[str, Any]:
    """
    Perform a quick vector search.
    
    Args:
        query: Query text for vector search
        limit: Maximum number of results
        host: VectraEdge server host
        port: VectraEdge server port
        
    Returns:
        Search results
    """
    client = VectraClient(host=host, port=port)
    return client.vector_search(query, limit)
