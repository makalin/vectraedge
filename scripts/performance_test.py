#!/usr/bin/env python3
"""
Performance testing script for VectraEdge.

This script runs various performance benchmarks to test the engine's capabilities.
"""

import time
import statistics
import json
import argparse
import sys
import os
from typing import Dict, List, Any
from concurrent.futures import ThreadPoolExecutor, as_completed
import threading

# Add the parent directory to the path so we can import vectra
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

try:
    from vectra import VectraClient
except ImportError:
    print("Warning: VectraEdge Python bindings not available. Using mock client.")
    VectraClient = None


class PerformanceTester:
    """Performance testing framework for VectraEdge."""
    
    def __init__(self, host: str = "localhost", port: int = 8080, use_rust: bool = False):
        self.host = host
        self.port = port
        self.use_rust = use_rust
        self.results = {}
        
        if VectraClient:
            self.client = VectraClient(host=host, port=port, use_rust=use_rust)
        else:
            self.client = None
            print("Using mock client for testing")
    
    def run_all_tests(self) -> Dict[str, Any]:
        """Run all performance tests."""
        print("🚀 Starting VectraEdge Performance Tests")
        print("=" * 50)
        
        # Test 1: Connection and basic operations
        self.test_connection_performance()
        
        # Test 2: Table operations
        self.test_table_operations()
        
        # Test 3: Data insertion performance
        self.test_data_insertion_performance()
        
        # Test 4: Query performance
        self.test_query_performance()
        
        # Test 5: Vector search performance
        self.test_vector_search_performance()
        
        # Test 6: Concurrent operations
        self.test_concurrent_operations()
        
        # Test 7: Memory usage
        self.test_memory_usage()
        
        # Test 8: Stress testing
        self.test_stress_scenarios()
        
        print("\n" + "=" * 50)
        print("✅ All performance tests completed!")
        
        return self.results
    
    def test_connection_performance(self):
        """Test connection establishment performance."""
        print("\n🔌 Testing Connection Performance...")
        
        if not self.client:
            print("  Skipped (mock client)")
            return
        
        connection_times = []
        
        for i in range(10):
            start_time = time.time()
            try:
                # Test health check as connection test
                health = self.client.health_check()
                connection_time = (time.time() - start_time) * 1000  # Convert to ms
                connection_times.append(connection_time)
            except Exception as e:
                print(f"  Connection {i+1} failed: {e}")
        
        if connection_times:
            avg_time = statistics.mean(connection_times)
            min_time = min(connection_times)
            max_time = max(connection_times)
            
            self.results['connection'] = {
                'avg_time_ms': avg_time,
                'min_time_ms': min_time,
                'max_time_ms': max_time,
                'samples': len(connection_times)
            }
            
            print(f"  Average connection time: {avg_time:.2f}ms")
            print(f"  Min/Max: {min_time:.2f}ms / {max_time:.2f}ms")
    
    def test_table_operations(self):
        """Test table creation and management performance."""
        print("\n📊 Testing Table Operations...")
        
        if not self.client:
            print("  Skipped (mock client)")
            return
        
        # Test table creation
        table_creation_times = []
        table_names = []
        
        for i in range(5):
            table_name = f"perf_test_table_{i}"
            table_names.append(table_name)
            
            start_time = time.time()
            try:
                self.client.create_table(table_name, "id INT, name TEXT, data TEXT")
                creation_time = (time.time() - start_time) * 1000
                table_creation_times.append(creation_time)
            except Exception as e:
                print(f"  Table creation {i+1} failed: {e}")
        
        if table_creation_times:
            avg_time = statistics.mean(table_creation_times)
            self.results['table_creation'] = {
                'avg_time_ms': avg_time,
                'samples': len(table_creation_times)
            }
            print(f"  Average table creation time: {avg_time:.2f}ms")
        
        # Clean up tables
        for table_name in table_names:
            try:
                # Note: drop_table not implemented in mock client
                pass
            except Exception:
                pass
    
    def test_data_insertion_performance(self):
        """Test data insertion performance."""
        print("\n📝 Testing Data Insertion Performance...")
        
        if not self.client:
            print("  Skipped (mock client)")
            return
        
        # Test different data sizes
        data_sizes = [100, 1000, 10000]  # bytes
        
        for size in data_sizes:
            insertion_times = []
            
            # Generate test data of specified size
            test_data = self._generate_test_data(size)
            
            for i in range(10):
                start_time = time.time()
                try:
                    self.client.insert_data("perf_test_table", f"key_{size}_{i}", test_data)
                    insertion_time = (time.time() - start_time) * 1000
                    insertion_times.append(insertion_time)
                except Exception as e:
                    print(f"  Data insertion {i+1} failed: {e}")
            
            if insertion_times:
                avg_time = statistics.mean(insertion_times)
                throughput = (size / 1024) / (avg_time / 1000)  # KB/s
                
                self.results[f'data_insertion_{size}b'] = {
                    'avg_time_ms': avg_time,
                    'throughput_kbs': throughput,
                    'samples': len(insertion_times)
                }
                
                print(f"  {size}B data: {avg_time:.2f}ms avg, {throughput:.2f} KB/s")
    
    def test_query_performance(self):
        """Test query execution performance."""
        print("\n🔍 Testing Query Performance...")
        
        if not self.client:
            print("  Skipped (mock client)")
            return
        
        # Test different query types
        queries = [
            "SELECT * FROM perf_test_table LIMIT 10",
            "SELECT COUNT(*) FROM perf_test_table",
            "SELECT * FROM perf_test_table WHERE id > 5",
        ]
        
        for i, query in enumerate(queries):
            execution_times = []
            
            for j in range(10):
                start_time = time.time()
                try:
                    result = self.client.execute_query(query)
                    execution_time = (time.time() - start_time) * 1000
                    execution_times.append(execution_time)
                except Exception as e:
                    print(f"  Query {i+1} execution {j+1} failed: {e}")
            
            if execution_times:
                avg_time = statistics.mean(execution_times)
                self.results[f'query_{i+1}'] = {
                    'query': query,
                    'avg_time_ms': avg_time,
                    'samples': len(execution_times)
                }
                print(f"  Query {i+1}: {avg_time:.2f}ms avg")
    
    def test_vector_search_performance(self):
        """Test vector search performance."""
        print("\n🧠 Testing Vector Search Performance...")
        
        if not self.client:
            print("  Skipped (mock client)")
            return
        
        # Test different search limits
        search_limits = [5, 10, 20, 50]
        
        for limit in search_limits:
            search_times = []
            
            for i in range(10):
                start_time = time.time()
                try:
                    results = self.client.vector_search("test query", limit)
                    search_time = (time.time() - start_time) * 1000
                    search_times.append(search_time)
                except Exception as e:
                    print(f"  Vector search {i+1} failed: {e}")
            
            if search_times:
                avg_time = statistics.mean(search_times)
                self.results[f'vector_search_{limit}'] = {
                    'limit': limit,
                    'avg_time_ms': avg_time,
                    'samples': len(search_times)
                }
                print(f"  Limit {limit}: {avg_time:.2f}ms avg")
    
    def test_concurrent_operations(self):
        """Test performance under concurrent load."""
        print("\n⚡ Testing Concurrent Operations...")
        
        if not self.client:
            print("  Skipped (mock client)")
            return
        
        # Test different concurrency levels
        concurrency_levels = [5, 10, 20]
        
        for level in concurrency_levels:
            print(f"  Testing {level} concurrent operations...")
            
            start_time = time.time()
            completed_operations = 0
            failed_operations = 0
            
            def worker(worker_id):
                nonlocal completed_operations, failed_operations
                
                try:
                    # Simulate a mix of operations
                    if worker_id % 3 == 0:
                        self.client.execute_query("SELECT * FROM perf_test_table LIMIT 1")
                    elif worker_id % 3 == 1:
                        self.client.vector_search("test", 5)
                    else:
                        self.client.get_stats()
                    
                    completed_operations += 1
                except Exception:
                    failed_operations += 1
            
            with ThreadPoolExecutor(max_workers=level) as executor:
                futures = [executor.submit(worker, i) for i in range(level)]
                for future in as_completed(futures):
                    try:
                        future.result()
                    except Exception:
                        pass
            
            total_time = time.time() - start_time
            throughput = completed_operations / total_time
            
            self.results[f'concurrent_{level}'] = {
                'concurrency_level': level,
                'total_time_s': total_time,
                'completed_operations': completed_operations,
                'failed_operations': failed_operations,
                'throughput_ops_per_sec': throughput
            }
            
            print(f"    Completed: {completed_operations}/{level} operations")
            print(f"    Throughput: {throughput:.2f} ops/sec")
    
    def test_memory_usage(self):
        """Test memory usage patterns."""
        print("\n💾 Testing Memory Usage...")
        
        if not self.client:
            print("  Skipped (mock client)")
            return
        
        # This is a simplified memory test
        # In a real implementation, you'd use psutil or similar
        
        import gc
        gc.collect()
        
        # Simulate memory-intensive operations
        large_data = []
        for i in range(1000):
            large_data.append({
                "id": i,
                "data": "x" * 1000,  # 1KB per item
                "vector": [0.1] * 384  # 384-dim vector
            })
        
        # Test memory usage after operations
        try:
            for i, item in enumerate(large_data[:100]):  # Insert first 100 items
                self.client.insert_data("memory_test_table", f"key_{i}", item)
        except Exception as e:
            print(f"  Memory test failed: {e}")
        
        # Clear large data
        large_data.clear()
        gc.collect()
        
        self.results['memory_usage'] = {
            'test_data_size_mb': 1.0,  # Approximate
            'status': 'completed'
        }
        
        print("  Memory usage test completed")
    
    def test_stress_scenarios(self):
        """Test performance under stress conditions."""
        print("\n🔥 Testing Stress Scenarios...")
        
        if not self.client:
            print("  Skipped (mock client)")
            return
        
        # Test 1: Rapid successive operations
        print("  Testing rapid successive operations...")
        rapid_times = []
        
        for i in range(50):
            start_time = time.time()
            try:
                self.client.get_stats()
                rapid_time = (time.time() - start_time) * 1000
                rapid_times.append(rapid_time)
            except Exception as e:
                print(f"    Rapid operation {i+1} failed: {e}")
        
        if rapid_times:
            avg_time = statistics.mean(rapid_times)
            self.results['stress_rapid_operations'] = {
                'avg_time_ms': avg_time,
                'samples': len(rapid_times)
            }
            print(f"    Average time: {avg_time:.2f}ms")
        
        # Test 2: Large data operations
        print("  Testing large data operations...")
        large_data = {"data": "x" * 100000}  # 100KB
        
        large_op_times = []
        for i in range(10):
            start_time = time.time()
            try:
                self.client.insert_data("stress_test_table", f"large_key_{i}", large_data)
                large_op_time = (time.time() - start_time) * 1000
                large_op_times.append(large_op_time)
            except Exception as e:
                print(f"    Large data operation {i+1} failed: {e}")
        
        if large_op_times:
            avg_time = statistics.mean(large_op_times)
            self.results['stress_large_data'] = {
                'avg_time_ms': avg_time,
                'data_size_kb': 100,
                'samples': len(large_op_times)
            }
            print(f"    Average time: {avg_time:.2f}ms")
    
    def _generate_test_data(self, size_bytes: int) -> Dict[str, Any]:
        """Generate test data of specified size."""
        # Generate data that's approximately the specified size
        target_size = size_bytes - 50  # Leave room for JSON structure
        
        if target_size <= 0:
            return {"id": 1, "data": "small"}
        
        # Generate a string of the target size
        data_string = "x" * target_size
        
        return {
            "id": 1,
            "data": data_string,
            "timestamp": "2024-01-01T00:00:00Z"
        }
    
    def save_results(self, filename: str = "performance_results.json"):
        """Save test results to a JSON file."""
        with open(filename, 'w') as f:
            json.dump(self.results, f, indent=2)
        print(f"\n💾 Results saved to {filename}")
    
    def print_summary(self):
        """Print a summary of all test results."""
        print("\n📊 Performance Test Summary")
        print("=" * 50)
        
        if not self.results:
            print("No test results available.")
            return
        
        # Connection performance
        if 'connection' in self.results:
            conn = self.results['connection']
            print(f"🔌 Connection: {conn['avg_time_ms']:.2f}ms avg")
        
        # Table operations
        if 'table_creation' in self.results:
            table = self.results['table_creation']
            print(f"📊 Table Creation: {table['avg_time_ms']:.2f}ms avg")
        
        # Data insertion
        for key, value in self.results.items():
            if key.startswith('data_insertion_'):
                size = key.split('_')[-1].replace('b', 'B')
                print(f"📝 {size} Data Insertion: {value['avg_time_ms']:.2f}ms avg, {value['throughput_kbs']:.2f} KB/s")
        
        # Query performance
        for key, value in self.results.items():
            if key.startswith('query_'):
                print(f"🔍 Query {key.split('_')[-1]}: {value['avg_time_ms']:.2f}ms avg")
        
        # Vector search
        for key, value in self.results.items():
            if key.startswith('vector_search_'):
                limit = value['limit']
                print(f"🧠 Vector Search (limit {limit}): {value['avg_time_ms']:.2f}ms avg")
        
        # Concurrent operations
        for key, value in self.results.items():
            if key.startswith('concurrent_'):
                level = value['concurrency_level']
                throughput = value['throughput_ops_per_sec']
                print(f"⚡ Concurrent {level}: {throughput:.2f} ops/sec")
        
        # Stress tests
        if 'stress_rapid_operations' in self.results:
            stress = self.results['stress_rapid_operations']
            print(f"🔥 Rapid Operations: {stress['avg_time_ms']:.2f}ms avg")
        
        if 'stress_large_data' in self.results:
            stress = self.results['stress_large_data']
            print(f"🔥 Large Data ({stress['data_size_kb']}KB): {stress['avg_time_ms']:.2f}ms avg")


def main():
    """Main entry point for performance testing."""
    parser = argparse.ArgumentParser(description="VectraEdge Performance Testing")
    parser.add_argument("--host", default="localhost", help="VectraEdge server host")
    parser.add_argument("--port", type=int, default=8080, help="VectraEdge server port")
    parser.add_argument("--use-rust", action="store_true", help="Use Rust bindings if available")
    parser.add_argument("--output", default="performance_results.json", help="Output file for results")
    parser.add_argument("--skip-tests", nargs="+", help="Skip specific tests")
    
    args = parser.parse_args()
    
    # Create performance tester
    tester = PerformanceTester(
        host=args.host,
        port=args.port,
        use_rust=args.use_rust
    )
    
    try:
        # Run all tests
        results = tester.run_all_tests()
        
        # Print summary
        tester.print_summary()
        
        # Save results
        tester.save_results(args.output)
        
        print(f"\n🎉 Performance testing completed successfully!")
        print(f"Results saved to: {args.output}")
        
    except KeyboardInterrupt:
        print("\n⚠️  Performance testing interrupted by user")
        sys.exit(1)
    except Exception as e:
        print(f"\n❌ Performance testing failed: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
