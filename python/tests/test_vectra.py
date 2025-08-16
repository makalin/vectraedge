"""
Comprehensive tests for VectraEdge Python bindings.
"""

import pytest
import asyncio
import json
from unittest.mock import Mock, patch
import sys
import os

# Add the parent directory to the path so we can import vectra
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from vectra import VectraClient, VectorIndex, StreamSubscription
from vectra.client import MockStreamSubscription, MockVectorIndex


class TestVectraClient:
    """Test the main VectraClient class."""
    
    def test_client_initialization(self):
        """Test client initialization with default and custom parameters."""
        # Test default initialization
        client = VectraClient()
        assert client.host == "127.0.0.1"
        assert client.port == 8080
        assert client.use_rust is False
        
        # Test custom initialization
        client = VectraClient(host="localhost", port=9000, use_rust=True)
        assert client.host == "localhost"
        assert client.port == 9000
        assert client.use_rust is True
    
    def test_client_initialization_with_rust(self):
        """Test client initialization with Rust bindings."""
        try:
            client = VectraClient(use_rust=True)
            assert client._rust_client is not None
        except ImportError:
            # Rust bindings not available, skip test
            pytest.skip("Rust bindings not available")
    
    @patch('vectra.client.requests.post')
    def test_execute_query_http(self, mock_post):
        """Test SQL query execution via HTTP."""
        # Mock successful response
        mock_response = Mock()
        mock_response.json.return_value = {"rows": 1, "status": "success"}
        mock_response.raise_for_status.return_value = None
        mock_post.return_value = mock_response
        
        client = VectraClient(host="localhost", port=8080)
        result = client.execute_query("SELECT * FROM test")
        
        assert result == {"rows": 1, "status": "success"}
        mock_post.assert_called_once()
    
    @patch('vectra.client.requests.post')
    def test_vector_search_http(self, mock_post):
        """Test vector search via HTTP."""
        # Mock successful response
        mock_response = Mock()
        mock_response.json.return_value = {
            "results": [{"id": 1, "score": 0.95}],
            "query": "test query",
            "limit": 10
        }
        mock_response.raise_for_status.return_value = None
        mock_post.return_value = mock_response
        
        client = VectraClient(host="localhost", port=8080)
        result = client.vector_search("test query", 10)
        
        assert result["query"] == "test query"
        assert result["limit"] == 10
        assert len(result["results"]) == 1
        mock_post.assert_called_once()
    
    @patch('vectra.client.requests.post')
    def test_subscribe_stream_http(self, mock_post):
        """Test stream subscription via HTTP."""
        # Mock successful response
        mock_response = Mock()
        mock_response.json.return_value = {
            "subscription_id": "sub_123",
            "topic": "test_topic",
            "status": "active"
        }
        mock_response.raise_for_status.return_value = None
        mock_post.return_value = mock_response
        
        client = VectraClient(host="localhost", port=8080)
        subscription = client.subscribe_stream("test_topic")
        
        assert isinstance(subscription, MockStreamSubscription)
        assert subscription.get_topic() == "test_topic"
        assert subscription.get_status() == "active"
        mock_post.assert_called_once()
    
    def test_create_table(self):
        """Test table creation."""
        client = VectraClient()
        
        # This should not raise an exception
        client.create_table("test_table", "id INT, name TEXT")
    
    def test_insert_data(self):
        """Test data insertion."""
        client = VectraClient()
        test_data = {"id": 1, "name": "test"}
        
        # This should not raise an exception
        client.insert_data("test_table", test_data)
    
    def test_create_vector_index(self):
        """Test vector index creation."""
        client = VectraClient()
        index = client.create_vector_index("test_table", "embedding")
        
        assert isinstance(index, MockVectorIndex)
        assert index.table_name == "test_table"
        assert index.column_name == "embedding"
    
    def test_list_tables(self):
        """Test table listing."""
        client = VectraClient()
        tables = client.list_tables()
        
        assert isinstance(tables, list)
        assert len(tables) > 0
        assert all(isinstance(table, str) for table in tables)
    
    def test_get_table_info(self):
        """Test table information retrieval."""
        client = VectraClient()
        info = client.get_table_info("test_table")
        
        assert isinstance(info, dict)
        assert "name" in info
        assert "rows" in info
        assert "size_bytes" in info
    
    def test_get_stats(self):
        """Test statistics retrieval."""
        client = VectraClient()
        stats = client.get_stats()
        
        assert isinstance(stats, dict)
        assert "total_tables" in stats
        assert "total_rows" in stats
        assert "total_size_bytes" in stats
    
    @patch('vectra.client.requests.get')
    def test_health_check(self, mock_get):
        """Test health check."""
        # Mock successful response
        mock_response = Mock()
        mock_response.json.return_value = {"status": "healthy"}
        mock_response.raise_for_status.return_value = None
        mock_get.return_value = mock_response
        
        client = VectraClient(host="localhost", port=8080)
        health = client.health_check()
        
        assert health["status"] == "healthy"
        mock_get.assert_called_once()


class TestMockStreamSubscription:
    """Test the MockStreamSubscription class."""
    
    def test_subscription_creation(self):
        """Test subscription object creation."""
        subscription = MockStreamSubscription("sub_123", "test_topic", "active")
        
        assert subscription.get_id() == "sub_123"
        assert subscription.get_topic() == "test_topic"
        assert subscription.get_status() == "active"
    
    def test_unsubscribe(self):
        """Test unsubscription."""
        subscription = MockStreamSubscription("sub_123", "test_topic", "active")
        
        # This should not raise an exception
        subscription.unsubscribe()


class TestMockVectorIndex:
    """Test the MockVectorIndex class."""
    
    def test_index_creation(self):
        """Test index object creation."""
        index = MockVectorIndex("test_table", "embedding")
        
        assert index.table_name == "test_table"
        assert index.column_name == "embedding"
    
    def test_insert_vector(self):
        """Test vector insertion."""
        index = MockVectorIndex("test_table", "embedding")
        
        # This should not raise an exception
        index.insert_vector(1, [0.1, 0.2, 0.3])
    
    def test_search(self):
        """Test vector search."""
        index = MockVectorIndex("test_table", "embedding")
        query_vector = [0.1, 0.2, 0.3]
        
        results = index.search(query_vector, 5)
        
        assert isinstance(results, dict)
        assert "results" in results
        assert "limit" in results
        assert results["limit"] == 5
        assert len(results["results"]) > 0
    
    def test_delete_index(self):
        """Test index deletion."""
        index = MockVectorIndex("test_table", "embedding")
        
        # This should not raise an exception
        index.delete_index()


class TestVectorIndex:
    """Test the VectorIndex class (if Rust bindings available)."""
    
    def test_vector_index_creation(self):
        """Test VectorIndex creation."""
        try:
            index = VectorIndex("test_table", "embedding")
            assert index.table_name == "test_table"
            assert index.column_name == "embedding"
        except (ImportError, AttributeError):
            pytest.skip("VectorIndex not available")
    
    def test_insert_vector(self):
        """Test vector insertion."""
        try:
            index = VectorIndex("test_table", "embedding")
            index.insert_vector(1, [0.1, 0.2, 0.3])
        except (ImportError, AttributeError):
            pytest.skip("VectorIndex not available")
    
    def test_search(self):
        """Test vector search."""
        try:
            index = VectorIndex("test_table", "embedding")
            query_vector = [0.1, 0.2, 0.3]
            results = index.search(query_vector, 5)
            
            assert isinstance(results, dict)
            assert "results" in results
        except (ImportError, AttributeError):
            pytest.skip("VectorIndex not available")


class TestStreamSubscription:
    """Test the StreamSubscription class (if Rust bindings available)."""
    
    def test_subscription_creation(self):
        """Test StreamSubscription creation."""
        try:
            subscription = StreamSubscription("sub_123", "test_topic", "active")
            assert subscription.get_id() == "sub_123"
            assert subscription.get_topic() == "test_topic"
            assert subscription.get_status() == "active"
        except (ImportError, AttributeError):
            pytest.skip("StreamSubscription not available")
    
    def test_unsubscribe(self):
        """Test unsubscription."""
        try:
            subscription = StreamSubscription("sub_123", "test_topic", "active")
            subscription.unsubscribe()
        except (ImportError, AttributeError):
            pytest.skip("StreamSubscription not available")


class TestConvenienceFunctions:
    """Test convenience functions."""
    
    def test_connect_function(self):
        """Test the connect convenience function."""
        from vectra.client import connect
        
        client = connect(host="localhost", port=9000)
        assert client.host == "localhost"
        assert client.port == 9000
        assert client.use_rust is False
    
    def test_quick_query_function(self):
        """Test the quick_query convenience function."""
        from vectra.client import quick_query
        
        # This will fail without a running server, but we can test the function exists
        assert callable(quick_query)
    
    def test_quick_search_function(self):
        """Test the quick_search convenience function."""
        from vectra.client import quick_search
        
        # This will fail without a running server, but we can test the function exists
        assert callable(quick_search)


class TestErrorHandling:
    """Test error handling scenarios."""
    
    @patch('vectra.client.requests.post')
    def test_network_error_handling(self, mock_post):
        """Test handling of network errors."""
        from requests.exceptions import RequestException
        
        # Mock network error
        mock_post.side_effect = RequestException("Network error")
        
        client = VectraClient(host="localhost", port=8080)
        
        with pytest.raises(RuntimeError) as exc_info:
            client.execute_query("SELECT * FROM test")
        
        assert "Failed to execute query" in str(exc_info.value)
    
    @patch('vectra.client.requests.post')
    def test_http_error_handling(self, mock_post):
        """Test handling of HTTP errors."""
        from requests.exceptions import HTTPError
        
        # Mock HTTP error
        mock_response = Mock()
        mock_response.raise_for_status.side_effect = HTTPError("404 Not Found")
        mock_post.return_value = mock_response
        
        client = VectraClient(host="localhost", port=8080)
        
        with pytest.raises(RuntimeError) as exc_info:
            client.execute_query("SELECT * FROM test")
        
        assert "Failed to execute query" in str(exc_info.value)


class TestConfiguration:
    """Test configuration and environment handling."""
    
    def test_default_configuration(self):
        """Test default configuration values."""
        client = VectraClient()
        
        assert client.host == "127.0.0.1"
        assert client.port == 8080
        assert client.base_url == "http://127.0.0.1:8080"
    
    def test_custom_configuration(self):
        """Test custom configuration values."""
        client = VectraClient(host="custom.host", port=12345)
        
        assert client.host == "custom.host"
        assert client.port == 12345
        assert client.base_url == "http://custom.host:12345"


class TestDataTypes:
    """Test handling of different data types."""
    
    def test_json_data_handling(self):
        """Test JSON data handling."""
        client = VectraClient()
        
        # Test various data types
        test_cases = [
            {"string": "test"},
            {"number": 42},
            {"float": 3.14},
            {"boolean": True},
            {"null": None},
            {"array": [1, 2, 3]},
            {"nested": {"key": "value"}}
        ]
        
        for test_data in test_cases:
            # This should not raise an exception
            client.insert_data("test_table", test_data)
    
    def test_vector_data_handling(self):
        """Test vector data handling."""
        client = VectraClient()
        
        # Test vector data
        vector_data = {
            "id": 1,
            "embedding": [0.1, 0.2, 0.3, 0.4, 0.5]
        }
        
        # This should not raise an exception
        client.insert_data("test_table", vector_data)


if __name__ == "__main__":
    pytest.main([__file__])
